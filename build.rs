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
    println!("cargo:rerun-if-changed=src/wrapper.h");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    // all SDL related stuff isn't transpiled to bindings
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    add_wrapper_h_to_bindings(&out_path);
    // add_art_h_to_bindings(&out_path);
    // add_autorun_h_to_bindings(&out_path);
    // add_color_h_to_bindings(&out_path);
    // add_credits_h_to_bindings(&out_path);
    // add_cycle_h_to_bindings(&out_path);
    // add_db_h_to_bindings(&out_path);
    // add_debug_h_to_bindings(&out_path);
    // add_draw_h_to_bindings(&out_path);
    // add_endgame_h_to_bindings(&out_path);
    // add_game_h_to_bindings(&out_path);
    // add_game_mouse_h_to_bindings(&out_path);
    // add_game_movie_h_to_bindings(&out_path);
    // add_game_sound_h_to_bindings(&out_path);
    // add_input_h_to_bindings(&out_path);
    // // add_kb_h_to_bindings(&out_path);
    // add_loadsave_h_to_bindings(&out_path);
    // add_mainmenu_h_to_bindings(&out_path);
    // add_object_h_to_bindings(&out_path);
    // add_palette_h_to_bindings(&out_path);
    // add_platform_compat_h_to_bindings(&out_path);
    // add_preferences_h_to_bindings(&out_path);
    // add_proto_h_to_bindings(&out_path);
    // add_random_h_to_bindings(&out_path);
    // add_script_h_to_bindings(&out_path);
    // add_selfrun_h_to_bindings(&out_path);
    // add_settings_h_to_bindings(&out_path);
    // add_sfall_config_h_to_bindings(&out_path);
    // add_sfall_global_scripts_h_to_bindings(&out_path);
    // // add_svga_h_to_bindings(&out_path);
    // add_text_font_h_to_bindings(&out_path);
    // add_window_h_to_bindings(&out_path);
    // add_window_manager_h_to_bindings(&out_path);
    // add_window_manager_private_h_to_bindings(&out_path);
    // add_word_wrap_h_to_bindings(&out_path);
    // add_worldmap_h_to_bindings(&out_path);
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

fn add_wrapper_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("src/wrapper.h")
        .allowlist_type("fallout::File")
        .allowlist_type("fallout::FrmImage")
        .allowlist_type("fallout::EndgameDeathEndingReason")
        .allowlist_type("fallout::SoundEndCallback")
        .allowlist_type("fallout::ObjectFlags")
        .allowlist_type("fallout::InventoryItem")
        .allowlist_type("fallout::Inventory")
        .allowlist_type("fallout::WeaponObjectData")
        .allowlist_type("fallout::AmmoItemData")
        .allowlist_type("fallout::MiscItemData")
        .allowlist_type("fallout::KeyItemData")
        .allowlist_type("fallout::ItemObjectData")
        .allowlist_type("fallout::CritterCombatData")
        .allowlist_type("fallout::CritterObjectData")
        .allowlist_type("fallout::DoorSceneryData")
        .allowlist_type("fallout::StairsSceneryData")
        .allowlist_type("fallout::ElevatorSceneryData")
        .allowlist_type("fallout::LadderSceneryData")
        .allowlist_type("fallout::SceneryObjectData")
        .allowlist_type("fallout::MiscObjectData")
        .allowlist_type("fallout::ObjectData")
        .allowlist_type("fallout::Object")
        .allowlist_type("fallout::MouseCursorType")
        .allowlist_type("fallout::Dam")
        .allowlist_type("fallout::WindowFlags")
        .allowlist_type("fallout::FpsLimiter")
        .allowlist_type("fallout::MainMenuOption")
        .allowlist_type("fallout::LoadSaveMode")
        .allowlist_type("fallout::GameMovie")
        .allowlist_type("fallout::DictionaryReadProc")
        .allowlist_type("fallout::DictionaryWriteProc")
        .allowlist_type("fallout::DictionaryIO")
        .allowlist_type("fallout::DictionaryEntry")
        .allowlist_type("fallout::Dictionary")
        .allowlist_type("fallout::Config")
        .allowlist_type("fallout::GameMovieFlags")
        .allowlist_function("fallout::autorunMutexCreate")
        .allowlist_function("fallout::autorunMutexClose")
        .allowlist_function("fallout::colorPaletteLoad")
        .allowlist_function("fallout::paletteFadeTo")
        .allowlist_function("fallout::debugPrint")
        .allowlist_function("fallout::gameMoviePlay")
        .allowlist_function("fallout::lsgLoadGame")
        .allowlist_function("fallout::mainMenuWindowFree")
        .allowlist_function("fallout::mainMenuWindowHide")
        .allowlist_function("fallout::mainMenuWindowHandleEvents")
        .allowlist_function("fallout::screenGetWidth")
        .allowlist_function("fallout::screenGetHeight")
        .allowlist_function("fallout::renderPresent")
        .allowlist_function("fallout::sfall_gl_scr_exec_start_proc")
        .allowlist_function("fallout::sfall_gl_scr_process_main")
        .allowlist_function("fallout::windowCreate")
        .allowlist_function("fallout::windowDestroy")
        .allowlist_function("fallout::windowGetBuffer")
        .allowlist_function("fallout::windowRefresh")
        .allowlist_function("fallout::gameMouseSetCursor")
        .allowlist_function("fallout::mouseGetEvent")
        .allowlist_function("fallout::mouseShowCursor")
        .allowlist_function("fallout::mouseHideCursor")
        .allowlist_function("fallout::cursorIsHidden")
        .allowlist_function("fallout::_map_init")
        .allowlist_function("fallout::_map_exit")
        .allowlist_function("fallout::backgroundSoundDelete")
        .allowlist_function("fallout::_gsound_speech_play_preloaded")
        .allowlist_function("fallout::speechDelete")
        .allowlist_function("fallout::speechLoad")
        .allowlist_function("fallout::speechSetEndCallback")
        .allowlist_function("fallout::creditsOpen")
        .allowlist_function("fallout::doPreferences")
        .allowlist_function("fallout::endgameSetupDeathEnding")
        .allowlist_function("fallout::endgameDeathEndingGetFileName")
        .allowlist_function("fallout::fileNameListInit")
        .allowlist_function("fallout::fileClose")
        .allowlist_function("fallout::fileOpen")
        .allowlist_function("fallout::fileReadChar")
        .allowlist_function("fallout::gameHandleKey")
        .allowlist_function("fallout::gameInitWithOptions")
        .allowlist_function("fallout::gameExit")
        .allowlist_function("fallout::gameReset")
        .allowlist_function("fallout::inputGetInput")
        .allowlist_function("fallout::inputEventQueueReset")
        .allowlist_function("fallout::getTicks")
        .allowlist_function("fallout::inputPauseForTocks")
        .allowlist_function("fallout::inputBlockForTocks")
        .allowlist_function("fallout::objectHide")
        .allowlist_function("fallout::objectShow")
        .allowlist_function("fallout::scriptsEnable")
        .allowlist_function("fallout::scriptsDisable")
        .allowlist_function("fallout::scriptsHandleRequests")
        .allowlist_function("fallout::selfrunInitFileList")
        .allowlist_function("fallout::selfrunFreeFileList")
        .allowlist_function("fallout::selfrunPreparePlayback")
        .allowlist_function("fallout::selfrunPrepareRecording")
        .allowlist_function("fallout::selfrunRecordingLoop")
        .allowlist_function("fallout::wmMapMusicStart")
        .allowlist_function("fallout::mapLoadByName")
        .allowlist_function("fallout::_proto_dude_init")
        .allowlist_function("fallout::_win_list_select")
        .allowlist_function("fallout::artCacheFlush")
        .allowlist_function("fallout::blitBufferToBuffer")
        .allowlist_function("fallout::bufferFill")
        .allowlist_function("fallout::colorCycleDisable")
        .allowlist_function("fallout::colorCycleEnabled")
        .allowlist_function("fallout::colorCycleEnable")
        .allowlist_function("fallout::keyboardReset")
        .allowlist_function("fallout::randomSeedPrerandom")
        .allowlist_function("fallout::characterSelectorOpen")
        .allowlist_function("fallout::mapHandleTransition")
        .allowlist_function("fallout::configGetString")
        .allowlist_function("fallout::configGetInt")
        .allowlist_function("fallout::mainMenuWindowInit")
        .allowlist_function("fallout::mainMenuWindowUnhide")
        .allowlist_function("fallout::_gsound_background_play_level_music")
        .allowlist_function("fallout::fileNameListFree")
        .allowlist_function("fallout::selfrunPlaybackLoop")
        .allowlist_function("fallout::wordWrap");
    write_bindings("fallout2ce_h", &builder, out_path.clone());
    return builder;
}

