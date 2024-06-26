// Copyright (c) Microsoft Corporation
// License: MIT OR Apache-2.0

use std::env;
use std::fs::OpenOptions;

use std::io::Write;
use std::path::{Path, PathBuf};

fn p(s: String) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("output.txt")
        .expect("Unable to open file");

    writeln!(file, "{}", s).expect("Unable to write to file");
}

fn main() -> Result<(), wdk_build::ConfigError> {
    // replace to wdm driver
    let wdk_sys_crate_dep_key = format!("DEP_WDK_{}", "wdk_config".to_ascii_uppercase());
    let wdk_crate_dep_key = format!("DEP_WDK-SYS_{}", "wdk_config".to_ascii_uppercase());

    let wdk_sys_crate_config_serialized = std::env::var(&wdk_sys_crate_dep_key);
    let wdk_crate_config_serialized = std::env::var(&wdk_crate_dep_key);

    if let Ok(s) = wdk_sys_crate_config_serialized {
        p(s.replace(
            "\"KMDF\":{\"kmdf_version_major\":1,\"kmdf_version_minor\":33}",
            "\"WDM\":[]",
        )
        .to_string());
        std::env::set_var(
            &wdk_sys_crate_dep_key,
            s.replace(
                "\"KMDF\":{\"kmdf_version_major\":1,\"kmdf_version_minor\":33}",
                "\"WDM\":[]",
            ),
        );
    }

    if let Ok(s) = wdk_crate_config_serialized {
        p(s.replace(
            "\"KMDF\":{\"kmdf_version_major\":1,\"kmdf_version_minor\":33}",
            "\"WDM\":[]",
        )
        .to_string());
        std::env::set_var(
            &wdk_crate_dep_key,
            s.replace(
                "\"KMDF\":{\"kmdf_version_major\":1,\"kmdf_version_minor\":33}",
                "\"WDM\":[]",
            ),
        );
    }

    // other c/c++ library dependences
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());

    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&root).join("lib").display()
    );
    println!("cargo::rustc-link-lib=distorm35");

    p(format!("root:{:?}", Path::new(&root).join("lib").display()));

    wdk_build::Config::from_env_auto()?.configure_binary_build();
    Ok(())
}
