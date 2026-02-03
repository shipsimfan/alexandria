use win32::{GetProductInfo, ntddk::RtlGetVersion, wdm::RTL_OSVERSIONINFOEXW};

/// Get the full name of the operating system
pub fn os_name() -> String {
    let mut version_information = RTL_OSVERSIONINFOEXW::default();
    unsafe { RtlGetVersion(&mut version_information as *mut _ as _) };

    let name = get_windows_name(&version_information);
    let service_pack = get_windows_service_pack(&version_information);
    let product_info = get_windows_product_version(&version_information);

    format!(
        "{}{}{}{}{}",
        name,
        if product_info.len() == 0 { "" } else { " " },
        product_info,
        if service_pack.len() == 0 { "" } else { " " },
        service_pack,
    )
}

/// Get the name of this Windows version
fn get_windows_name(info: &RTL_OSVERSIONINFOEXW) -> &'static str {
    if info.major_version == 5 {
        if info.minor_version == 0 {
            return "Windows 2000";
        }

        if info.minor_version == 1 || info.minor_version == 2 {
            return "Windows XP";
        }
    }

    if info.major_version == 6 {
        if info.minor_version == 0 {
            return "Windows Vista";
        }

        if info.minor_version == 1 {
            return "Windows 7";
        }

        if info.minor_version == 2 {
            return "Windows 8";
        }

        if info.minor_version == 3 {
            return "Windows 8.1";
        }
    }

    if info.major_version == 10 {
        if info.build_number < 22000 {
            return "Windows 10";
        } else {
            return "Windows 11";
        }
    }

    "unknown Windows"
}

/// Get the service pack of this windows version
fn get_windows_service_pack(info: &RTL_OSVERSIONINFOEXW) -> String {
    let mut service_pack_length = 0;
    for byte in info.csd_version {
        if byte == 0 {
            break;
        }

        service_pack_length += 1;
    }

    String::from_utf16_lossy(&info.csd_version[..service_pack_length])
}