fn add_worldmap_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/worldmap.h")
        .allowlist_type("fallout::MapFlags")
        .allowlist_type("fallout::CityState")
        .allowlist_type("fallout::City")
        .allowlist_type("fallout::Map")
        .allowlist_function("fallout::wmWorldMap_init")
        .allowlist_function("fallout::wmWorldMap_exit")
        .allowlist_function("fallout::wmWorldMap_reset")
        .allowlist_function("fallout::wmWorldMap_save")
        .allowlist_function("fallout::wmWorldMap_load")
        .allowlist_function("fallout::wmMapMaxCount")
        .allowlist_function("fallout::wmMapIdxToName")
        .allowlist_function("fallout::wmMapMatchNameToIdx")
        .allowlist_function("fallout::wmMapIdxIsSaveable")
        .allowlist_function("fallout::wmMapIsSaveable")
        .allowlist_function("fallout::wmMapDeadBodiesAge")
        .allowlist_function("fallout::wmMapCanRestHere")
        .allowlist_function("fallout::wmMapPipboyActive")
        .allowlist_function("fallout::wmMapMarkVisited")
        .allowlist_function("fallout::wmMapMarkMapEntranceState")
        .allowlist_function("fallout::wmWorldMap")
        .allowlist_function("fallout::wmCheckGameAreaEvents")
        .allowlist_function("fallout::wmSetupRandomEncounter")
        .allowlist_function("fallout::wmEvalTileNumForPlacement")
        .allowlist_function("fallout::wmSubTileMarkRadiusVisited")
        .allowlist_function("fallout::wmSubTileGetVisitedState")
        .allowlist_function("fallout::wmGetAreaIdxName")
        .allowlist_function("fallout::wmAreaIsKnown")
        .allowlist_function("fallout::wmAreaVisitedState")
        .allowlist_function("fallout::wmMapIsKnown")
        .allowlist_function("fallout::wmAreaMarkVisited")
        .allowlist_function("fallout::wmAreaMarkVisitedState")
        .allowlist_function("fallout::wmAreaSetVisibleState")
        .allowlist_function("fallout::wmAreaSetWorldPos")
        .allowlist_function("fallout::wmGetPartyWorldPos")
        .allowlist_function("fallout::wmGetPartyCurArea")
        .allowlist_function("fallout::wmTownMap")
        .allowlist_function("fallout::wmCarUseGas")
        .allowlist_function("fallout::wmCarFillGas")
        .allowlist_function("fallout::wmCarGasAmount")
        .allowlist_function("fallout::wmCarIsOutOfGas")
        .allowlist_function("fallout::wmCarCurrentArea")
        .allowlist_function("fallout::wmCarGiveToParty")
        .allowlist_function("fallout::wmSfxMaxCount")
        .allowlist_function("fallout::wmSfxRollNextIdx")
        .allowlist_function("fallout::wmSfxIdxName")
        .allowlist_function("fallout::wmMapMusicStart")
        .allowlist_function("fallout::wmSetMapMusic")
        .allowlist_function("fallout::wmMatchAreaContainingMapIdx")
        .allowlist_function("fallout::wmTeleportToArea")
        .allowlist_function("fallout::wmSetPartyWorldPos")
        .allowlist_function("fallout::wmCarSetCurrentArea")
        .allowlist_function("fallout::wmForceEncounter");
    write_bindings("worldmap_h", &builder, out_path.clone());
    return builder;

    // extern unsigned char* circleBlendTable;
}

fn add_word_wrap_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/word_wrap.h")
        .allowlist_function("fallout::wordWrap");
    write_bindings("word_wrap_h", &builder, out_path.clone());
    return builder;
}

fn add_window_manager_private_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/window_manager_private.h")
        .allowlist_type("fallout::MenuBar")
        .allowlist_function("fallout::_win_list_select")
        .allowlist_function("fallout::_win_list_select_at")
        .allowlist_function("fallout::_win_get_str")
        .allowlist_function("fallout::win_yes_no")
        .allowlist_function("fallout::_win_msg")
        .allowlist_function("fallout::_win_pull_down")
        .allowlist_function("fallout::_create_pull_down")
        .allowlist_function("fallout::_win_debug")
        .allowlist_function("fallout:: _win_debug_delete")
        .allowlist_function("fallout::_win_register_menu_bar")
        .allowlist_function("fallout::_win_register_menu_pulldown")
        .allowlist_function("fallout:: _win_delete_menu_bar")
        .allowlist_function("fallout::_find_first_letter")
        .allowlist_function("fallout::_win_width_needed")
        .allowlist_function("fallout::_win_input_str")
        .allowlist_function("fallout::process_pull_down")
        .allowlist_function("fallout::_GNW_process_menu")
        .allowlist_function("fallout::win_get_num_i")
        .allowlist_function("fallout::_calc_max_field_chars_wcursor")
        .allowlist_function("fallout::_GNW_intr_init")
        .allowlist_function("fallout::_GNW_intr_exit")
        .allowlist_function("fallout::win_timed_msg");
    write_bindings("window_manager_private_h", &builder, out_path.clone());
    return builder;
    // extern char gProgramWindowTitle[256];
}

