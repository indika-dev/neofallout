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
    add_art_h_to_bindings(&out_path);
    add_autorun_h_to_bindings(&out_path);
    add_color_h_to_bindings(&out_path);
    add_credits_h_to_bindings(&out_path);
    add_cycle_h_to_bindings(&out_path);
    add_db_h_to_bindings(&out_path);
    add_debug_h_to_bindings(&out_path);
    add_draw_h_to_bindings(&out_path);
    add_endgame_h_to_bindings(&out_path);
    add_game_h_to_bindings(&out_path);
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

fn write_bindings(sourcefile: &str, builder: &bindgen::Builder, out_path: PathBuf) {
    let bindings = assign_defaults(builder.clone())
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file(out_path.join(String::from("bindings_") + sourcefile + ".rs"))
        .expect("Couldn't write bindings!");
    bindings
        .write_to_file(String::from("src/bindings_") + sourcefile + ".rs")
        .expect("Couldn't write bindings!");
}

fn add_game_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/game.h")
        .allowlist_type("fallout::GameState")
        .allowlist_type("fallout::MessageList")
        .allowlist_type("fallout::GameMode")
        .allowlist_type("fallout::GameMode::Flags")
        .allowlist_type("fallout::ScopedGameMode")
        .allowlist_function("fallout::gameInitWithOptions")
        .allowlist_function("fallout::gameReset")
        .allowlist_function("fallout::gameExit")
        .allowlist_function("fallout::gameHandleKey")
        .allowlist_function("fallout::gameUiDisable")
        .allowlist_function("fallout::gameUiEnable")
        .allowlist_function("fallout::gameUiIsDisabled")
        .allowlist_function("fallout::gameGetGlobalVar")
        .allowlist_function("fallout::gameSetGlobalVar")
        .allowlist_function("fallout::globalVarsRead")
        .allowlist_function("fallout::gameGetState")
        .allowlist_function("fallout::gameRequestState")
        .allowlist_function("fallout::gameUpdateState")
        .allowlist_function("fallout::showQuitConfirmationDialog")
        .allowlist_function("fallout::gameShowDeathDialog")
        .allowlist_function("fallout::gameGetGlobalPointer")
        .allowlist_function("fallout::gameSetGlobalPointer");
    write_bindings("game_h", &builder, out_path.clone());
    return builder;

    // extern int* gGameGlobalVars;
    // extern int gGameGlobalVarsLength;
    // extern const char* asc_5186C8;
    // extern int _game_user_wants_to_quit;
}

fn add_endgame_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/endgame.h")
        .allowlist_type("fallout::EndgameDeathEndingReason")
        .allowlist_function("fallout::endgamePlaySlideshow")
        .allowlist_function("fallout::endgamePlayMovie")
        .allowlist_function("fallout::endgameDeathEndingInit")
        .allowlist_function("fallout::endgameDeathEndingExit")
        .allowlist_function("fallout::endgameSetupDeathEnding")
        .allowlist_function("fallout::endgameDeathEndingGetFileName");
    write_bindings("endgame_h", &builder, out_path.clone());
    return builder;
}

fn add_draw_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/draw.h")
        .allowlist_function("fallout::bufferDrawLine")
        .allowlist_function("fallout::bufferDrawRect")
        .allowlist_function("fallout::bufferDrawRectShadowed")
        .allowlist_function("fallout::blitBufferToBufferStretch")
        .allowlist_function("fallout::blitBufferToBufferStretchTrans")
        .allowlist_function("fallout::blitBufferToBuffer")
        .allowlist_function("fallout::blitBufferToBufferTrans")
        .allowlist_function("fallout::bufferFill")
        .allowlist_function("fallout::_buf_texture")
        .allowlist_function("fallout::_lighten_buf")
        .allowlist_function("fallout::_swap_color_buf")
        .allowlist_function("fallout::bufferOutline")
        .allowlist_function("fallout::srcCopy")
        .allowlist_function("fallout::transSrcCopy");
    write_bindings("draw_h", &builder, out_path.clone());
    return builder;
}

fn add_debug_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/debug.h")
        .allowlist_function("fallout::_GNW_debug_init")
        .allowlist_function("fallout::_debug_register_mono")
        .allowlist_function("fallout::_debug_register_log")
        .allowlist_function("fallout::_debug_register_screen")
        .allowlist_function("fallout::_debug_register_env")
        .allowlist_function("fallout::_debug_register_func")
        .allowlist_function("fallout::debugPrint")
        .allowlist_function("fallout::_debug_exit");
    write_bindings("debug_h", &builder, out_path.clone());
    return builder;
}

