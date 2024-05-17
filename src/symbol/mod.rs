pub mod generic;
pub mod symbol {
    use core::ffi::c_void;

    use alloc::ffi::CString;
    use moon_driver_utils::string::{cstr_to_rust_str, str_to_unicode_string};
    use moon_instructions::read_msr;
    use moon_struct::pe::{ImageDosHeader, ImageExportDirectory, ImageFileHeader, ImageOptionalHeader64};
    use wdk_sys::{ntddk::{memcpy, strcmp, IoGetCurrentProcess, KeStackAttachProcess, KeUnstackDetachProcess, MmIsAddressValid, PsLookupProcessByProcessId, RtlCompareUnicodeString}, KAPC_STATE, LIST_ENTRY64, NT_SUCCESS, PEPROCESS, UCHAR, _KPROCESS};

    use super::generic::OS_INFO;

    extern "C" {
        fn PsGetProcessImageFileName(Process:PEPROCESS) -> *mut UCHAR;
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct SystemServiceTable {
        pub service_table_base: *mut u32,
        service_counter_table_base: *mut core::ffi::c_void,
        number_of_services: usize,
        param_table_base: *mut core::ffi::c_void,
    }

    pub fn get_ssdt_address() -> *mut SystemServiceTable {
        let start_search_address = read_msr(0xC0000082);
        let end_search_address = start_search_address + 0x500;

        let mut p = 0u64;

        let mut b1 = 0u8;
        let mut b2 = 0u8;
        let mut b3 = 0u8;

        for  i in start_search_address..end_search_address {
            unsafe {
                if MmIsAddressValid(i as _) != 0 &&
                    MmIsAddressValid((i+1) as _) != 0 &&
                    MmIsAddressValid((i+2) as _) != 0 {
                        b1 = *(i as *mut u8);
                        b2 = *((i+1) as *mut u8);
                        b3 = *((i+2) as *mut u8);
                        if b1 == 0x4c && b2 == 0x8d && b3 == 0x15 {
                            let mut temp = 0u32;
                            memcpy(&mut temp as *mut u32 as _, (i + 3) as _, 4 as _);
                            p = temp as u64 + i as u64 + 7u64;
                        }
                }
            }
        }    

        return p as _;
    }

    pub fn lookup_process(pid: u32) -> *mut _KPROCESS{
        let mut process:*mut _KPROCESS = core::ptr::null_mut();
        let status = unsafe { PsLookupProcessByProcessId(pid as _, &mut process as *mut PEPROCESS) };
        if !NT_SUCCESS(status) {
            return core::ptr::null_mut();
        }
        return process;
    }

    pub fn get_ldr_module_base_by_name(name: &str) -> *mut c_void{
        let mut name = str_to_unicode_string(name);
        let cprocess = unsafe { IoGetCurrentProcess() };

        unsafe{
            let peb = *((cprocess as u64 + OS_INFO.offset.eprocess_peb) as *mut u64);
            if peb == 0 {
                // driver
                return core::ptr::null_mut();
            }

            let ldr = *((peb + OS_INFO.offset.peb_ldr) as *mut u64);

            let list_head = (ldr + OS_INFO.offset.ldr_in_load_order_module_list) as *mut LIST_ENTRY64;

            let mut c = (*list_head).Flink as *mut LIST_ENTRY64;

            let mut count = 0;
            loop {
               if c == list_head || count >= 0x1f4 {
                   break;
               }

               if RtlCompareUnicodeString(((c as u64) + OS_INFO.offset.ldre_base_dll_name) as _,
                         &mut name as _, 0) == 0 {
                    return *((c as u64 + OS_INFO.offset.ldre_dll_base) as *mut u64) as _;
               }

               c = (*c).Flink as _;
               count = count + 1;
            }
        }

        core::ptr::null_mut()
    }

    pub fn get_process_by_name(name: &str) -> *mut _KPROCESS {
        for i in (4..=262144).step_by(4) {
            let process = lookup_process(i as _);
            if process.is_null(){
                continue;
            }

            unsafe {
                let cname: *mut u8 =  PsGetProcessImageFileName(process as _) ;
                if cstr_to_rust_str(cname) == name {
                    return process;
                }
            }
        }

        core::ptr::null_mut()
    }

    pub fn get_module_export_address(module: *mut c_void,name: &str) -> *mut c_void {
        let dos_header = module as *mut ImageDosHeader;
        if dos_header.is_null(){
            return core::ptr::null_mut();
        }
        unsafe{

        
            let option_header:*mut ImageOptionalHeader64 = (module as u64
                + (*dos_header).e_lfanew as u64 + core::mem::size_of::<u32>() as u64 
                + core::mem::size_of::<ImageFileHeader>() as u64) as _;
            if option_header.is_null() {
                return core::ptr::null_mut();
            }

            let export_table: *mut ImageExportDirectory = (module as u64 
                + (*option_header).data_directory[0].virtual_address as u64) as _;

            if export_table.is_null(){
                return core::ptr::null_mut();
            }

            let size = (*option_header).data_directory[0].size;

            let name_table_base = (module as u64
                + (*export_table).address_of_names as u64) as *mut u32;

            // 如果通过id，就可用直接取name_ordinal_table_base[id]
            let name_ordinal_table_base = (module as u64
                + (*export_table).address_of_name_ordinals as u64) as *mut u16;

            let address_table_base = (module as u64
                + (*export_table).address_of_functions as u64) as *mut u32;

            let mut low = 0u32;
            let mut middle = 0u32;
            let mut hight = (*export_table).number_of_names - 1;

            let cname = CString::new(name).unwrap();

            // 二分找导出函数
            while hight >= low {
                middle = (low + hight) >> 1;
                
                let s1 = cname.as_ptr();
                let s2 = module.add(*(name_table_base.add(middle as _)) as _);

                let result = strcmp(s1 as _, s2 as _);

                if result < 0 {
                    hight = middle - 1;
                }else if result > 0{
                    low = middle + 1;
                }else{
                    break;
                }
            }

            if hight < low{
                return core::ptr::null_mut();
            }
            
            let ordinal_number = *(name_ordinal_table_base.add(middle as _));
            if ordinal_number > (*export_table).number_of_functions as _ {
                return core::ptr::null_mut();
            }

            let function_address = module.add(*(address_table_base.add(ordinal_number as _)) as _);
            let end = export_table as u64 + size as u64;
            if function_address as u64 > export_table as u64 && (function_address as u64) < end {
                return core::ptr::null_mut();
            }

            return function_address as _;
        }
    }

    pub fn get_ntdll_function_id(name: &str) -> u32 {
        let process = get_process_by_name("smss.exe");
        if process.is_null(){
            return 0;
        }

        let mut result = u32::MAX;

        unsafe{
            let mut apc_state = KAPC_STATE::default();
            KeStackAttachProcess(process, &mut apc_state as _);

            let ntdll = get_ldr_module_base_by_name("ntdll.dll");
            if !ntdll.is_null() {
                let func = get_module_export_address(ntdll,name);
                if !func.is_null() {
                    result = *((func.add(4)) as *mut u16) as _;
                }
            }

            KeUnstackDetachProcess(&mut apc_state as _);
        }
        
        result
    }

    pub fn get_ssdt_address_by_id(id: u32) -> *mut c_void {
        let ssdt = get_ssdt_address();
        if ssdt.is_null(){
            return core::ptr::null_mut();
        }

        unsafe{
            let base = (*ssdt).service_table_base;
            if base.is_null(){
                return core::ptr::null_mut();
            }

            let mut temp = *base.add(id as _);
            temp = temp >> 4;
            return (temp as u64 + base as u64) as _;
        }
    }

    pub fn get_ssdt_function_by_name(name: &str) -> *mut c_void {
        let id = get_ntdll_function_id(name);

        if id != u32::MAX{
            return get_ssdt_address_by_id(id);            
        }

        return core::ptr::null_mut();
    }
}