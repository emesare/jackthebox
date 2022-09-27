use std::{env, path::Path, process::Command};

#[cfg(target_os = "windows")]
pub fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let from_path = Path::new("bin/Debug").join("Jackload.dll");
    let dest_path = Path::new(&out_dir).join("Jackload.dll");
    env::set_current_dir("./loader").expect("go to loader dir");
    let _out = Command::new("dotnet")
        .arg("build")
        .output()
        .expect("failed to build loader");
    if from_path.exists() {
        std::fs::copy(&from_path, &dest_path).expect("copy loader assembly to dest");
    } else {
        println!("cargo:warning=failed to build loader assembly");
    }
}
