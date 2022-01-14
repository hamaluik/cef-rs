use anyhow::{Context, Result};
use std::env;
use std::path::PathBuf;

fn download_cef() -> Result<()> {
    #[cfg(target_os = "linux")]
    let url = "https://cef-builds.spotifycdn.com/cef_binary_97.1.2%2Bgb821dc3%2Bchromium-97.0.4692.71_linux64_minimal.tar.bz2";
    #[cfg(target_os = "linux")]
    let filename = "cef_binary_97.1.1+g50067f2+chromium-97.0.4692.71_linux64_minimal.tar.bz2";

    let path = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = PathBuf::from(&path).join(filename);
    if path.exists() {
        return Ok(());
    }

    use curl::easy::Easy;
    use std::io::Write;

    let mut handle = Easy::new();
    handle
        .progress(true)
        .with_context(|| "Failed to enable progress reporting")?;
    handle
        .url(url)
        .with_context(|| format!("Failed to open url {}", url))?;
    let mut file = std::fs::File::create(&path)
        .with_context(|| format!("Failed to create download file path {}", path.display()))?;
    handle
        .write_function(move |data| {
            file.write_all(data).unwrap();
            Ok(data.len())
        })
        .with_context(|| "Failed to cURL write function")?;
    handle
        .perform()
        .with_context(|| "Failed to initiate download")?;

    Ok(())
}

fn main() -> Result<()> {
    // make sure we have CEF downloaded
    // TODO: check for it being downloaded
    download_cef().with_context(|| "Failed to download CEF!")?;

    // let us link the proper CEF version depending on what host we're compiling for
    let target_os = env::var("TARGET").expect("target");
    let cef_lib_name = match target_os.as_ref() {
        "x86_64-pc-windows-msvc" => "libcef",
        _ => "cef",
    };
    println!("cargo:rustc-link-lib={}", cef_lib_name);

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cef_lib_path = PathBuf::from(&manifest_dir).join("Release");
    assert!(cef_lib_path.exists());
    println!("cargo:rustc-link-search={}", cef_lib_path.display());

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", manifest_dir))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .derive_default(true)
        .generate()
        .expect("Can generate CAPI bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("cef_bindings.rs"))
        .expect("Can write bindings");

    Ok(())
}