fn add_window_manager_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/window_manager.h")
        .allowlist_type("fallout::WindowManagerErr")
        .allowlist_type("fallout::WindowFlags")
        .allowlist_type("fallout::ButtonFlags")
        .allowlist_type("fallout::MenuPulldown")
        .allowlist_type("fallout::MenuBar")
        .allowlist_type("fallout::Window")
        .allowlist_type("fallout::Button")
        .allowlist_type("fallout::ButtonGroup")
        .allowlist_function("fallout::windowManagerInit")
        .allowlist_function("fallout::windowManagerExit")
        .allowlist_function("fallout::windowCreate")
        .allowlist_function("fallout::windowDestroy")
        .allowlist_function("fallout::windowDrawBorder")
        .allowlist_function("fallout::windowDrawText")
        .allowlist_function("fallout::_win_text")
        .allowlist_function("fallout::windowDrawLine")
        .allowlist_function("fallout::windowDrawRect")
        .allowlist_function("fallout::windowFill")
        .allowlist_function("fallout::windowShow")
        .allowlist_function("fallout::windowHide")
        .allowlist_function("fallout::windowRefresh")
        .allowlist_function("fallout::windowRefreshRect")
        .allowlist_function("fallout::_GNW_win_refresh")
        .allowlist_function("fallout::windowRefreshAll")
        .allowlist_function("fallout::_win_get_mouse_buf")
        .allowlist_function("fallout::windowGetWindow")
        .allowlist_function("fallout::windowGetBuffer")
        .allowlist_function("fallout::windowGetAtPoint")
        .allowlist_function("fallout::windowGetWidth")
        .allowlist_function("fallout::windowGetHeight")
        .allowlist_function("fallout::windowGetRect")
        .allowlist_function("fallout::_win_check_all_buttons")
        .allowlist_function("fallout::_GNW_check_menu_bars")
        .allowlist_function("fallout::programWindowSetTitle")
        .allowlist_function("fallout::showMesageBox")
        .allowlist_function("fallout::buttonCreate")
        .allowlist_function("fallout::_win_register_text_button")
        .allowlist_function("fallout::_win_register_button_disable")
        .allowlist_function("fallout::_win_register_button_image")
        .allowlist_function("fallout::buttonSetMouseCallbacks")
        .allowlist_function("fallout::buttonSetRightMouseCallbacks")
        .allowlist_function("fallout::buttonSetCallbacks")
        .allowlist_function("fallout::buttonSetMask")
        .allowlist_function("fallout::_win_button_down")
        .allowlist_function("fallout::buttonGetWindowId")
        .allowlist_function("fallout::_win_last_button_winID")
        .allowlist_function("fallout::buttonDestroy")
        .allowlist_function("fallout::buttonEnable")
        .allowlist_function("fallout::buttonDisable")
        .allowlist_function("fallout::_win_set_button_rest_state")
        .allowlist_function("fallout::_win_group_radio_buttons")
        .allowlist_function("fallout::_win_button_press_and_release");
    write_bindings("window_manager_h", &builder, out_path.clone());
    return builder;
    // extern bool gWindowSystemInitialized;
    // extern int _GNW_wcolor[6];
}

fn add_window_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/window.h")
        .allowlist_type("fallout::TextAlignment")
        .allowlist_type("fallout::ManagedButtonMouseEvent")
        .allowlist_type("fallout::ManagedButtonRightMouseEvent")
        .allowlist_function("fallout::windowGetFont")
        .allowlist_function("fallout::windowSetFont")
        .allowlist_function("fallout::windowResetTextAttributes")
        .allowlist_function("fallout::windowGetTextFlags")
        .allowlist_function("fallout::windowSetTextFlags")
        .allowlist_function("fallout::windowGetHighlightColor")
        .allowlist_function("fallout::windowSetTextColor")
        .allowlist_function("fallout::windowSetHighlightColor")
        .allowlist_function("fallout::_checkRegion")
        .allowlist_function("fallout::_windowCheckRegion")
        .allowlist_function("fallout::_windowRefreshRegions")
        .allowlist_function("fallout::_checkAllRegions")
        .allowlist_function("fallout::_windowAddInputFunc")
        .allowlist_function("fallout::_doRegionRightFunc")
        .allowlist_function("fallout::_doRegionFunc")
        .allowlist_function("fallout::_windowActivateRegion")
        .allowlist_function("fallout::_getInput")
        .allowlist_function("fallout::_doButtonOn")
        .allowlist_function("fallout::sub_4B6F68")
        .allowlist_function("fallout::_doButtonOff")
        .allowlist_function("fallout::_doButtonPress")
        .allowlist_function("fallout::_doButtonRelease")
        .allowlist_function("fallout::_doRightButtonPress")
        .allowlist_function("fallout::sub_4B704C")
        .allowlist_function("fallout::_doRightButtonRelease")
        .allowlist_function("fallout::_setButtonGFX")
        .allowlist_function("fallout::_windowWidth")
        .allowlist_function("fallout::_windowHeight")
        .allowlist_function("fallout::_windowDraw")
        .allowlist_function("fallout::_deleteWindow")
        .allowlist_function("fallout::sub_4B7AC4")
        .allowlist_function("fallout::sub_4B7E7C")
        .allowlist_function("fallout::_createWindow")
        .allowlist_function("fallout::_windowOutput")
        .allowlist_function("fallout::_windowGotoXY")
        .allowlist_function("fallout::_selectWindowID")
        .allowlist_function("fallout::_selectWindow")
        .allowlist_function("fallout::_windowGetBuffer")
        .allowlist_function("fallout::_pushWindow")
        .allowlist_function("fallout::_popWindow")
        .allowlist_function("fallout::_windowPrintBuf")
        .allowlist_function("fallout::_windowWordWrap")
        .allowlist_function("fallout::_windowFreeWordList")
        .allowlist_function("fallout::_windowWrapLineWithSpacing")
        .allowlist_function("fallout::_windowWrapLine")
        .allowlist_function("fallout::_windowPrintRect")
        .allowlist_function("fallout::_windowFormatMessage")
        .allowlist_function("fallout::_windowPrint")
        .allowlist_function("fallout::_displayInWindow")
        .allowlist_function("fallout::_displayFile")
        .allowlist_function("fallout::_displayFileRaw")
        .allowlist_function("fallout::_windowDisplay")
        .allowlist_function("fallout::_windowDisplayBuf")
        .allowlist_function("fallout::_windowGetXres")
        .allowlist_function("fallout::_windowGetYres")
        .allowlist_function("fallout::_removeProgramReferences_3")
        .allowlist_function("fallout::_initWindow")
        .allowlist_function("fallout::_windowClose")
        .allowlist_function("fallout::_windowDeleteButton")
        .allowlist_function("fallout::_windowSetButtonFlag")
        .allowlist_function("fallout::_windowAddButton")
        .allowlist_function("fallout::_windowAddButtonGfx")
        .allowlist_function("fallout::_windowAddButtonProc")
        .allowlist_function("fallout::_windowAddButtonRightProc")
        .allowlist_function("fallout::_windowAddButtonCfunc")
        .allowlist_function("fallout::_windowAddButtonRightCfunc")
        .allowlist_function("fallout::_windowAddButtonText")
        .allowlist_function("fallout::_windowAddButtonTextWithOffsets")
        .allowlist_function("fallout::_windowFill")
        .allowlist_function("fallout::_windowFillRect")
        .allowlist_function("fallout::_windowEndRegion")
        .allowlist_function("fallout::_windowCheckRegionExists")
        .allowlist_function("fallout::_windowStartRegion")
        .allowlist_function("fallout::_windowAddRegionPoint")
        .allowlist_function("fallout::_windowAddRegionProc")
        .allowlist_function("fallout::_windowAddRegionRightProc")
        .allowlist_function("fallout::_windowSetRegionFlag")
        .allowlist_function("fallout::_windowAddRegionName")
        .allowlist_function("fallout::_windowDeleteRegion")
        .allowlist_function("fallout::_updateWindows")
        .allowlist_function("fallout::_windowMoviePlaying")
        .allowlist_function("fallout::_windowSetMovieFlags")
        .allowlist_function("fallout::_windowPlayMovie")
        .allowlist_function("fallout::_windowPlayMovieRect")
        .allowlist_function("fallout::_windowStopMovie")
        .allowlist_function("fallout::_drawScaled")
        .allowlist_function("fallout::_drawScaledBuf")
        .allowlist_function("fallout::_alphaBltBuf")
        .allowlist_function("fallout::_fillBuf3x3");
    write_bindings("window_h", &builder, out_path.clone());
    return builder;
}

