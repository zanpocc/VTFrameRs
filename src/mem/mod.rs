pub mod global_alloc;

pub mod mem {
    use cty::c_void;
    use moon_instructions::read_cr3;
    use wdk_sys::{ntddk::MmGetVirtualForPhysical, PHYSICAL_ADDRESS};

    #[derive(Default)]
    pub struct PageTableTansform {
        pte_base: u64,
        pxe_selfmapping_index: u64,
        pde_base: u64,
        ppe_base: u64,
        pxe_base: u64,
        pxe_end: u64,
        pte_end: u64,
    }

    impl PageTableTansform {
        pub fn new(random_page_table: bool) -> Result<Self, ()> {
            let mut result = PageTableTansform::default();

            let mut pml4t: PHYSICAL_ADDRESS = PHYSICAL_ADDRESS::default();
            let pml4t_va: *mut c_void;

            if random_page_table {
                pml4t.QuadPart = read_cr3() as i64 & 0xFFFFFFFFF000;

                pml4t_va = unsafe { MmGetVirtualForPhysical(pml4t) };
                if !pml4t_va.is_null() {
                    let mut slot = 0;
                    let mut index = 0;

                    while unsafe {
                        (*(((pml4t_va as u64) + index * 8) as *mut u64) & 0xFFFFFFFFF000)
                            != pml4t.QuadPart as _
                    } {
                        slot += 1;
                        index += 1;
                        if index >= 512 {
                            ()
                        }
                    }

                    let v6: u64 = (slot + 0x1FFFE00) << 39;
                    result.pte_base = (slot + 0x1FFFE00) << 39;
                    result.pxe_selfmapping_index = slot;
                    result.pde_base = v6 + (slot << 30);
                    result.ppe_base = v6 + (slot << 30) + (slot << 21);
                    result.pxe_base = result.ppe_base + (slot << 12);
                    result.pxe_end = result.pxe_base + 4096;
                    result.pte_end = v6 + 0x8000000000;
                }
            } else {
                result.pxe_selfmapping_index = 493;
                result.pte_base = 0xFFFFF68000000000;
                result.pde_base = 0xFFFFF6FB40000000;
                result.ppe_base = 0xFFFFF6FB7DA00000;
                result.pxe_base = 0xFFFFF6FB7DBED000;
                result.pxe_end = 0xFFFFF6FB7DBEE000;
                result.pte_end = 0xFFFFF70000000000;
            }

            Ok(result)
        }

        pub fn get_pte_address(self, addr: u64) -> u64 {
            return (((addr & 0xffffffffffff) >> 12) << 3) + self.pte_base;
        }

        pub fn get_pde_address(self, addr: u64) -> u64 {
            return (((addr & 0xffffffffffff) >> 21) << 3) + self.pde_base;
        }

        pub fn get_ppe_address(self, addr: u64) -> u64 {
            return (((addr & 0xffffffffffff) >> 21) << 3) + self.ppe_base;
        }

        pub fn get_pxe_address(self, addr: u64) -> u64 {
            return (((addr & 0xffffffffffff) >> 21) << 3) + self.pxe_base;
        }
    }
}
