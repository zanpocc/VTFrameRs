const IMAGE_NUMBEROF_DIRECTORY_ENTRIES: usize = 16;

#[repr(C)]
#[allow(dead_code)]
pub struct ImageDosHeader {
    pub e_magic: u16,      // Magic number
    pub e_cblp: u16,       // Bytes on last page of file
    pub e_cp: u16,         // Pages in file
    pub e_crlc: u16,       // Relocations
    pub e_cparhdr: u16,    // Size of header in paragraphs
    pub e_minalloc: u16,   // Minimum extra paragraphs needed
    pub e_maxalloc: u16,   // Maximum extra paragraphs needed
    pub e_ss: u16,         // Initial (relative) SS value
    pub e_sp: u16,         // Initial SP value
    pub e_csum: u16,       // Checksum
    pub e_ip: u16,         // Initial IP value
    pub e_cs: u16,         // Initial (relative) CS value
    pub e_lfarlc: u16,     // File address of relocation table
    pub e_ovno: u16,       // Overlay number
    pub e_res: [u16; 4],   // Reserved words
    pub e_oemid: u16,      // OEM identifier (for e_oeminfo)
    pub e_oeminfo: u16,    // OEM information; e_oemid specific
    pub e_res2: [u16; 10], // Reserved words
    pub e_lfanew: i32,     // File address of new exe header
}

#[repr(C)]
#[allow(dead_code)]
pub struct ImageDataDirectory {
    pub virtual_address: u32, // DWORD -> u32
    pub size: u32,            // DWORD -> u32
}

#[repr(C)]
#[allow(dead_code)]
pub struct ImageOptionalHeader64 {
    pub magic: u16,                          // WORD -> u16
    pub major_linker_version: u8,            // BYTE -> u8
    pub minor_linker_version: u8,            // BYTE -> u8
    pub size_of_code: u32,                   // DWORD -> u32
    pub size_of_initialized_data: u32,       // DWORD -> u32
    pub size_of_uninitialized_data: u32,     // DWORD -> u32
    pub address_of_entry_point: u32,         // DWORD -> u32
    pub base_of_code: u32,                   // DWORD -> u32
    pub image_base: u64,                     // ULONGLONG -> u64
    pub section_alignment: u32,              // DWORD -> u32
    pub file_alignment: u32,                 // DWORD -> u32
    pub major_operating_system_version: u16, // WORD -> u16
    pub minor_operating_system_version: u16, // WORD -> u16
    pub major_image_version: u16,            // WORD -> u16
    pub minor_image_version: u16,            // WORD -> u16
    pub major_subsystem_version: u16,        // WORD -> u16
    pub minor_subsystem_version: u16,        // WORD -> u16
    pub win32_version_value: u32,            // DWORD -> u32
    pub size_of_image: u32,                  // DWORD -> u32
    pub size_of_headers: u32,                // DWORD -> u32
    pub check_sum: u32,                      // DWORD -> u32
    pub subsystem: u16,                      // WORD -> u16
    pub dll_characteristics: u16,            // WORD -> u16
    pub size_of_stack_reserve: u64,          // ULONGLONG -> u64
    pub size_of_stack_commit: u64,           // ULONGLONG -> u64
    pub size_of_heap_reserve: u64,           // ULONGLONG -> u64
    pub size_of_heap_commit: u64,            // ULONGLONG -> u64
    pub loader_flags: u32,                   // DWORD -> u32
    pub number_of_rva_and_sizes: u32,        // DWORD -> u32
    pub data_directory: [ImageDataDirectory; IMAGE_NUMBEROF_DIRECTORY_ENTRIES], // array of ImageDataDirectory
}

#[repr(C)]
#[allow(dead_code)]
pub struct ImageExportDirectory {
    pub characteristics: u32,          // DWORD -> u32
    pub time_date_stamp: u32,          // DWORD -> u32
    pub major_version: u16,            // WORD -> u16
    pub minor_version: u16,            // WORD -> u16
    pub name: u32,                     // DWORD -> u32
    pub base: u32,                     // DWORD -> u32
    pub number_of_functions: u32,      // DWORD -> u32
    pub number_of_names: u32,          // DWORD -> u32
    pub address_of_functions: u32,     // DWORD -> u32
    pub address_of_names: u32,         // DWORD -> u32
    pub address_of_name_ordinals: u32, // DWORD -> u32
}

#[repr(C)]
#[allow(dead_code)]
pub struct ImageFileHeader {
    machine: u16,                 // WORD -> u16
    number_of_sections: u16,      // WORD -> u16
    time_date_stamp: u32,         // DWORD -> u32
    pointer_to_symbol_table: u32, // DWORD -> u32
    number_of_symbols: u32,       // DWORD -> u32
    size_of_optional_header: u16, // WORD -> u16
    characteristics: u16,         // WORD -> u16
}