fn add_text_font_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/text_font.h")
        .allowlist_type("fallout::FontManager")
        .allowlist_function("fallout::textFontsInit")
        .allowlist_function("fallout::textFontsExit")
        .allowlist_function("fallout::textFontLoad")
        .allowlist_function("fallout::fontManagerAdd")
        .allowlist_function("fallout::fontGetCurrent")
        .allowlist_function("fallout::fontSetCurrent");
    write_bindings("text_font_h", &builder, out_path.clone());
    return builder;
    // extern FontManager gTextFontManager;
    // extern int gCurrentFont;
    // extern int gFontManagersCount;
    // extern FontManagerDrawTextProc* fontDrawText;
    // extern FontManagerGetLineHeightProc* fontGetLineHeight;
    // extern FontManagerGetStringWidthProc* fontGetStringWidth;
    // extern FontManagerGetCharacterWidthProc* fontGetCharacterWidth;
    // extern FontManagerGetMonospacedStringWidthProc* fontGetMonospacedStringWidth;
    // extern FontManagerGetLetterSpacingProc* fontGetLetterSpacing;
    // extern FontManagerGetBufferSizeProc* fontGetBufferSize;
    // extern FontManagerGetMonospacedCharacterWidth* fontGetMonospacedCharacterWidth;
}

fn add_svga_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/svga.h")
        .allowlist_function("fallout::_init_mode_320_200")
        .allowlist_function("fallout::_init_mode_320_400")
        .allowlist_function("fallout::_init_mode_640_480_16")
        .allowlist_function("fallout::_init_mode_640_480")
        .allowlist_function("fallout::_init_mode_640_400")
        .allowlist_function("fallout::_init_mode_800_600")
        .allowlist_function("fallout::_init_mode_1024_768")
        .allowlist_function("fallout::_init_mode_1280_1024")
        .allowlist_function("fallout::_get_start_mode_")
        .allowlist_function("fallout::_zero_vid_mem")
        .allowlist_function("fallout::_GNW95_init_mode_ex")
        .allowlist_function("fallout::_init_vesa_mode")
        .allowlist_function("fallout::_GNW95_init_window")
        .allowlist_function("fallout::directDrawInit")
        .allowlist_function("fallout::directDrawFree")
        .allowlist_function("fallout::directDrawSetPaletteInRange")
        .allowlist_function("fallout::directDrawSetPalette")
        .allowlist_function("fallout::directDrawGetPalette")
        .allowlist_function("fallout::_GNW95_ShowRect")
        .allowlist_function("fallout::_GNW95_zero_vid_mem")
        .allowlist_function("fallout::screenGetWidth")
        .allowlist_function("fallout::screenGetHeight")
        .allowlist_function("fallout::screenGetVisibleHeight")
        .allowlist_function("fallout::handleWindowSizeChanged")
        .allowlist_function("fallout::renderPresent");
    write_bindings("svga_h", &builder, out_path.clone());
    return builder;
}

fn add_sfall_global_scripts_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/sfall_global_scripts.h")
        .allowlist_function("fallout::sfall_gl_scr_init")
        .allowlist_function("fallout::sfall_gl_scr_reset")
        .allowlist_function("fallout::sfall_gl_scr_exit")
        .allowlist_function("fallout::sfall_gl_scr_exec_start_proc")
        .allowlist_function("fallout::sfall_gl_scr_remove_all")
        .allowlist_function("fallout::sfall_gl_scr_exec_map_update_scripts")
        .allowlist_function("fallout::sfall_gl_scr_process_main")
        .allowlist_function("fallout::sfall_gl_scr_process_input")
        .allowlist_function("fallout::sfall_gl_scr_process_worldmap")
        .allowlist_function("fallout::sfall_gl_scr_set_repeat")
        .allowlist_function("fallout::sfall_gl_scr_set_type")
        .allowlist_function("fallout::sfall_gl_scr_is_loaded")
        .allowlist_function("fallout::sfall_gl_scr_update");
    write_bindings("sfall_global_scripts_h", &builder, out_path.clone());
    return builder;
}

fn add_sfall_config_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/sfall_config.h")
        .allowlist_function("fallout::sfallConfigInit")
        .allowlist_function("fallout::sfallConfigExit");
    write_bindings("sfall_config_h", &builder, out_path.clone());
    return builder;
    // extern bool gSfallConfigInitialized;
    // extern Config gSfallConfig;
}

fn add_settings_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/settings.h")
        .allowlist_type("fallout::SystemSettings")
        .allowlist_type("fallout::PreferencesSettings")
        .allowlist_type("fallout::SoundSettings")
        .allowlist_type("fallout::DebugSettings")
        .allowlist_type("fallout::MapperSettings")
        .allowlist_type("fallout::Settings")
        .allowlist_function("fallout::settingsInit")
        .allowlist_function("fallout::settingsSave")
        .allowlist_function("fallout::settingsExit");
    write_bindings("settings_h", &builder, out_path.clone());
    return builder;

    // extern Settings settings;
}

