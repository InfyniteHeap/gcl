const COMPANY_NAME: &str = "InfyniteHeap";
const FILE_DESCRIPTION: &str = "A beautiful, fast and memory-safe Minecraft launcher.";
const BINARY_NAME: &str = "gcl";
const PRODUCT_NAME: &str = "Grid Craft Launcher";

#[cfg(all(target_os = "windows", target_env = "msvc"))]
fn parse_version(version: &str) -> Vec<String> {
    let mut parts = Vec::new();

    'outer: for part in version.split('.') {
        let mut digits = 0;
        for c in part.chars() {
            if !c.is_ascii_digit() {
                parts.push(part[..digits].to_string());

                break 'outer;
            } else {
                digits += 1;
            }
        }

        if !part.is_empty() {
            parts.push(part.to_string());
            if parts.len() == 4 {
                break;
            }
        }
    }

    while parts.len() < 4 {
        parts.push("0".into());
    }

    parts
}

fn main() {
    #[cfg(all(target_os = "windows", target_env = "msvc"))]
    embed_resources();
    #[cfg(not(all(target_os = "windows", target_env = "msvc")))]
    compile_error!("This project must be built on Windows with MSVC toolchain!");
}

#[cfg(all(target_os = "windows", target_env = "msvc"))]
fn embed_resources() {
    println!("cargo:rerun-if-changed=res/gcl.exe.manifest");
    println!("cargo:rerun-if-env-changed=CARGO_PKG_VERSION");

    let version = std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.0.0".into());
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");

    build_manifest(&version, &out_dir);
    let rc_path = build_resource_file(&version, &out_dir);

    embed_resource::compile(rc_path, embed_resource::NONE)
        .manifest_required()
        .unwrap();
}

#[cfg(all(target_os = "windows", target_env = "msvc"))]
fn build_manifest(version: &str, out_dir: &str) {
    let raw_manifest = std::fs::read_to_string("res/gcl.exe.manifest")
        .expect("failed to read res/gcl.exe.manifest");
    let manifest = raw_manifest.replace("{{VERSION}}", &parse_version(version).join("."));

    let manifest_path = std::path::Path::new(out_dir).join("gcl.exe.manifest");
    std::fs::write(&manifest_path, manifest).expect("failed to write generated gcl.exe.manifest");
}

#[cfg(all(target_os = "windows", target_env = "msvc"))]
fn build_resource_file(version: &str, out_dir: &str) -> std::path::PathBuf {
    let file_version = parse_version(version).join(",");
    let file_flags = match std::env::var("PROFILE") {
        Ok(p) if p == "debug" => "VS_FF_DEBUG",
        _ => "0",
    };

    let raw_rc = format!(
        r#"#include <winres.h>

CREATEPROCESS_MANIFEST_RESOURCE_ID RT_MANIFEST "gcl.exe.manifest"

VS_VERSION_INFO VERSIONINFO
FILEVERSION     {file_version}
PRODUCTVERSION  {file_version}
FILEFLAGSMASK   VS_FFI_FILEFLAGSMASK
FILEFLAGS       {file_flags}
FILEOS          VOS_NT_WINDOWS32
FILETYPE        VFT_APP
FILESUBTYPE     VFT2_UNKNOWN
BEGIN
    BLOCK "StringFileInfo"
    BEGIN
        BLOCK "040904B0"
        BEGIN
            VALUE "CompanyName",      "{COMPANY_NAME}"
            VALUE "FileDescription",  "{FILE_DESCRIPTION}"
            VALUE "FileVersion",      "{version}"
            VALUE "InternalName",     "{BINARY_NAME}"
            VALUE "OriginalFilename", "{BINARY_NAME}.exe"
            VALUE "ProductName",      "{PRODUCT_NAME}"
            VALUE "ProductVersion",   "{version}"
        END
    END
    BLOCK "VarFileInfo"
    BEGIN
        VALUE "Translation", 0x0409, 1200
    END
END
"#
    );
    let rc = format!("\u{FEFF}{raw_rc}");

    let rc_path = std::path::Path::new(out_dir).join("gcl.rc");
    std::fs::write(&rc_path, rc).expect("failed to write generated gcl.rc");

    rc_path
}
