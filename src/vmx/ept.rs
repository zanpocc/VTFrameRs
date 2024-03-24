pub mod ept {
    use core::{ffi::c_void, mem::size_of};

    use moon_driver_utils::bitfield::{get_bits_value, set_bits_value};
    use moon_instructions::{bit_scan_forward64, read_msr, stosq};
    use moon_log::{error, info};
    use moon_struct::msr::{ia32_mtrr_capabilities_msr, ia32_mtrr_phys_base_msr, ia32_mtrr_phys_mask_msr, msr_index::{MSR_IA32_MTRR_CAPABILITIES, MSR_IA32_MTRR_PHYSBASE0, MSR_IA32_MTRR_PHYSMASK0}};
    use wdk_sys::{ntddk::{memset, MmAllocateContiguousMemory, MmFreeContiguousMemory}, LIST_ENTRY, PAGE_SIZE, PHYSICAL_ADDRESS};

    use crate::{inner::initialize_list_head, utils::utils::virtual_address_to_physical_address, vmx::data::{pml3e, pml2e_2mb, ept_memory_type::{MEMORY_TYPE_UNCACHEABLE, MEMORY_TYPE_WRITE_BACK}, ept_pointer, pml4e}};

    
    // sizeof=512*8 + 512*8 + 512*8*512 + 0x1000 = 2109440
    // actual=2109440
    #[repr(C)]
    #[repr(align(0x1000))]
    pub struct VmmEptPageTable {
        pml4: [u64;512],
        pml3: [u64;512],
        pml2: [[u64;512];512],
        dynamic_split_list: LIST_ENTRY,
    }

    #[derive(Default)]
    pub struct MtrrRangeDescriptor {
        physical_base_address: u64,
        physical_end_address: u64,
        memory_type: u8,
    }

    #[derive(Default)]
    pub struct EptState {
        hooked_pages_list: LIST_ENTRY, 
        memory_ranges: [MtrrRangeDescriptor;9],
        number_of_enabled_memory_ranges: u32,
        ept_pointer: u64,
        ept_page_table: Option<*mut VmmEptPageTable>,
    }

    impl EptState {
        pub fn get_ept_pointer(&mut self) -> u64 {
            return self.ept_pointer;
        }

        fn ept_build_mtrr_map(&mut self) {
            let mtrr_cap = read_msr(MSR_IA32_MTRR_CAPABILITIES);

            let variable_range_count = get_bits_value(mtrr_cap, ia32_mtrr_capabilities_msr::VARIABLE_RANGE_COUNT_START, 
                ia32_mtrr_capabilities_msr::VARIABLE_RANGE_COUNT_LEN);

            for i in 0..variable_range_count {
                let phys_base = read_msr(MSR_IA32_MTRR_PHYSBASE0 + (i*2) as u32);
                let phys_mask = read_msr(MSR_IA32_MTRR_PHYSMASK0 + (i*2) as u32);

                if (phys_mask & ia32_mtrr_phys_mask_msr::VALID) != 0 {
                    let descriptor = &mut self.memory_ranges[self.number_of_enabled_memory_ranges as usize];
                    self.number_of_enabled_memory_ranges += 1;

                    descriptor.physical_base_address = get_bits_value(phys_base, ia32_mtrr_phys_base_msr::PAGE_FRAME_NUMBER_START, 
                        ia32_mtrr_capabilities_msr::PAGE_FRAME_NUMBER_LEN) * PAGE_SIZE as u64;

                    let mut number_of_bitmask = 0u32;

                    bit_scan_forward64(&mut number_of_bitmask , get_bits_value(phys_mask, ia32_mtrr_phys_mask_msr::PAGE_FRAME_NUMBER_START, 
                        ia32_mtrr_phys_mask_msr::PAGE_FRAME_NUMBER_LEN) * PAGE_SIZE as u64);

                    descriptor.physical_end_address = descriptor.physical_base_address + (1u64 << number_of_bitmask - 1);

                    descriptor.memory_type = get_bits_value(phys_base, ia32_mtrr_phys_base_msr::TYPE_START, ia32_mtrr_phys_base_msr::TYPE_LEN) as u8;
                    
                    if descriptor.memory_type == MEMORY_TYPE_WRITE_BACK {
                        self.number_of_enabled_memory_ranges -= 1;
                    }

                }
            }
        }

        fn ept_setup_pml2_entry(&mut self, new_entry: &mut u64, pfn: u64) {
            *new_entry = set_bits_value(*new_entry, pml2e_2mb::PAGE_FRAME_NUMBER_START, pml2e_2mb::PAGE_FRAME_NUMBER_LEN, pfn);

            let address_of_page = pfn * (512 * PAGE_SIZE as u64);

            if pfn == 0 {
                *new_entry = set_bits_value(*new_entry, pml2e_2mb::MEMORY_TYPE_START, pml2e_2mb::MEMORY_TYPE_LEN, MEMORY_TYPE_UNCACHEABLE as u64);
                return
            }

            let mut target_memory_type = MEMORY_TYPE_WRITE_BACK;

            for i in 0..self.number_of_enabled_memory_ranges {
                if address_of_page <= self.memory_ranges[i as usize].physical_end_address {
                    if (address_of_page + 512 * PAGE_SIZE as u64 - 1) >= self.memory_ranges[i as usize].physical_base_address {
                        // address in range
                        target_memory_type = self.memory_ranges[i as usize].memory_type as _;
                        if target_memory_type == MEMORY_TYPE_UNCACHEABLE {
                            break;
                        }
                    }
                }
            }

            *new_entry = set_bits_value(*new_entry, pml2e_2mb::MEMORY_TYPE_START, pml2e_2mb::MEMORY_TYPE_LEN, target_memory_type as u64);

        }

        fn ept_logical_processor_initialize(&mut self) {
            let mut max_size:PHYSICAL_ADDRESS = PHYSICAL_ADDRESS::default();
            max_size.QuadPart = i64::MAX;

            // allocate all page tabele memory
            let page_table:*mut VmmEptPageTable = unsafe{ 
                MmAllocateContiguousMemory((core::mem::size_of::<VmmEptPageTable>() as u64 / PAGE_SIZE as u64) * PAGE_SIZE as u64,max_size) 
            } as _;

            if page_table.is_null() {
                error!("error to allocate page_table memory");
                panic!();
            }

            // zero memory
            unsafe { memset(page_table as *mut c_void,0, size_of::<VmmEptPageTable>() as _) };

            self.ept_page_table = Some(page_table);

            initialize_list_head(unsafe { &mut (*page_table).dynamic_split_list });

            let page_table =  unsafe { &mut (*page_table) };
            let pml4 = &mut page_table.pml4;
            let pml3 = &mut page_table.pml3;
            let pml2 = &mut  page_table.pml2;

            // fill pml4e
            pml4[0] |= pml4e::READ_ACCESS;
            pml4[0] |= pml4e::WRITE_ACCESS;
            pml4[0] |= pml4e::EXECUTE_ACCESS;
            let pfn = virtual_address_to_physical_address(((&mut pml3[0]) as *mut u64) as *mut c_void) / PAGE_SIZE as u64;
            pml4[0] = set_bits_value(pml4[0], pml4e::PAGE_FRAME_NUMBER_START, pml4e::PAGE_FRAME_NUMBER_LEN, pfn);

            // fill pml3e
            let mut rwx_template:u64 = 0;
            rwx_template |= pml3e::READ_ACCESS;
            rwx_template |= pml3e::WRITE_ACCESS;
            rwx_template |= pml3e::EXECUTE_ACCESS;

            stosq(&mut pml3[0], rwx_template, pml3.len() as u64);

            for i in 0..pml3.len() {
                let pfn = virtual_address_to_physical_address(((&mut pml2[i][0]) as *mut u64) as *mut c_void) / PAGE_SIZE as u64;
                pml3[i] = set_bits_value(pml3[i], pml3e::PAGE_FRAME_NUMBER_START, pml3e::PAGE_FRAME_NUMBER_LEN, pfn);
            }

            // fill pml2e
            let mut pml2e_template:u64 = 0;
            pml2e_template |= pml2e_2mb::READ_ACCESS;
            pml2e_template |= pml2e_2mb::WRITE_ACCESS;
            pml2e_template |= pml2e_2mb::EXECUTE_ACCESS;
            pml2e_template |= pml2e_2mb::LARGET_PAGE;
            
            stosq((&mut pml2[0]) as *mut u64, pml2e_template, (pml2.len() * pml2[0].len()) as _);

            for i in 0..pml2.len() {
                for j in 0..pml2[0].len() {
                    let pfn = i * pml2.len() + j;
                    self.ept_setup_pml2_entry(&mut pml2[i][j],pfn as _);
                }
            }

        } 

        pub fn new() -> Self {
            let mut ept_state = EptState::default();
            ept_state.ept_build_mtrr_map();
            ept_state.ept_logical_processor_initialize();
            initialize_list_head(&mut ept_state.hooked_pages_list);

            // ept_pointer
            ept_state.ept_pointer = 0;
            ept_state.ept_pointer = set_bits_value(ept_state.ept_pointer, 
                ept_pointer::MEMORY_TYPE_START, ept_pointer::MEMORY_TYPE_LEN, MEMORY_TYPE_WRITE_BACK as _);
            ept_state.ept_pointer = set_bits_value(ept_state.ept_pointer, 
                ept_pointer::PAGE_WALK_LENGTH_START, ept_pointer::PAGE_WALK_LENGTH_LEN, 3);

            let phys_addr = virtual_address_to_physical_address(ept_state.ept_page_table.unwrap() as _);
            ept_state.ept_pointer = set_bits_value(ept_state.ept_pointer, ept_pointer::PHYS_ADDR_START, ept_pointer::PHYS_ADDR_LEN,
                phys_addr >> 12
            );

            return ept_state;
        }
    }

    impl Drop for EptState{
        fn drop(&mut self) {
            info!("EptState Drop");
            if let Some(ept_table) = self.ept_page_table {
                if !ept_table.is_null() {
                    info!("free ept data");
                    unsafe { MmFreeContiguousMemory(ept_table as _) };
                }
            }
        }
    }
}