fn add_selfrun_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/selfrun.h")
        .allowlist_type("fallout::SelfrunState")
        .allowlist_type("fallout::SelfrunData")
        .allowlist_function("fallout::selfrunInitFileList")
        .allowlist_function("fallout::selfrunFreeFileList")
        .allowlist_function("fallout::selfrunPreparePlayback")
        .allowlist_function("fallout::selfrunPlaybackLoop")
        .allowlist_function("fallout::selfrunPrepareRecording")
        .allowlist_function("fallout::selfrunRecordingLoop")
        .allowlist_function("fallout::selfrunPlaybackCompleted")
        .allowlist_function("fallout::selfrunReadData")
        .allowlist_function("fallout::selfrunWriteData");
    write_bindings("selfrun_h", &builder, out_path.clone());
    return builder;
    // extern int gSelfrunState;
}

fn add_script_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/scripts.h")
        .allowlist_type("fallout::ScriptRequests")
        .allowlist_type("fallout::ScriptType")
        .allowlist_type("fallout::ScriptProc")
        .allowlist_type("fallout::Script")
        .allowlist_function("fallout::gameTimeGetTime")
        .allowlist_function("fallout::gameTimeGetDate")
        .allowlist_function("fallout::gameTimeGetHour")
        .allowlist_function("fallout::gameTimeGetTimeString")
        .allowlist_function("fallout::gameTimeAddTicks")
        .allowlist_function("fallout::gameTimeAddSeconds")
        .allowlist_function("fallout::gameTimeSetTime")
        .allowlist_function("fallout::gameTimeScheduleUpdateEvent")
        .allowlist_function("fallout::gameTimeEventProcess")
        .allowlist_function("fallout::_scriptsCheckGameEvents")
        .allowlist_function("fallout::mapUpdateEventProcess")
        .allowlist_function("fallout::scriptsNewObjectId")
        .allowlist_function("fallout::scriptGetSid")
        .allowlist_function("fallout::scriptGetSelf")
        .allowlist_function("fallout::scriptSetObjects")
        .allowlist_function("fallout::scriptSetFixedParam")
        .allowlist_function("fallout::scriptSetActionBeingUsed")
        .allowlist_function("fallout::_scrSetQueueTestVals")
        .allowlist_function("fallout::_scrQueueRemoveFixed")
        .allowlist_function("fallout::scriptAddTimerEvent")
        .allowlist_function("fallout::scriptEventWrite")
        .allowlist_function("fallout::scriptEventRead")
        .allowlist_function("fallout::scriptEventProcess")
        .allowlist_function("fallout::_scripts_clear_combat_requests")
        .allowlist_function("fallout::scriptsHandleRequests")
        .allowlist_function("fallout::_scripts_check_state_in_combat")
        .allowlist_function("fallout::scriptsRequestCombat")
        .allowlist_function("fallout::_scripts_request_combat_locked")
        .allowlist_function("fallout::scripts_request_townmap")
        .allowlist_function("fallout::scriptsRequestWorldMap")
        .allowlist_function("fallout::scriptsRequestElevator")
        .allowlist_function("fallout::scriptsRequestExplosion")
        .allowlist_function("fallout::scriptsRequestDialog")
        .allowlist_function("fallout::scriptsRequestEndgame")
        .allowlist_function("fallout::scriptsRequestLooting")
        .allowlist_function("fallout::scriptsRequestStealing")
        .allowlist_function("fallout::_script_make_path")
        .allowlist_function("fallout::scriptExecProc")
        .allowlist_function("fallout::scriptHasProc")
        .allowlist_function("fallout::_scr_find_str_run_info")
        .allowlist_function("fallout::scriptsSetDudeScript")
        .allowlist_function("fallout::scriptsClearDudeScript")
        .allowlist_function("fallout::scriptsInit")
        .allowlist_function("fallout::_scr_reset")
        .allowlist_function("fallout::_scr_game_init")
        .allowlist_function("fallout::scriptsReset")
        .allowlist_function("fallout::scriptsExit")
        .allowlist_function("fallout::_scr_message_free")
        .allowlist_function("fallout::_scr_game_exit")
        .allowlist_function("fallout::scriptsEnable")
        .allowlist_function("fallout::scriptsDisable")
        .allowlist_function("fallout::_scr_enable_critters")
        .allowlist_function("fallout::_scr_disable_critters")
        .allowlist_function("fallout::scriptsSaveGameGlobalVars")
        .allowlist_function("fallout::scriptsLoadGameGlobalVars")
        .allowlist_function("fallout::scriptsSkipGameGlobalVars")
        .allowlist_function("fallout::scriptSaveAll")
        .allowlist_function("fallout::scriptLoadAll")
        .allowlist_function("fallout::scriptGetScript")
        .allowlist_function("fallout::scriptAdd")
        .allowlist_function("fallout::scriptRemove")
        .allowlist_function("fallout::_scr_remove_all")
        .allowlist_function("fallout::_scr_remove_all_force")
        .allowlist_function("fallout::scriptGetFirstSpatialScript")
        .allowlist_function("fallout::scriptGetNextSpatialScript")
        .allowlist_function("fallout::_scr_spatials_enable")
        .allowlist_function("fallout::_scr_spatials_disable")
        .allowlist_function("fallout::scriptsExecSpatialProc")
        .allowlist_function("fallout::scriptsExecStartProc")
        .allowlist_function("fallout::scriptsExecMapEnterProc")
        .allowlist_function("fallout::scriptsExecMapUpdateProc")
        .allowlist_function("fallout::scriptsExecMapUpdateScripts")
        .allowlist_function("fallout::scriptsExecMapExitProc")
        .allowlist_function("fallout::_scr_get_msg_str")
        .allowlist_function("fallout::_scr_get_msg_str_speech")
        .allowlist_function("fallout::scriptGetLocalVar")
        .allowlist_function("fallout::scriptSetLocalVar")
        .allowlist_function("fallout::_scr_end_combat")
        .allowlist_function("fallout::_scr_explode_scenery");
    write_bindings("scripts_h", &builder, out_path.clone());
    return builder;

    // extern const char* gScriptProcNames[SCRIPT_PROC_COUNT];
}

fn add_random_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/random.h")
        .allowlist_type("fallout::Roll")
        .allowlist_function("fallout::randomInit")
        .allowlist_function("fallout::randomReset")
        .allowlist_function("fallout::randomExit")
        .allowlist_function("fallout::randomSave")
        .allowlist_function("fallout::randomLoad")
        .allowlist_function("fallout::randomRoll")
        .allowlist_function("fallout::randomBetween")
        .allowlist_function("fallout::randomSeedPrerandom");
    write_bindings("random_h", &builder, out_path.clone());
    return builder;
}

