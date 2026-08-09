fn main() {
    #[cfg(all(target_os = "windows", target_env = "msvc"))]
    set_windows_exe_options();
}

/// Embed a Windows manifest and set some linker options.
#[cfg(all(target_os = "windows", target_env = "msvc"))]
fn set_windows_exe_options() {
    static MANIFEST: &str = "res\\gcl.exe.manifest";

    let Ok(mut manifest) = std::env::current_dir() else {
        return;
    };
    manifest.push(MANIFEST);
    let Some(manifest) = manifest.to_str() else {
        return;
    };

    println!("cargo:rerun-if-changed={MANIFEST}");
    // Embed the Windows application manifest file.
    println!("cargo:rustc-link-arg-bin=gcl=/MANIFEST:EMBED");
    println!("cargo:rustc-link-arg-bin=gcl=/MANIFESTINPUT:{manifest}");
    // Turn linker warnings into errors. Helps debugging, otherwise the
    // warnings get squashed.
    println!("cargo:rustc-link-arg-bin=gcl=/WX");
}
