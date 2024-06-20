use wdk_sys::{ntddk::RtlGetVersion, NT_SUCCESS, RTL_OSVERSIONINFOW};

extern crate alloc;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Offset {
    pub eprocess_peb: u64,
    pub peb_ldr: u64,
    pub ldr_in_load_order_module_list: u64,
    pub ldre_base_dll_name: u64,
    pub ldre_dll_base: u64,
}

impl Default for Offset {
    fn default() -> Self {
        Self {
            eprocess_peb: 0x550,
            peb_ldr: 0x18,
            ldr_in_load_order_module_list: 0x10,
            ldre_base_dll_name: 0x58, // _KLDR_DATA_TABLE_ENTRY
            ldre_dll_base: 0x30,
        }
    }
}

#[allow(unused)]
#[derive(Clone, Copy)]
pub struct OSVersionInfo {
    major_version: u32,
    minor_version: u32,
    build_number: u32,
    pub version_name: &'static str,
    pub offset: Offset,
}

const VERSION_MAP: [OSVersionInfo; 21] = [
    OSVersionInfo {
        major_version: 0,
        minor_version: 0,
        build_number: 0,
        version_name: "Unknown OS Version",
        offset: {
            let offset = Offset {
                eprocess_peb: 0x550,
                peb_ldr: 0x18,
                ldr_in_load_order_module_list: 0x10,
                ldre_base_dll_name: 0x58,
                ldre_dll_base: 0x30,
            };
            offset
        },
    },
    OSVersionInfo {
        major_version: 7,
        minor_version: 0,
        build_number: 7601,
        version_name: "Windows 7",
        offset: {
            let offset = Offset {
                eprocess_peb: 0x550,
                peb_ldr: 0x18,
                ldr_in_load_order_module_list: 0x10,
                ldre_base_dll_name: 0x58,
                ldre_dll_base: 0x30,
            };
            offset
        },
    },
    OSVersionInfo {
        major_version: 10,
        minor_version: 0,
        build_number: 10240,
        version_name: "Windows 10 Version 1507",
        offset: {
            let offset = Offset {
                eprocess_peb: 0x550,
                peb_ldr: 0x18,
                ldr_in_load_order_module_list: 0x10,
                ldre_base_dll_name: 0x58,
                ldre_dll_base: 0x30,
            };
            offset
        },
    },
    OSVersionInfo {
        major_version: 10,
        minor_version: 0,
        build_number: 14393,
        version_name: "Windows 10 Version 1607",
        offset: {
            let offset = Offset {
                eprocess_peb: 0x550,
                peb_ldr: 0x18,
                ldr_in_load_order_module_list: 0x10,
                ldre_base_dll_name: 0x58,
                ldre_dll_base: 0x30,
            };
            offset
        },
    },
    OSVersionInfo {
        major_version: 10,
        minor_version: 0,
        build_number: 15063,
        version_name: "Windows 10 Version 1703",
        offset: {
            let offset = Offset {
                eprocess_peb: 0x550,
                peb_ldr: 0x18,
                ldr_in_load_order_module_list: 0x10,
                ldre_base_dll_name: 0x58,
                ldre_dll_base: 0x30,
            };
            offset
        },
    },
    OSVersionInfo {
        major_version: 10,
        minor_version: 0,
        build_number: 16299,
        version_name: "Windows 10 Version 1709",
        offset: {
            let offset = Offset {
                eprocess_peb: 0x550,
                peb_ldr: 0x18,
                ldr_in_load_order_module_list: 0x10,
                ldre_base_dll_name: 0x58,
                ldre_dll_base: 0x30,
            };
            offset
        },
    },
    OSVersionInfo {
        major_version: 10,
        minor_version: 0,
        build_number: 17134,
        version_name: "Windows 10 Version 1803",
        offset: {
            let offset = Offset {
                eprocess_peb: 0x550,

                peb_ldr: 0x18,
                ldr_in_load_order_module_list: 0x10,
                ldre_base_dll_name: 0x58,
                ldre_dll_base: 0x30,
            };
            offset
        },
    },
    OSVersionInfo {
        major_version: 10,
        minor_version: 0,
        build_number: 17763,
        version_name: "Windows 10 Version 1809",
        offset: {
            let offset = Offset {
                eprocess_peb: 0x3f8,

                peb_ldr: 0x18,
                ldr_in_load_order_module_list: 0x10,
                ldre_base_dll_name: 0x58,
                ldre_dll_base: 0x30,
            };
            offset
        },
    },
    OSVersionInfo {
        major_version: 10,
        minor_version: 0,
        build_number: 18362,
        version_name: "Windows 10 Version 1903",
        offset: {
            let offset = Offset {
                eprocess_peb: 0x550,
                peb_ldr: 0x18,
                ldr_in_load_order_module_list: 0x10,
                ldre_base_dll_name: 0x58,
                ldre_dll_base: 0x30,
            };
            offset
        },
    },
    OSVersionInfo {
        major_version: 10,
        minor_version: 0,
        build_number: 18363,
        version_name: "Windows 10 Version 1909",
        offset: {
            let offset = Offset {
                eprocess_peb: 0x550,
                peb_ldr: 0x18,
                ldr_in_load_order_module_list: 0x10,
                ldre_base_dll_name: 0x58,
                ldre_dll_base: 0x30,
            };
            offset
        },
    },
    // Hook GetCpuClock will KERNEL_SECURITY_CHECK_FAILURE BugCheck
    OSVersionInfo {
        major_version: 10,
        minor_version: 0,
        build_number: 18950,
        version_name: "Windows 10 Version 20H1",
        offset: {
            let offset = Offset {
                eprocess_peb: 0x550,
                peb_ldr: 0x18,
                ldr_in_load_order_module_list: 0x10,
                ldre_base_dll_name: 0x58,
                ldre_dll_base: 0x30,
            };
            offset
        },
    },
    // ExAllocatePool2 support lowest version
    OSVersionInfo {
        major_version: 10,
        minor_version: 0,
        build_number: 19041,
        version_name: "Windows 10 Version 2004",
        offset: {
            let offset = Offset {
                eprocess_peb: 0x550,
                peb_ldr: 0x18,
                ldr_in_load_order_module_list: 0x10,
                ldre_base_dll_name: 0x58,
                ldre_dll_base: 0x30,
            };
            offset
        },
    },
    OSVersionInfo {
        major_version: 10,
        minor_version: 0,
        build_number: 19042,
        version_name: "Windows 10 Version 20H2",
        offset: {
            let offset = Offset {
                eprocess_peb: 0x550,
                peb_ldr: 0x18,
                ldr_in_load_order_module_list: 0x10,
                ldre_base_dll_name: 0x58,
                ldre_dll_base: 0x30,
            };
            offset
        },
    },
    OSVersionInfo {
        major_version: 10,
        minor_version: 0,
        build_number: 19043,
        version_name: "Windows 10 Version 21H1",
        offset: {
            let offset = Offset {
                eprocess_peb: 0x550,
                peb_ldr: 0x18,
                ldr_in_load_order_module_list: 0x10,
                ldre_base_dll_name: 0x58,
                ldre_dll_base: 0x30,
            };
            offset
        },
    },
    OSVersionInfo {
        major_version: 10,
        minor_version: 0,
        build_number: 19044,
        version_name: "Windows 10 Version 21H2",
        offset: {
            let offset = Offset {
                eprocess_peb: 0x550,
                peb_ldr: 0x18,
                ldr_in_load_order_module_list: 0x10,
                ldre_base_dll_name: 0x58,
                ldre_dll_base: 0x30,
            };
            offset
        },
    },
    OSVersionInfo {
        major_version: 10,
        minor_version: 0,
        build_number: 19045,
        version_name: "Windows 10 Version 22H2",
        offset: {
            let offset = Offset {
                eprocess_peb: 0x550,
                peb_ldr: 0x18,
                ldr_in_load_order_module_list: 0x10,
                ldre_base_dll_name: 0x58,
                ldre_dll_base: 0x30,
            };
            offset
        },
    },
    // windows 11
    OSVersionInfo {
        major_version: 10,
        minor_version: 0,
        build_number: 22000,
        version_name: "Windows 10 Version 21H2",
        offset: {
            let offset = Offset {
                eprocess_peb: 0x550,
                peb_ldr: 0x18,
                ldr_in_load_order_module_list: 0x10,
                ldre_base_dll_name: 0x58,
                ldre_dll_base: 0x30,
            };
            offset
        },
    },
    OSVersionInfo {
        major_version: 11,
        minor_version: 0,
        build_number: 22000,
        version_name: "Windows 11 Version 21H2",
        offset: {
            let offset = Offset {
                eprocess_peb: 0x550,
                peb_ldr: 0x18,
                ldr_in_load_order_module_list: 0x10,
                ldre_base_dll_name: 0x58,
                ldre_dll_base: 0x30,
            };
            offset
        },
    },
    // kdmapper supported latest version
    OSVersionInfo {
        major_version: 11,
        minor_version: 0,
        build_number: 22449,
        version_name: "Windows 11 Insider Preview Build 22449",
        offset: {
            let offset = Offset {
                eprocess_peb: 0x550,
                peb_ldr: 0x18,
                ldr_in_load_order_module_list: 0x10,
                ldre_base_dll_name: 0x58,
                ldre_dll_base: 0x30,
            };
            offset
        },
    },
    OSVersionInfo {
        major_version: 11,
        minor_version: 0,
        build_number: 22621,
        version_name: "Windows 11 Version 22H2",
        offset: {
            let offset = Offset {
                eprocess_peb: 0x550,
                peb_ldr: 0x18,
                ldr_in_load_order_module_list: 0x10,
                ldre_base_dll_name: 0x58,
                ldre_dll_base: 0x30,
            };
            offset
        },
    },
    OSVersionInfo {
        major_version: 11,
        minor_version: 0,
        build_number: 22631,
        version_name: "Windows 11 Version 23H2",
        offset: {
            let offset = Offset {
                eprocess_peb: 0x550,
                peb_ldr: 0x18,
                ldr_in_load_order_module_list: 0x10,
                ldre_base_dll_name: 0x58,
                ldre_dll_base: 0x30,
            };
            offset
        },
    },
];

// 全局唯一的系统信息,首次访问时初始化
lazy_static! {
    pub static ref OS_INFO: OSVersionInfo = {
        let mut os_version = RTL_OSVERSIONINFOW {
            dwOSVersionInfoSize: 0,
            dwMajorVersion: 0,
            dwMinorVersion: 0,
            dwBuildNumber: 0,
            dwPlatformId: 0,
            szCSDVersion: [0; 128],
        };

        let os_version_ptr: *mut RTL_OSVERSIONINFOW = &mut os_version;

        let status = unsafe { RtlGetVersion(os_version_ptr) };

        if !NT_SUCCESS(status) {
            return VERSION_MAP[0].clone();
        }

        for os_info in VERSION_MAP {
            if os_info.major_version == os_version.dwMajorVersion
                && os_info.build_number == os_version.dwBuildNumber
            {
                return os_info;
            }
        }

        VERSION_MAP[0].clone()
    };
}