fn add_proto_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/proto.h")
        .allowlist_type("fallout::ItemDataMember")
        .allowlist_type("fallout::CritterDataMember")
        .allowlist_type("fallout::SceneryDataMember")
        .allowlist_type("fallout::WallDataMember")
        .allowlist_type("fallout::MiscDataMember")
        .allowlist_type("fallout::ProtoDataMemberType")
        .allowlist_type("fallout::ProtoDataMemberValue")
        .allowlist_type("fallout::PrototypeMessage")
        .allowlist_function("fallout::proto_make_path")
        .allowlist_function("fallout::_proto_list_str")
        .allowlist_function("fallout::proto_size")
        .allowlist_function("fallout::_proto_action_can_use")
        .allowlist_function("fallout::_proto_action_can_use_on")
        .allowlist_function("fallout::_proto_action_can_talk_to")
        .allowlist_function("fallout::_proto_action_can_pickup")
        .allowlist_function("fallout::protoGetMessage")
        .allowlist_function("fallout::protoGetName")
        .allowlist_function("fallout::protoGetDescription")
        .allowlist_function("fallout::proto_item_init")
        .allowlist_function("fallout::proto_item_subdata_init")
        .allowlist_function("fallout::proto_critter_init")
        .allowlist_function("fallout::objectDataReset")
        .allowlist_function("fallout::objectDataRead")
        .allowlist_function("fallout::objectDataWrite")
        .allowlist_function("fallout::_proto_update_init")
        .allowlist_function("fallout::_proto_dude_update_gender")
        .allowlist_function("fallout::_proto_dude_init")
        .allowlist_function("fallout::proto_scenery_init")
        .allowlist_function("fallout::proto_scenery_subdata_init")
        .allowlist_function("fallout::proto_wall_init")
        .allowlist_function("fallout::proto_tile_init")
        .allowlist_function("fallout::proto_misc_init")
        .allowlist_function("fallout::proto_copy_proto")
        .allowlist_function("fallout::proto_is_subtype")
        .allowlist_function("fallout::protoGetDataMember")
        .allowlist_function("fallout::protoInit")
        .allowlist_function("fallout::protoReset")
        .allowlist_function("fallout::protoExit")
        .allowlist_function("fallout::_proto_save_pid")
        .allowlist_function("fallout::proto_new")
        .allowlist_function("fallout::_proto_remove_all")
        .allowlist_function("fallout::protoGetProto")
        .allowlist_function("fallout::_ResetPlayer")
        .allowlist_function("fallout::proto_max_id")
        .allowlist_function("fallout::isExitGridPid");
    write_bindings("proto_h", &builder, out_path.clone());
    return builder;
    // extern char _cd_path_base[COMPAT_MAX_PATH];
    // extern MessageList gProtoMessageList;
    // extern char* _proto_none_str;
}

fn add_preferences_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/preferences.h")
        .allowlist_function("fallout::preferencesInit")
        .allowlist_function("fallout::doPreferences")
        .allowlist_function("fallout::preferencesSave")
        .allowlist_function("fallout::preferencesLoad")
        .allowlist_function("fallout::brightnessIncrease")
        .allowlist_function("fallout::brightnessDecrease");
    write_bindings("preferences_h", &builder, out_path.clone());
    return builder;
}

fn add_platform_compat_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/platform_compat.h")
        .allowlist_function("fallout::compat_stricmp")
        .allowlist_function("fallout::compat_strnicmp")
        .allowlist_function("fallout::compat_strupr")
        .allowlist_function("fallout::compat_strlwr")
        .allowlist_function("fallout::compat_itoa")
        .allowlist_function("fallout::compat_splitpath")
        .allowlist_function("fallout::compat_makepath")
        .allowlist_function("fallout::compat_tell")
        .allowlist_function("fallout::compat_filelength")
        .allowlist_function("fallout::compat_mkdir")
        .allowlist_function("fallout::compat_timeGetTime")
        .allowlist_function("fallout::compat_fopen")
        .allowlist_function("fallout::compat_gzopen")
        .allowlist_function("fallout::compat_fgets")
        .allowlist_function("fallout::compat_gzgets")
        .allowlist_function("fallout::compat_remove")
        .allowlist_function("fallout::compat_rename")
        .allowlist_function("fallout::compat_windows_path_to_native")
        .allowlist_function("fallout::compat_resolve_path")
        .allowlist_function("fallout::compat_access")
        .allowlist_function("fallout::compat_strdup")
        .allowlist_function("fallout::getFileSize");
    write_bindings("platform_compat_h", &builder, out_path.clone());
    return builder;
}

fn add_palette_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/palette.h")
        .allowlist_function("fallout::paletteInit")
        .allowlist_function("fallout::paletteReset")
        .allowlist_function("fallout::paletteExit")
        .allowlist_function("fallout::paletteFadeTo")
        .allowlist_function("fallout::paletteSetEntries")
        .allowlist_function("fallout::paletteSetEntriesInRange");
    write_bindings("palette_h", &builder, out_path.clone());
    return builder;

    // extern unsigned char gPaletteWhite[256 * 3];
    // extern unsigned char gPaletteBlack[256 * 3];
}

