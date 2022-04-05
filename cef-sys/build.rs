use anyhow::Result;
use std::env;
use std::path::PathBuf;

/*fn download_cef() -> Result<()> {
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
}*/

fn main() -> Result<()> {
    // make sure we have CEF downloaded
    // TODO: check for it being downloaded
    //download_cef().with_context(|| "Failed to download CEF!")?;
    // instead for now, make sure that cef's include, Release, and Resources folders are placed in the cef-sys directory

    // let us link the proper CEF version depending on what host we're compiling for
    let target_os = env::var("TARGET").expect("target");
    let cef_lib_name = match target_os.as_ref() {
        "x86_64-pc-windows-msvc" => "libcef",
        _ => "cef",
    };
    println!("cargo:rustc-link-lib={}", cef_lib_name);

    if cfg!(windows) {
        println!("cargo:rustc-link-lib=cef_sandbox");
        println!("cargo:rustc-link-lib=delayimp");
    }

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cef_lib_path = PathBuf::from(&manifest_dir).join("Release");
    assert!(cef_lib_path.exists());
    println!("cargo:rustc-link-search={}", cef_lib_path.display());

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", manifest_dir))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_type("cef_main_args_t")
        .allowlist_function("cef_execute_process")
        .allowlist_type("cef_settings_t")
        .allowlist_function("cef_initialize")
        .allowlist_function("cef_run_message_loop")
        .allowlist_function("cef_shutdown")
        .allowlist_type("cef_string_t")
        .allowlist_function("cef_string_utf8_to_utf16")
        .allowlist_type("cef_base_ref_counted_t")
        .allowlist_type("cef_client_t")
        .allowlist_type("cef_life_span_handler_t")
        .allowlist_type("cef_display_handler_t")
        .allowlist_type("cef_browser_t")
        .allowlist_function("cef_browser_view_get_for_browser")
        .allowlist_function("cef_quit_message_loop")
        .allowlist_type("cef_frame_t")
        .allowlist_type("cef_load_handler_t")
        .allowlist_type("cef_app_t")
        .allowlist_type("cef_browser_process_handler_t")
        .allowlist_type("cef_browser_settings_t")
        .allowlist_type("cef_browser_view_delegate_t")
        .allowlist_type("cef_window_delegate_t")
        .allowlist_function("cef_browser_view_create")
        .allowlist_function("cef_window_create_top_level")
        .allowlist_type("cef_window_delegate_t")
        .allowlist_type("cef_browser_view_delegate_t")
        .allowlist_type("cef_view_delegate_t")
        .allowlist_type("cef_panel_delegate_t")
        .allowlist_type("cef_size_t")
        .allowlist_type("cef_render_handler_t")
        .allowlist_type("cef_text_input_mode_t")
        .allowlist_function("cef_sandbox_info_create")
        .allowlist_function("cef_sandbox_info_destroy")
        .allowlist_function("cef_enable_highdpi_support")
        .allowlist_function("cef_currently_on")
        .allowlist_type("cef_thread_id_t")
        .derive_default(true)
        .generate()
        .expect("Can generate CAPI bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("cef_bindings.rs"))
        .expect("Can write bindings");

    Ok(())
}
