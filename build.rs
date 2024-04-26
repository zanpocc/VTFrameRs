// Copyright (c) Microsoft Corporation
// License: MIT OR Apache-2.0

use std::fs::OpenOptions;

use std::io::Write;

fn p(s: String){
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("output.txt")
        .expect("Unable to open file");

    writeln!(file, "{}", s).expect("Unable to write to file");
}

fn main() -> Result<(), wdk_build::ConfigError> {

    p(format!("This is a debug message from build.rs"));

    let wdk_sys_crate_dep_key =
        format!("DEP_WDK_{}", "wdk_config".to_ascii_uppercase());
    let wdk_crate_dep_key = format!(
        "DEP_WDK-SYS_{}",
        "wdk_config".to_ascii_uppercase()
    );

    let wdk_sys_crate_config_serialized = std::env::var(&wdk_sys_crate_dep_key);
    let wdk_crate_config_serialized = std::env::var(&wdk_crate_dep_key);

    match wdk_sys_crate_config_serialized {
        Ok(s) =>{
            p(format!("{}",s.replace("\"KMDF\":{\"kmdf_version_major\":1,\"kmdf_version_minor\":33}","\"WDM\":[]")));
            std::env::set_var(&wdk_sys_crate_dep_key,s.replace("\"KMDF\":{\"kmdf_version_major\":1,\"kmdf_version_minor\":33}","\"WDM\":[]"));
        }
        Err(_) =>{}
    }

    match wdk_crate_config_serialized {
        Ok(s) =>{
            p(format!("{}",s.replace("\"KMDF\":{\"kmdf_version_major\":1,\"kmdf_version_minor\":33}","\"WDM\":[]")));
            std::env::set_var(&wdk_crate_dep_key,s.replace("\"KMDF\":{\"kmdf_version_major\":1,\"kmdf_version_minor\":33}","\"WDM\":[]"));
        }
        Err(_) =>{}
    }

    wdk_build::Config::from_env_auto()?.configure_binary_build();
    Ok(())
}