fn add_object_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/mainmenu.h")
        .allowlist_type("fallout::ObjectWithFlags")
        .allowlist_function("fallout::objectsInit")
        .allowlist_function("fallout::objectsReset")
        .allowlist_function("fallout::objectsExit")
        .allowlist_function("fallout::objectRead")
        .allowlist_function("fallout::objectLoadAll")
        .allowlist_function("fallout::objectSaveAll")
        .allowlist_function("fallout::_obj_render_pre_roof")
        .allowlist_function("fallout::_obj_render_post_roof")
        .allowlist_function("fallout::objectCreateWithFidPid")
        .allowlist_function("fallout::objectCreateWithPid")
        .allowlist_function("fallout::_obj_copy")
        .allowlist_function("fallout::_obj_connect")
        .allowlist_function("fallout::_obj_disconnect")
        .allowlist_function("fallout::_obj_offset")
        .allowlist_function("fallout::_obj_move")
        .allowlist_function("fallout::objectSetLocation")
        .allowlist_function("fallout::_obj_reset_roof")
        .allowlist_function("fallout::objectSetFid")
        .allowlist_function("fallout::objectSetFrame")
        .allowlist_function("fallout::objectSetNextFrame")
        .allowlist_function("fallout::objectSetPrevFrame")
        .allowlist_function("fallout::objectSetRotation")
        .allowlist_function("fallout::objectRotateClockwise")
        .allowlist_function("fallout::objectRotateCounterClockwise")
        .allowlist_function("fallout::_obj_rebuild_all_light")
        .allowlist_function("fallout::objectSetLight")
        .allowlist_function("fallout::objectGetLightIntensity")
        .allowlist_function("fallout::_obj_turn_on_light")
        .allowlist_function("fallout::_obj_turn_off_light")
        .allowlist_function("fallout::objectShow")
        .allowlist_function("fallout::objectHide")
        .allowlist_function("fallout::objectEnableOutline")
        .allowlist_function("fallout::objectDisableOutline")
        .allowlist_function("fallout::_obj_toggle_flat")
        .allowlist_function("fallout::objectDestroy")
        .allowlist_function("fallout::_obj_inven_free")
        .allowlist_function("fallout::_obj_action_can_use")
        .allowlist_function("fallout::_obj_action_can_talk_to")
        .allowlist_function("fallout::_obj_portal_is_walk_thru")
        .allowlist_function("fallout::objectFindById")
        .allowlist_function("fallout::objectGetOwner")
        .allowlist_function("fallout::_obj_remove_all")
        .allowlist_function("fallout::objectFindFirst")
        .allowlist_function("fallout::objectFindNext")
        .allowlist_function("fallout::objectFindFirstAtElevation")
        .allowlist_function("fallout::objectFindNextAtElevation")
        .allowlist_function("fallout::objectFindFirstAtLocation")
        .allowlist_function("fallout::objectFindNextAtLocation")
        .allowlist_function("fallout::objectGetRect")
        .allowlist_function("fallout::_obj_occupied")
        .allowlist_function("fallout::_obj_blocking_at")
        .allowlist_function("fallout::_obj_shoot_blocking_at")
        .allowlist_function("fallout::_obj_ai_blocking_at")
        .allowlist_function("fallout::_obj_scroll_blocking_at")
        .allowlist_function("fallout::_obj_sight_blocking_at")
        .allowlist_function("fallout::objectGetDistanceBetween")
        .allowlist_function("fallout::objectGetDistanceBetweenTiles")
        .allowlist_function("fallout::objectListCreate")
        .allowlist_function("fallout::objectListFree")
        .allowlist_function("fallout::_translucent_trans_buf_to_buf")
        .allowlist_function("fallout::_dark_trans_buf_to_buf")
        .allowlist_function("fallout::_dark_translucent_trans_buf_to_buf")
        .allowlist_function("fallout::_intensity_mask_buf_to_buf")
        .allowlist_function("fallout::objectSetOutline")
        .allowlist_function("fallout::objectClearOutline")
        .allowlist_function("fallout::_obj_intersects_with")
        .allowlist_function("fallout::_obj_create_intersect_list")
        .allowlist_function("fallout::_obj_delete_intersect_list")
        .allowlist_function("fallout::obj_set_seen")
        .allowlist_function("fallout::_obj_clear_seen")
        .allowlist_function("fallout::_obj_process_seen")
        .allowlist_function("fallout::objectGetName")
        .allowlist_function("fallout::objectGetDescription")
        .allowlist_function("fallout::_obj_preload_art_cache")
        .allowlist_function("fallout::_obj_save_dude")
        .allowlist_function("fallout::_obj_load_dude")
        .allowlist_function("fallout::_obj_fix_violence_settings")
        .allowlist_function("fallout::objectTypedFindById")
        .allowlist_function("fallout::isExitGridAt");
    write_bindings("mainmenu_h", &builder, out_path.clone());
    return builder;
    // extern unsigned char* _wallBlendTable;
    // extern Object* _moveBlockObj;
    // extern unsigned char _commonGrayTable[256];
    // extern Object* gEgg;
    // extern Object* gDude;
}

fn add_mainmenu_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/mainmenu.h")
        .allowlist_type("fallout::MainMenuOption")
        .allowlist_function("fallout::mainMenuWindowInit")
        .allowlist_function("fallout::mainMenuWindowFree")
        .allowlist_function("fallout::mainMenuWindowHide")
        .allowlist_function("fallout::mainMenuWindowUnhide")
        .allowlist_function("fallout::_main_menu_is_enabled")
        .allowlist_function("fallout::mainMenuWindowHandleEvents");
    write_bindings("mainmenu_h", &builder, out_path.clone());
    return builder;
}

fn add_loadsave_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/loadsave.h")
        .allowlist_type("fallout::LoadSaveMode")
        .allowlist_function("fallout::_InitLoadSave")
        .allowlist_function("fallout::_ResetLoadSave")
        .allowlist_function("fallout::lsgSaveGame")
        .allowlist_function("fallout::lsgLoadGame")
        .allowlist_function("fallout::_isLoadingGame")
        .allowlist_function("fallout::lsgInit")
        .allowlist_function("fallout::MapDirErase")
        .allowlist_function("fallout::_MapDirEraseFile_");
    write_bindings("loadsave_h", &builder, out_path.clone());
    return builder;
}

fn add_kb_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/kb.h")
        .allowlist_type("fallout::Key")
        .allowlist_type("fallout::KeyboardLayout")
        .allowlist_type("fallout::KeyboardData")
        .allowlist_function("fallout::keyboardInit")
        .allowlist_function("fallout::keyboardFree")
        .allowlist_function("fallout::keyboardReset")
        .allowlist_function("fallout::_kb_getch")
        .allowlist_function("fallout::keyboardDisable")
        .allowlist_function("fallout::keyboardEnable")
        .allowlist_function("fallout::keyboardIsDisabled")
        .allowlist_function("fallout::keyboardSetLayout")
        .allowlist_function("fallout::keyboardGetLayout")
        .allowlist_function("fallout::_kb_simulate_key");
    write_bindings("kb_h", &builder, out_path.clone());
    return builder;
    // extern unsigned char gPressedPhysicalKeys[SDL_NUM_SCANCODES];
    // extern int gKeyboardLayout;
    // extern unsigned char gPressedPhysicalKeysCount;
}

fn add_input_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/input.h")
        .allowlist_type("fallout::GameMovieFlags")
        .allowlist_type("fallout::GameMovie")
        .allowlist_function("fallout::inputInit")
        .allowlist_function("fallout::inputExit")
        .allowlist_function("fallout::inputGetInput")
        .allowlist_function("fallout::_process_bk")
        .allowlist_function("fallout::enqueueInputEvent")
        .allowlist_function("fallout::inputEventQueueReset")
        .allowlist_function("fallout::tickersExecute")
        .allowlist_function("fallout::tickersAdd")
        .allowlist_function("fallout::tickersRemove")
        .allowlist_function("fallout::tickersEnable")
        .allowlist_function("fallout::tickersDisable")
        .allowlist_function("fallout::pauseHandlerConfigure")
        .allowlist_function("fallout::takeScreenshot")
        .allowlist_function("fallout::screenshotHandlerDefaultImpl")
        .allowlist_function("fallout::screenshotHandlerConfigure")
        .allowlist_function("fallout::getTicks")
        .allowlist_function("fallout::inputPauseForTocks")
        .allowlist_function("fallout::inputBlockForTocks")
        .allowlist_function("fallout::getTicksSince")
        .allowlist_function("fallout::getTicksBetween")
        .allowlist_function("fallout::_get_bk_time")
        .allowlist_function("fallout::inputSetKeyboardKeyRepeatRate")
        .allowlist_function("fallout::inputGetKeyboardKeyRepeatRate")
        .allowlist_function("fallout::inputSetKeyboardKeyRepeatDelay")
        .allowlist_function("fallout::inputGetKeyboardKeyRepeatDelay")
        .allowlist_function("fallout::inputSetFocusFunc")
        .allowlist_function("fallout::inputGetFocusFunc")
        .allowlist_function("fallout::inputSetIdleFunc")
        .allowlist_function("fallout::inputGetIdleFunc")
        .allowlist_function("fallout::_GNW95_input_init")
        .allowlist_function("fallout::_GNW95_process_message")
        .allowlist_function("fallout::_GNW95_clear_time_stamps")
        .allowlist_function("fallout::_GNW95_lost_focus")
        .allowlist_function("fallout::beginTextInput")
        .allowlist_function("fallout::endTextInput");
    write_bindings("input_h", &builder, out_path.clone());
    return builder;
}