/// Get the Windows product version
fn get_windows_product_version(info: &RTL_OSVERSIONINFOEXW) -> &'static str {
    let mut product_info = 0;
    if unsafe {
        GetProductInfo(
            info.major_version,
            info.minor_version,
            info.service_pack_major as _,
            info.service_pack_minor as _,
            &mut product_info,
        )
    } == 0
    {
        panic!("TESTING");
    };

    match product_info {
        win32::PRODUCT_BUSINESS => "Business",
        win32::PRODUCT_BUSINESS_N => "Business N",
        win32::PRODUCT_CLUSTER_SERVER => "HPC Edition",
        win32::PRODUCT_CLUSTER_SERVER_V => "Server Hyper Core V",
        win32::PRODUCT_CORE => "Home",
        win32::PRODUCT_CORE_COUNTRYSPECIFIC => "Home China",
        win32::PRODUCT_CORE_N => "Home N",
        win32::PRODUCT_CORE_SINGLELANGUAGE => "Home Single Language",
        win32::PRODUCT_DATACENTER_EVALUATION_SERVER => "Server Datacenter",
        win32::PRODUCT_DATACENTER_A_SERVER_CORE => "Server Datacenter, Semi-Annual Channel",
        win32::PRODUCT_STANDARD_A_SERVER_CORE => "Server Standard, Semi-Annual Channel",
        win32::PRODUCT_DATACENTER_SERVER => "Server Datacenter",
        win32::PRODUCT_DATACENTER_SERVER_CORE => "Server Datacenter",
        win32::PRODUCT_DATACENTER_SERVER_CORE_V => "Server Datacenter without Hyper-V",
        win32::PRODUCT_DATACENTER_SERVER_V => "Server Datacenter without Hyper-V",
        win32::PRODUCT_EDUCATION => "Education",
        win32::PRODUCT_EDUCATION_N => "Education N",
        win32::PRODUCT_ENTERPRISE => "Enterprise",
        win32::PRODUCT_ENTERPRISE_E => "Enterprise E",
        win32::PRODUCT_ENTERPRISE_EVALUATION => "Enterprise Evaluation",
        win32::PRODUCT_ENTERPRISE_N => "Enterprise N",
        win32::PRODUCT_ENTERPRISE_N_EVALUATION => "Enterprise N Evaluation",
        win32::PRODUCT_ENTERPRISE_S => "Enterprise 2015 LTSB",
        win32::PRODUCT_ENTERPRISE_S_EVALUATION => "Enterprise 2015 LTSB Evaluation",
        win32::PRODUCT_ENTERPRISE_S_N => "Enterprise 2015 LTSB N",
        win32::PRODUCT_ENTERPRISE_S_N_EVALUATION => "Enterprise 2015 LTSB N Evaluation",
        win32::PRODUCT_ENTERPRISE_SERVER => "Server Enterprise",
        win32::PRODUCT_ENTERPRISE_SERVER_CORE => "Server Enterprise",
        win32::PRODUCT_ENTERPRISE_SERVER_CORE_V => "Server Enterprise without Hyper-V",
        win32::PRODUCT_ENTERPRISE_SERVER_IA64 => "Server Enterprise for Itanium-based Systems",
        win32::PRODUCT_ENTERPRISE_SERVER_V => "Server Enterprise without Hyper-V",
        win32::PRODUCT_ESSENTIALBUSINESS_SERVER_ADDL => "Essential Server Solution Additional",
        win32::PRODUCT_ESSENTIALBUSINESS_SERVER_ADDLSVC => {
            "Essential Server Solution Additional SVC"
        }
        win32::PRODUCT_ESSENTIALBUSINESS_SERVER_MGMT => "Essential Server Solution Management",
        win32::PRODUCT_ESSENTIALBUSINESS_SERVER_MGMTSVC => {
            "Essential Server Solution Management SVC"
        }
        win32::PRODUCT_HOME_BASIC => "Home Basic",
        win32::PRODUCT_HOME_BASIC_E => "Not supported",
        win32::PRODUCT_HOME_BASIC_N => "Home Basic N",
        win32::PRODUCT_HOME_PREMIUM => "Home Premium",
        win32::PRODUCT_HOME_PREMIUM_E => "Not supported",
        win32::PRODUCT_HOME_PREMIUM_N => "Home Premium N",
        win32::PRODUCT_HOME_PREMIUM_SERVER => "Home Server 2011",
        win32::PRODUCT_HOME_SERVER => "Storage Server 2008 R2 Essentials",
        win32::PRODUCT_HYPERV => "Microsoft Hyper-V Server",
        win32::PRODUCT_IOTENTERPRISE => "IoT Enterprise",
        win32::PRODUCT_IOTENTERPRISE_S => "IoT Enterprise LTSC",
        win32::PRODUCT_IOTUAP => "IoT Core",
        win32::PRODUCT_IOTUAPCOMMERCIAL => "IoT Core Commercial",
        win32::PRODUCT_MEDIUMBUSINESS_SERVER_MANAGEMENT => {
            "Essential Business Server Management Server"
        }
        win32::PRODUCT_MEDIUMBUSINESS_SERVER_MESSAGING => {
            "Essential Business Server Messaging Server"
        }
        win32::PRODUCT_MEDIUMBUSINESS_SERVER_SECURITY => {
            "Essential Business Server Security Server"
        }
        win32::PRODUCT_MOBILE_CORE => "Mobile",
        win32::PRODUCT_MOBILE_ENTERPRISE => "Mobile Enterprise",
        win32::PRODUCT_MULTIPOINT_PREMIUM_SERVER => "MultiPoint Server Premium",
        win32::PRODUCT_MULTIPOINT_STANDARD_SERVER => "MultiPoint Server Standard ",
        win32::PRODUCT_PPI_PRO => "Team",
        win32::PRODUCT_PRO_FOR_EDUCATION => "Pro Education",
        win32::PRODUCT_PRO_WORKSTATION => "Pro for Workstations",
        win32::PRODUCT_PRO_WORKSTATION_N => "Pro for Workstations N",
        win32::PRODUCT_PROFESSIONAL => "Pro",
        win32::PRODUCT_PROFESSIONAL_E => "Not supported",
        win32::PRODUCT_PROFESSIONAL_N => "Pro N",
        win32::PRODUCT_PROFESSIONAL_WMC => "Professional with Media Center",
        win32::PRODUCT_SB_SOLUTION_SERVER => "Small Business Server 2011 Essentials",
        win32::PRODUCT_SB_SOLUTION_SERVER_EM => "Server For SB Solutions EM",
        win32::PRODUCT_SERVER_FOR_SB_SOLUTIONS => "Server For SB Solutions",
        win32::PRODUCT_SERVER_FOR_SB_SOLUTIONS_EM => "Server For SB Solutions EM",
        win32::PRODUCT_SERVER_FOR_SMALLBUSINESS => {
            "Server 2008 for Windows Essential Server Solutions"
        }
        win32::PRODUCT_SERVER_FOR_SMALLBUSINESS_V => {
            "Server 2008 without Hyper-V for Windows Essential Server Solutions"
        }
        win32::PRODUCT_SERVER_FOUNDATION => "Server Foundation",
        win32::PRODUCT_SERVERRDSH => "Enterprise for Virtual Desktops",
        win32::PRODUCT_SMALLBUSINESS_SERVER => "Small Business Server",
        win32::PRODUCT_SMALLBUSINESS_SERVER_PREMIUM => "Small Business Server Premium",
        win32::PRODUCT_SMALLBUSINESS_SERVER_PREMIUM_CORE => "Small Business Server Premium",
        win32::PRODUCT_SOLUTION_EMBEDDEDSERVER => "MultiPoint Server",
        win32::PRODUCT_STANDARD_EVALUATION_SERVER => "Server Standard",
        win32::PRODUCT_STANDARD_SERVER => "Server Standard",
        win32::PRODUCT_STANDARD_SERVER_CORE => "Server Standard",
        win32::PRODUCT_STANDARD_SERVER_CORE_V => "Server Standard without Hyper-V",
        win32::PRODUCT_STANDARD_SERVER_V => "Server Standard without Hyper-V",
        win32::PRODUCT_STANDARD_SERVER_SOLUTIONS => "Server Solutions Premium",
        win32::PRODUCT_STANDARD_SERVER_SOLUTIONS_CORE => "Server Solutions Premium",
        win32::PRODUCT_STARTER => "Starter",
        win32::PRODUCT_STARTER_E => "Not supported",
        win32::PRODUCT_STARTER_N => "Starter N",
        win32::PRODUCT_STORAGE_ENTERPRISE_SERVER => "Storage Server Enterprise",
        win32::PRODUCT_STORAGE_ENTERPRISE_SERVER_CORE => "Storage Server Enterprise",
        win32::PRODUCT_STORAGE_EXPRESS_SERVER => "Storage Server Express",
        win32::PRODUCT_STORAGE_EXPRESS_SERVER_CORE => "Storage Server Express",
        win32::PRODUCT_STORAGE_STANDARD_EVALUATION_SERVER => "Storage Server Standard",
        win32::PRODUCT_STORAGE_STANDARD_SERVER => "Storage Server Standard",
        win32::PRODUCT_STORAGE_STANDARD_SERVER_CORE => "Storage Server Standard",
        win32::PRODUCT_STORAGE_WORKGROUP_EVALUATION_SERVER => "Storage Server Workgroup",
        win32::PRODUCT_STORAGE_WORKGROUP_SERVER => "Storage Server Workgroup",
        win32::PRODUCT_STORAGE_WORKGROUP_SERVER_CORE => "Storage Server Workgroup",
        win32::PRODUCT_ULTIMATE => "Ultimate",
        win32::PRODUCT_ULTIMATE_E => "Not supported",
        win32::PRODUCT_ULTIMATE_N => "Ultimate N",
        win32::PRODUCT_UNDEFINED => "An unknown product",
        win32::PRODUCT_WEB_SERVER => "Web Server",
        win32::PRODUCT_WEB_SERVER_CORE => "Web Server",
        _ => "",
    }
}
