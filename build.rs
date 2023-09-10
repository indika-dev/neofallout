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
    add_game_mouse_h_to_bindings(&out_path);
    add_game_movie_h_to_bindings(&out_path);
    add_game_sound_h_to_bindings(&out_path);
    add_input_h_to_bindings(&out_path);
    // add_kb_h_to_bindings(&out_path);
    add_loadsave_h_to_bindings(&out_path);
    add_mainmenu_h_to_bindings(&out_path);
    add_object_h_to_bindings(&out_path);
    add_palette_h_to_bindings(&out_path);
    add_platform_compat_h_to_bindings(&out_path);
    add_preferences_h_to_bindings(&out_path);
    add_proto_h_to_bindings(&out_path);
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