fn add_game_sound_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/game_sound.h")
        .allowlist_type("fallout::WeaponSoundEffect")
        .allowlist_type("fallout::ScenerySoundEffect")
        .allowlist_type("fallout::CharacterSoundEffect")
        .allowlist_type("fallout::SoundEndCallback")
        .allowlist_type("fallout::Sound")
        .allowlist_type("fallout::Object")
        .allowlist_function("fallout::gameSoundInit")
        .allowlist_function("fallout::gameSoundReset")
        .allowlist_function("fallout::gameSoundExit")
        .allowlist_function("fallout::gameSoundSetMasterVolume")
        .allowlist_function("fallout::gameSoundGetMasterVolume")
        .allowlist_function("fallout::soundEffectsSetVolume")
        .allowlist_function("fallout::backgroundSoundIsEnabled")
        .allowlist_function("fallout::backgroundSoundSetVolume")
        .allowlist_function("fallout::backgroundSoundGetVolume")
        .allowlist_function("fallout::_gsound_background_volume_get_set")
        .allowlist_function("fallout::backgroundSoundSetEndCallback")
        .allowlist_function("fallout::backgroundSoundLoad")
        .allowlist_function("fallout::_gsound_background_play_level_music")
        .allowlist_function("fallout::backgroundSoundDelete")
        .allowlist_function("fallout::backgroundSoundRestart")
        .allowlist_function("fallout::backgroundSoundPause")
        .allowlist_function("fallout::backgroundSoundResume")
        .allowlist_function("fallout::speechIsEnabled")
        .allowlist_function("fallout::speechSetVolume")
        .allowlist_function("fallout::speechGetVolume")
        .allowlist_function("fallout::speechSetEndCallback")
        .allowlist_function("fallout::speechGetDuration")
        .allowlist_function("fallout::speechLoad")
        .allowlist_function("fallout::_gsound_speech_play_preloaded")
        .allowlist_function("fallout::speechDelete")
        .allowlist_function("fallout::_gsound_play_sfx_file_volume")
        .allowlist_function("fallout::soundEffectLoad")
        .allowlist_function("fallout::soundEffectLoadWithVolume")
        .allowlist_function("fallout::soundEffectDelete")
        .allowlist_function("fallout::_gsnd_anim_sound")
        .allowlist_function("fallout::soundEffectPlay")
        .allowlist_function("fallout::_gsound_compute_relative_volume")
        .allowlist_function("fallout::sfxBuildCharName")
        .allowlist_function("fallout::gameSoundBuildAmbientSoundEffectName")
        .allowlist_function("fallout::gameSoundBuildInterfaceName")
        .allowlist_function("fallout::sfxBuildWeaponName")
        .allowlist_function("fallout::sfxBuildSceneryName")
        .allowlist_function("fallout::sfxBuildOpenName")
        .allowlist_function("fallout::_gsound_red_butt_press")
        .allowlist_function("fallout::_gsound_red_butt_release")
        .allowlist_function("fallout::_gsound_toggle_butt_press_")
        .allowlist_function("fallout::_gsound_med_butt_press")
        .allowlist_function("fallout::_gsound_med_butt_release")
        .allowlist_function("fallout::_gsound_lrg_butt_press")
        .allowlist_function("fallout::_gsound_lrg_butt_release")
        .allowlist_function("fallout::soundPlayFile")
        .allowlist_function("fallout::_gsound_sfx_q_start")
        .allowlist_function("fallout::ambientSoundEffectEventProcess");
    write_bindings("game_sound_h", &builder, out_path.clone());
    return builder;

    // extern int gMusicVolume;
}

fn add_game_movie_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/game_movie.h")
        .allowlist_type("fallout::GameMovieFlags")
        .allowlist_type("fallout::GameMovie")
        .allowlist_function("fallout::gameMoviesInit")
        .allowlist_function("fallout::gameMoviesReset")
        .allowlist_function("fallout::gameMoviesLoad")
        .allowlist_function("fallout::gameMoviesSave")
        .allowlist_function("fallout::gameMoviePlay")
        .allowlist_function("fallout::gameMovieFadeOut")
        .allowlist_function("fallout::gameMovieIsSeen")
        .allowlist_function("fallout::gameMovieIsPlaying");
    write_bindings("game_movie_h", &builder, out_path.clone());
    return builder;
}

fn add_game_mouse_h_to_bindings(out_path: &PathBuf) -> bindgen::Builder {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        .header("fallout2-ce/src/game_mouse.h")
        .allowlist_type("fallout::GameMouseMode")
        .allowlist_type("fallout::GameMouseActionMenuItem")
        .allowlist_type("fallout::MouseCursorType")
        .allowlist_function("fallout::gameMouseInit")
        .allowlist_function("fallout::gameMouseReset")
        .allowlist_function("fallout::gameMouseExit")
        .allowlist_function("fallout::_gmouse_enable")
        .allowlist_function("fallout::_gmouse_disable")
        .allowlist_function("fallout::_gmouse_enable_scrolling")
        .allowlist_function("fallout::_gmouse_disable_scrolling")
        .allowlist_function("fallout::gmouse_scrolling_is_enabled")
        .allowlist_function("fallout::_gmouse_is_scrolling")
        .allowlist_function("fallout::gameMouseRefresh")
        .allowlist_function("fallout::_gmouse_handle_event")
        .allowlist_function("fallout::gameMouseSetCursor")
        .allowlist_function("fallout::gameMouseGetCursor")
        .allowlist_function("fallout::gmouse_set_mapper_mode")
        .allowlist_function("fallout::gameMouseSetMode")
        .allowlist_function("fallout::gameMouseGetMode")
        .allowlist_function("fallout::gameMouseCycleMode")
        .allowlist_function("fallout::_gmouse_3d_refresh")
        .allowlist_function("fallout::gameMouseResetBouncingCursorFid")
        .allowlist_function("fallout::gameMouseObjectsShow")
        .allowlist_function("fallout::gameMouseObjectsHide")
        .allowlist_function("fallout::gameMouseObjectsIsVisible")
        .allowlist_function("fallout::gameMouseRenderPrimaryAction")
        .allowlist_function("fallout::_gmouse_3d_pick_frame_hot")
        .allowlist_function("fallout::gameMouseRenderActionMenuItems")
        .allowlist_function("fallout::gameMouseHighlightActionMenuItemAtIndex")
        .allowlist_function("fallout::gameMouseLoadItemHighlight")
        .allowlist_function("fallout::_gmouse_remove_item_outline")
        .allowlist_function("fallout::gameMouseRefreshImmediately");
    write_bindings("game_mouse_h", &builder, out_path.clone());
    return builder;

    // extern bool _gmouse_clicked_on_edge;
    // extern Object* gGameMouseBouncingCursor;
    // extern Object* gGameMouseHexCursor;
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