fn add_db_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/db.h")
        .allowlist_function("fallout::dbOpen")
        .allowlist_function("fallout::_db_total")
        .allowlist_function("fallout::dbExit")
        .allowlist_function("fallout::dbGetFileSize")
        .allowlist_function("fallout::dbGetFileContents")
        .allowlist_function("fallout::fileClose")
        .allowlist_function("fallout::fileOpen")
        .allowlist_function("fallout::filePrintFormatted")
        .allowlist_function("fallout::fileReadChar")
        .allowlist_function("fallout::fileReadString")
        .allowlist_function("fallout::fileWriteString")
        .allowlist_function("fallout::fileRead")
        .allowlist_function("fallout::fileWrite")
        .allowlist_function("fallout::fileSeek")
        .allowlist_function("fallout::fileTell")
        .allowlist_function("fallout::fileRewind")
        .allowlist_function("fallout::fileEof")
        .allowlist_function("fallout::fileReadUInt8")
        .allowlist_function("fallout::fileReadInt16")
        .allowlist_function("fallout::fileReadUInt16")
        .allowlist_function("fallout::fileReadInt32")
        .allowlist_function("fallout::fileReadUInt32")
        .allowlist_function("fallout::_db_freadInt")
        .allowlist_function("fallout::fileReadFloat")
        .allowlist_function("fallout::fileReadBool")
        .allowlist_function("fallout::fileWriteUInt8")
        .allowlist_function("fallout::fileWriteInt16")
        .allowlist_function("fallout::fileWriteUInt16")
        .allowlist_function("fallout::fileWriteInt32")
        .allowlist_function("fallout::_db_fwriteLong")
        .allowlist_function("fallout::fileWriteUInt32")
        .allowlist_function("fallout::fileWriteFloat")
        .allowlist_function("fallout::fileWriteBool")
        .allowlist_function("fallout::fileReadUInt8List")
        .allowlist_function("fallout::fileReadFixedLengthString")
        .allowlist_function("fallout::fileReadInt16List")
        .allowlist_function("fallout::fileReadUInt16List")
        .allowlist_function("fallout::fileReadInt32List")
        .allowlist_function("fallout::_db_freadIntCount")
        .allowlist_function("fallout::fileReadUInt32List")
        .allowlist_function("fallout::fileWriteUInt8List")
        .allowlist_function("fallout::fileWriteFixedLengthString")
        .allowlist_function("fallout::fileWriteInt16List")
        .allowlist_function("fallout::fileWriteUInt16List")
        .allowlist_function("fallout::fileWriteInt32List")
        .allowlist_function("fallout::_db_fwriteLongCount")
        .allowlist_function("fallout::fileWriteUInt32List")
        .allowlist_function("fallout::fileNameListInit")
        .allowlist_function("fallout::fileNameListFree")
        .allowlist_function("fallout::fileGetSize")
        .allowlist_function("fallout::fileSetReadProgressHandler");
    write_bindings("db_h", &builder, out_path.clone());
    return builder;
}

fn add_cycle_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/cycle.h")
        .allowlist_function("fallout::colorCycleInit")
        .allowlist_function("fallout::colorCycleReset")
        .allowlist_function("fallout::colorCycleFree")
        .allowlist_function("fallout::colorCycleDisable")
        .allowlist_function("fallout::colorCycleEnable")
        .allowlist_function("fallout::colorCycleEnabled")
        .allowlist_function("fallout::cycleSetSpeedFactor")
        .allowlist_function("fallout::colorCycleTicker");
    write_bindings("cycle_h", &builder, out_path.clone());
    return builder;
}

fn add_credits_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/credits.h")
        .allowlist_function("fallout::creditsOpen");
    write_bindings("credits_h", &builder, out_path.clone());
    return builder;
}

fn add_color_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/color.h")
        .allowlist_type("fallout::MallocProc")
        .allowlist_type("fallout::ReallocProc")
        .allowlist_type("fallout::FreeProc")
        .allowlist_function("fallout::colorPaletteSetFileIO")
        .allowlist_function("fallout::_calculateColor")
        .allowlist_function("fallout::Color2RGB")
        .allowlist_function("fallout::colorPaletteFadeBetween")
        .allowlist_function("fallout::colorPaletteSetTransitionCallback")
        .allowlist_function("fallout::_setSystemPalette")
        .allowlist_function("fallout::_getSystemPalette")
        .allowlist_function("fallout::_setSystemPaletteEntries")
        .allowlist_function("fallout::_getColorBlendTable")
        .allowlist_function("fallout::_freeColorBlendTable")
        .allowlist_function("fallout::colorPaletteSetMemoryProcs")
        .allowlist_function("fallout::colorSetBrightness")
        .allowlist_function("fallout::colorPaletteLoad")
        .allowlist_function("fallout::colorPushColorPalette")
        .allowlist_function("fallout::_colorError")
        .allowlist_function("fallout::colorPopColorPalette")
        .allowlist_function("fallout::_initColors")
        .allowlist_function("fallout::_colorsClose");
    write_bindings("color_h", &builder, out_path.clone());
    return builder;
}

fn add_character_selector_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/character_selector.h")
        .allowlist_function("fallout::characterSelectorOpen")
        .allowlist_function("fallout::premadeCharactersInit")
        .allowlist_function("fallout::premadeCharactersExit");
    write_bindings("character_selector_h", &builder, out_path.clone());
    return builder;
}

fn add_autorun_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/autorun.h")
        .allowlist_function("fallout::autorunMutexCreate")
        .allowlist_function("fallout::autorunMutexClose");
    write_bindings("autorun_h", &builder, out_path.clone());
    return builder;
}

fn add_art_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
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
    write_bindings("art_h", &builder, out_path.clone());
    return builder;
}
