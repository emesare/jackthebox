use std::{env, path::Path, process::Command};

#[cfg(target_os = "windows")]
pub fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("Jackload.dll");
    env::set_current_dir("./loader").expect("go to loader dir");
    Command::new("dotnet")
        .arg("build")
        .output()
        .expect("failed to build loader");
    std::fs::copy("bin/Debug/Jackload.dll", &dest_path).expect("copy loader assembly to dest");
}
