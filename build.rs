use cmake;
use std::env;
use std::path::PathBuf;

fn main() {
    use cmake::Config;

    // first make a new build on fallout2-ce
    let dst = Config::new("fallout2-ce").build();

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=native={}", dst.display());

    // Tell cargo to tell rustc to link the fallout2-ce static library.
    println!("cargo:rustc-link-lib=static=fallout2-ce");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=build.rs");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    add_art_h_to_bindings(out_path);
}

fn assign_defaults(builder: bindgen::Builder) -> bindgen::Builder {
    let builder_with_defaults = builder
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        })
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .prepend_enum_name(false)
        .disable_name_namespacing()
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-I")
        .clang_arg("/usr/include/SDL2")
        .clang_arg("-fparse-all-comments");
    builder_with_defaults
}

fn write_bindings(sourcefile: &str, builder: bindgen::Builder, out_path: PathBuf) {
    let bindings = assign_defaults(builder)
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file(out_path.join(String::from("bindings_") + sourcefile + ".rs"))
        .expect("Couldn't write bindings!");
    bindings
        .write_to_file(String::from("src/bindings_") + sourcefile + ".rs")
        .expect("Couldn't write bindings!");
}

fn add_art_h_to_bindings(out_path: PathBuf) {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        // .header("wrapper.h")
        .header("fallout2-ce/src/art.h")
        .allowlist_type("fallout::DudeNativeLook")
        .allowlist_type("fallout::FrmImage")
        .allowlist_type("fallout::ArtFrame")
        .allowlist_type("fallout::Art")
        .allowlist_type("fallout::Head")
        .allowlist_type("fallout::HeadAnimation")
        .allowlist_type("fallout::Background")
        .allowlist_type("fallout::WeaponAnimation")
        .allowlist_function("fallout::artInit")
        .allowlist_function("fallout::artReset")
        .allowlist_function("fallout::artExit")
        .allowlist_function("fallout::artGetObjectTypeName")
        .allowlist_function("fallout::artIsObjectTypeHidden")
        .allowlist_function("fallout::artGetFidgetCount")
        .allowlist_function("fallout::artRender")
        .allowlist_function("fallout::art_list_str")
        .allowlist_function("fallout::artLock")
        .allowlist_function("fallout::artLockFrameData")
        .allowlist_function("fallout::artLockFrameDataReturningSize")
        .allowlist_function("fallout::artUnlock")
        .allowlist_function("fallout::artCacheFlush")
        .allowlist_function("fallout::artCopyFileName")
        .allowlist_function("fallout::_art_get_code")
        .allowlist_function("fallout::artBuildFilePath")
        .allowlist_function("fallout::artGetFramesPerSecond")
        .allowlist_function("fallout::artGetActionFrame")
        .allowlist_function("fallout::artGetFrameCount")
        .allowlist_function("fallout::artGetWidth")
        .allowlist_function("fallout::artGetHeight")
        .allowlist_function("fallout::artGetSize")
        .allowlist_function("fallout::artGetFrameOffsets")
        .allowlist_function("fallout::artGetRotationOffsets")
        .allowlist_function("fallout::artGetFrameData")
        .allowlist_function("fallout::artGetFrame")
        .allowlist_function("fallout::artExists")
        .allowlist_function("fallout::_art_fid_valid")
        .allowlist_function("fallout::_art_alias_num")
        .allowlist_function("fallout::artCritterFidShouldRun")
        .allowlist_function("fallout::artAliasFid")
        .allowlist_function("fallout::buildFid")
        .allowlist_function("fallout::artLoad")
        .allowlist_function("fallout::artRead")
        .allowlist_function("fallout::artWrite");
    write_bindings("art_h", builder, out_path);
}
