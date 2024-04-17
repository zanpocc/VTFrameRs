
struct OSVersionInfo {
    major_version: u32,
    minor_version: u32,
    build_number: u32,
    version_name: &'static str,
}

const VERSION_MAP: [OSVersionInfo;18] = [
    OSVersionInfo {
        major_version: 7,
        minor_version: 0,
        build_number: 7601,
        version_name: "Windows 7",
    },
    OSVersionInfo {
        major_version: 10,
        minor_version: 0,
        build_number: 10240,
        version_name: "Windows 10 Version 1507",
    },
    OSVersionInfo {
        major_version: 10,
        minor_version: 0,
        build_number: 14393,
        version_name: "Windows 10 Version 1607",
    },
    OSVersionInfo {
        major_version: 10,
        minor_version: 0,
        build_number: 15063,
        version_name: "Windows 10 Version 1703",
    },
    OSVersionInfo {
        major_version: 10,
        minor_version: 0,
        build_number: 16299,
        version_name: "Windows 10 Version 1709",
    },
    OSVersionInfo {
        major_version: 10,
        minor_version: 0,
        build_number: 17134,
        version_name: "Windows 10 Version 1803",
    },
    OSVersionInfo {
        major_version: 10,
        minor_version: 0,
        build_number: 17763,
        version_name: "Windows 10 Version 1809",
    },
    OSVersionInfo {
        major_version: 10,
        minor_version: 0,
        build_number: 18362,
        version_name: "Windows 10 Version 1903",
    },
    OSVersionInfo {
        major_version: 10,
        minor_version: 0,
        build_number: 18363,
        version_name: "Windows 10 Version 1909",
    },
    // Hook GetCpuClock will KERNEL_SECURITY_CHECK_FAILURE BugCheck
    OSVersionInfo {
        major_version: 10,
        minor_version: 0,
        build_number: 18950,
        version_name: "Windows 10 Version 20H1",
    },
    OSVersionInfo {
        major_version: 10,
        minor_version: 0,
        build_number: 19041,
        version_name: "Windows 10 Version 2004",
    },
    OSVersionInfo {
        major_version: 10,
        minor_version: 0,
        build_number: 19042,
        version_name: "Windows 10 Version 20H2",
    },
    OSVersionInfo {
        major_version: 10,
        minor_version: 0,
        build_number: 19043,
        version_name: "Windows 10 Version 21H1",
    },
    OSVersionInfo {
        major_version: 10,
        minor_version: 0,
        build_number: 19044,
        version_name: "Windows 10 Version 21H2",
    },
    OSVersionInfo {
        major_version: 10,
        minor_version: 0,
        build_number: 19045,
        version_name: "Windows 10 Version 22H2",
    },
    // windows 11
    OSVersionInfo {
        major_version: 11,
        minor_version: 0,
        build_number: 22000,
        version_name: "Windows 11 Version 21H2",
    },
    // kdmapper supported latest version
    OSVersionInfo {
        major_version: 11,
        minor_version: 0,
        build_number: 22449,
        version_name: "Windows 11 Insider Preview Build 22449",
    },
    OSVersionInfo {
        major_version: 11,
        minor_version: 0,
        build_number: 22621,
        version_name: "Windows 11 Version 22H2",
    },
    OSVersionInfo {
        major_version: 11,
        minor_version: 0,
        build_number: 22631,
        version_name: "Windows 11 Version 23H2",
    },
];
