#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use ini::Ini;
use log::{debug, error, LevelFilter};
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Root},
    Config,
};
use std::{
    env,
    ffi::{c_char, CString},
    fs::File,
    u8,
};

use neofallout::{
    autorunMutexCreate, colorPaletteLoad, gameMoviePlay, lsgLoadGame, mainMenuWindowHandleEvents,
    mainMenuWindowHide, paletteFadeTo, screenGetHeight, screenGetWidth,
    sfall_gl_scr_exec_start_proc, windowCreate, windowDestroy, Dam, LoadSaveMode, MainMenuOption,
    MouseCursorType, ObjectFlags, WindowFlags, _map_exit, _map_init, autorunMutexClose,
    backgroundSoundDelete, creditsOpen, cursorIsHidden, doPreferences, endgameSetupDeathEnding,
    fileNameListInit, gameHandleKey, gameInitWithOptions, inputGetInput, mainMenuWindowFree,
    mapHandleTransition, mouseHideCursor, mouseShowCursor, objectHide, objectShow, renderPresent,
    scriptsDisable, scriptsEnable, scriptsHandleRequests, selfrunFreeFileList, selfrunInitFileList,
    sfall_gl_scr_process_main, wmMapMusicStart, SelfrunData, _proto_dude_init, _win_list_select,
    artCacheFlush, blitBufferToBuffer, colorCycleDisable, endgameDeathEndingGetFileName,
    inputEventQueueReset, keyboardReset, randomSeedPrerandom, selfrunPreparePlayback,
    selfrunPrepareRecording, selfrunRecordingLoop, windowGetBuffer, FrmImage,
    _gsound_background_play_level_music, _gsound_speech_play_preloaded, bufferFill, buildFid,
    characterSelectorOpen, colorCycleEnable, colorCycleEnabled, configGetInt, configGetString,
    fileClose, fileNameListFree, fileOpen, fileReadChar, gameExit, gameMouseSetCursor, gameReset,
    getTicks, inputBlockForTocks, inputPauseForTocks, mainMenuWindowInit, mainMenuWindowUnhide,
    mapLoadByName, mouseGetEvent, selfrunPlaybackLoop, speechDelete, speechLoad,
    speechSetEndCallback, windowRefresh, wordWrap, GameMovie, GameMovieFlags, Object, ObjectTypes,
};

const SELFRUN_RECORDING_FILE_NAME_LENGTH: i32 = 13;
const SELFRUN_MAP_FILE_NAME_LENGTH: i32 = 13;
/*
from sfall_config.h
*/
const SFALL_CONFIG_FILE_NAME: &str = "ddraw.ini";
const SFALL_CONFIG_MISC_KEY: &str = "Misc";
const SFALL_CONFIG_SCRIPTS_KEY: &str = "Scripts";
const SFALL_CONFIG_DUDE_NATIVE_LOOK_JUMPSUIT_MALE_KEY: &str = "MaleDefaultModel";
const SFALL_CONFIG_DUDE_NATIVE_LOOK_JUMPSUIT_FEMALE_KEY: &str = "FemaleDefaultModel";
const SFALL_CONFIG_DUDE_NATIVE_LOOK_TRIBAL_MALE_KEY: &str = "MaleStartModel";
const SFALL_CONFIG_DUDE_NATIVE_LOOK_TRIBAL_FEMALE_KEY: &str = "FemaleStartModel";
const SFALL_CONFIG_START_YEAR: &str = "StartYear";
const SFALL_CONFIG_START_MONTH: &str = "StartMonth";
const SFALL_CONFIG_START_DAY: &str = "StartDay";
const SFALL_CONFIG_MAIN_MENU_BIG_FONT_COLOR_KEY: &str = "MainMenuBigFontColour";
const SFALL_CONFIG_MAIN_MENU_CREDITS_OFFSET_X_KEY: &str = "MainMenuCreditsOffsetX";
const SFALL_CONFIG_MAIN_MENU_CREDITS_OFFSET_Y_KEY: &str = "MainMenuCreditsOffsetY";
const SFALL_CONFIG_MAIN_MENU_FONT_COLOR_KEY: &str = "MainMenuFontColour";
const SFALL_CONFIG_MAIN_MENU_OFFSET_X_KEY: &str = "MainMenuOffsetX";
const SFALL_CONFIG_MAIN_MENU_OFFSET_Y_KEY: &str = "MainMenuOffsetY";
const SFALL_CONFIG_SKIP_OPENING_MOVIES_KEY: &str = "SkipOpeningMovies";
const SFALL_CONFIG_STARTING_MAP_KEY: &str = "StartingMap";
const SFALL_CONFIG_KARMA_FRMS_KEY: &str = "KarmaFRMs";
const SFALL_CONFIG_KARMA_POINTS_KEY: &str = "KarmaPoints";
const SFALL_CONFIG_DISPLAY_KARMA_CHANGES_KEY: &str = "DisplayKarmaChanges";
const SFALL_CONFIG_OVERRIDE_CRITICALS_MODE_KEY: &str = "OverrideCriticalTable";
const SFALL_CONFIG_OVERRIDE_CRITICALS_FILE_KEY: &str = "OverrideCriticalFile";
const SFALL_CONFIG_REMOVE_CRITICALS_TIME_LIMITS_KEY: &str = "RemoveCriticalTimelimits";
const SFALL_CONFIG_BOOKS_FILE_KEY: &str = "BooksFile";
const SFALL_CONFIG_ELEVATORS_FILE_KEY: &str = "ElevatorsFile";
const SFALL_CONFIG_CONSOLE_OUTPUT_FILE_KEY: &str = "ConsoleOutputPath";
const SFALL_CONFIG_PREMADE_CHARACTERS_FILE_NAMES_KEY: &str = "PremadePaths";
const SFALL_CONFIG_PREMADE_CHARACTERS_FACE_FIDS_KEY: &str = "PremadeFIDs";
const SFALL_CONFIG_BURST_MOD_ENABLED_KEY: &str = "ComputeSprayMod";
const SFALL_CONFIG_BURST_MOD_CENTER_MULTIPLIER_KEY: &str = "ComputeSpray_CenterMult";
const SFALL_CONFIG_BURST_MOD_CENTER_DIVISOR_KEY: &str = "ComputeSpray_CenterDiv";
const SFALL_CONFIG_BURST_MOD_TARGET_MULTIPLIER_KEY: &str = "ComputeSpray_TargetMult";
const SFALL_CONFIG_BURST_MOD_TARGET_DIVISOR_KEY: &str = "ComputeSpray_TargetDiv";
const SFALL_CONFIG_DYNAMITE_MIN_DAMAGE_KEY: &str = "Dynamite_DmgMin";
const SFALL_CONFIG_DYNAMITE_MAX_DAMAGE_KEY: &str = "Dynamite_DmgMax";
const SFALL_CONFIG_PLASTIC_EXPLOSIVE_MIN_DAMAGE_KEY: &str = "PlasticExplosive_DmgMin";
const SFALL_CONFIG_PLASTIC_EXPLOSIVE_MAX_DAMAGE_KEY: &str = "PlasticExplosive_DmgMax";
const SFALL_CONFIG_EXPLOSION_EMITS_LIGHT_KEY: &str = "ExplosionsEmitLight";
const SFALL_CONFIG_MOVIE_TIMER_ARTIMER1: &str = "MovieTimer_artimer1";
const SFALL_CONFIG_MOVIE_TIMER_ARTIMER2: &str = "MovieTimer_artimer2";
const SFALL_CONFIG_MOVIE_TIMER_ARTIMER3: &str = "MovieTimer_artimer3";
const SFALL_CONFIG_MOVIE_TIMER_ARTIMER4: &str = "MovieTimer_artimer4";
const SFALL_CONFIG_CITY_REPUTATION_LIST_KEY: &str = "CityRepsList";
const SFALL_CONFIG_UNARMED_FILE_KEY: &str = "UnarmedFile";
const SFALL_CONFIG_DAMAGE_MOD_FORMULA_KEY: &str = "DamageFormula";
const SFALL_CONFIG_BONUS_HTH_DAMAGE_FIX_KEY: &str = "BonusHtHDamageFix";
const SFALL_CONFIG_DISPLAY_BONUS_DAMAGE_KEY: &str = "DisplayBonusDamage";
const SFALL_CONFIG_USE_LOCKPICK_FRM_KEY: &str = "Lockpick";
const SFALL_CONFIG_USE_STEAL_FRM_KEY: &str = "Steal";
const SFALL_CONFIG_USE_TRAPS_FRM_KEY: &str = "Traps";
const SFALL_CONFIG_USE_FIRST_AID_FRM_KEY: &str = "FirstAid";
const SFALL_CONFIG_USE_DOCTOR_FRM_KEY: &str = "Doctor";
const SFALL_CONFIG_USE_SCIENCE_FRM_KEY: &str = "Science";
const SFALL_CONFIG_USE_REPAIR_FRM_KEY: &str = "Repair";
const SFALL_CONFIG_SCIENCE_REPAIR_TARGET_TYPE_KEY: &str = "ScienceOnCritters";
const SFALL_CONFIG_GAME_DIALOG_FIX_KEY: &str = "DialogueFix";
const SFALL_CONFIG_TWEAKS_FILE_KEY: &str = "TweaksFile";
const SFALL_CONFIG_GAME_DIALOG_GENDER_WORDS_KEY: &str = "DialogGenderWords";
const SFALL_CONFIG_TOWN_MAP_HOTKEYS_FIX_KEY: &str = "TownMapHotkeysFix";
const SFALL_CONFIG_EXTRA_MESSAGE_LISTS_KEY: &str = "ExtraGameMsgFileList";
const SFALL_CONFIG_NUMBERS_IS_DIALOG_KEY: &str = "NumbersInDialogue";
const SFALL_CONFIG_INI_CONFIG_FOLDER: &str = "IniConfigFolder";
const SFALL_CONFIG_GLOBAL_SCRIPT_PATHS: &str = "GlobalScriptPaths";
const SFALL_CONFIG_AUTO_QUICK_SAVE: &str = "AutoQuickSave";
const SFALL_CONFIG_BURST_MOD_DEFAULT_CENTER_MULTIPLIER: i32 = 1;
const SFALL_CONFIG_BURST_MOD_DEFAULT_CENTER_DIVISOR: i32 = 3;
const SFALL_CONFIG_BURST_MOD_DEFAULT_TARGET_MULTIPLIER: i32 = 1;
const SFALL_CONFIG_BURST_MOD_DEFAULT_TARGET_DIVISOR: i32 = 2;

const DEATH_WINDOW_WIDTH: i32 = 640;
const DEATH_WINDOW_HEIGHT: i32 = 480;

// 0x5194C8
const _mainMap: &str = "artemple.map";

// 0x5194D8
static _main_game_paused: bool = false;

// 0x5194DC
// static char** _main_selfrun_list = NULL;
static _main_selfrun_list: Vec<String> = Vec::new();

// 0x5194E0
static _main_selfrun_count: i32 = 0;

// 0x5194E4
static _main_selfrun_index: i32 = 0;

// 0x5194E8
static _main_show_death_scene: bool = false;

// A switch to pick selfrun vs. intro video for screensaver:
// - `false` - will play next selfrun recording
// - `true` - will play intro video
//
// This value will alternate on every attempt, even if there are no selfrun
// recordings.
//
// 0x5194EC
static gMainMenuScreensaverCycle: bool = true;

// 0x614838
static _main_death_voiceover_done: bool = true;

static _game_user_wants_to_quit: bool = false;

// 0x6639D0
static mut gPalette: [u8; 256 * 3] = [0; 256 * 3];

// 0x663CD0
const gPaletteWhite: [u8; 256 * 3] = [63; 256 * 3];
const gPaletteBlack: [u8; 256 * 3] = [0; 256 * 3];
static mut _colorTable: [u8; 32768] = [0; 32768];

// 0x48099C
fn falloutMain(argc: u32, argv: Vec<String>) -> u32 {
    unsafe {
        if !autorunMutexCreate() {
            return 1;
        }
    }
    if !falloutInit(argv) {
        return 1;
    }
    let parsed_sfall_config = Ini::load_from_file(SFALL_CONFIG_FILE_NAME);
    if parsed_sfall_config.is_err() {
        error!(
            "couldn't load {} ... stopping neofallout",
            SFALL_CONFIG_FILE_NAME
        );
        return 1;
    }
    let gsfallConfig: Ini = parsed_sfall_config.unwrap();
    // SFALL: Allow to skip intro movies
    let skipOpeningMovies: i32 = gsfallConfig
        .get_from_or(
            Some(SFALL_CONFIG_MISC_KEY),
            SFALL_CONFIG_SKIP_OPENING_MOVIES_KEY,
            "0",
        )
        .parse::<i32>()
        .unwrap();
    if skipOpeningMovies < 1 {
        unsafe {
            gameMoviePlay(
                GameMovie::MOVIE_IPLOGO as i32,
                GameMovieFlags::GAME_MOVIE_FADE_IN as i32,
            );
            gameMoviePlay(GameMovie::MOVIE_INTRO as i32, 0);
            gameMoviePlay(GameMovie::MOVIE_CREDITS as i32, 0);
        }
    }
    unsafe {
        if mainMenuWindowInit() == 0 {
            let mut done: bool = false;
            while !done {
                keyboardReset();
                _gsound_background_play_level_music(str_to_c_char("07desert"), 11);
                mainMenuWindowUnhide(true);

                mouseShowCursor();
                let mainMenuRc: i32 = mainMenuWindowHandleEvents();
                mouseHideCursor();

                if MainMenuOption::MAIN_MENU_INTRO as i32 == mainMenuRc {
                    mainMenuWindowHide(true);
                    gameMoviePlay(
                        GameMovie::MOVIE_INTRO as i32,
                        GameMovieFlags::GAME_MOVIE_PAUSE_MUSIC as i32,
                    );
                    gameMoviePlay(GameMovie::MOVIE_CREDITS as i32, 0);
                } else if MainMenuOption::MAIN_MENU_NEW_GAME as i32 == mainMenuRc {
                    mainMenuWindowHide(true);
                    mainMenuWindowFree();
                    if characterSelectorOpen() == 2 {
                        gameMoviePlay(
                            GameMovie::MOVIE_ELDER as i32,
                            GameMovieFlags::GAME_MOVIE_STOP_MUSIC as i32,
                        );
                        randomSeedPrerandom(-1);

                        // SFALL: Override starting map.
                        let mapName: &str = gsfallConfig.get_from_or(
                            Some(SFALL_CONFIG_MISC_KEY),
                            SFALL_CONFIG_STARTING_MAP_KEY,
                            _mainMap,
                        );

                        _main_load_new(mapName);

                        // SFALL: AfterNewGameStartHook.
                        sfall_gl_scr_exec_start_proc();

                        mainLoop();
                        paletteFadeTo(gPaletteWhite.as_mut_ptr());

                        // NOTE: Uninline.
                        main_unload_new();

                        // NOTE: Uninline.
                        main_reset_system();

                        if _main_show_death_scene {
                            showDeath();
                            _main_show_death_scene = false;
                        }
                    }

                    mainMenuWindowInit();
                } else if MainMenuOption::MAIN_MENU_LOAD_GAME as i32 == mainMenuRc {
                    let mut win: i32 = windowCreate(
                        0,
                        0,
                        screenGetWidth(),
                        screenGetHeight(),
                        _colorTable[0],
                        WindowFlags::WINDOW_MODAL as i32 | WindowFlags::WINDOW_MOVE_ON_TOP as i32,
                    );
                    mainMenuWindowHide(true);
                    mainMenuWindowFree();

                    // NOTE: Uninline.
                    main_loadgame_new();

                    rs_loadColorPalette("color.pal");
                    paletteFadeTo(_cmap);
                    let mut loadGameRc: i32 =
                        lsgLoadGame(LoadSaveMode::LOAD_SAVE_MODE_FROM_MAIN_MENU as i32);
                    if loadGameRc == -1 {
                        error!("Error running LoadGame()!");
                    } else if loadGameRc != 0 {
                        windowDestroy(win);
                        win = -1;
                        mainLoop();
                    }
                    paletteFadeTo(gPaletteWhite.as_mut_ptr());
                    if win != -1 {
                        windowDestroy(win);
                    }

                    // NOTE: Uninline.
                    main_unload_new();

                    // NOTE: Uninline.
                    main_reset_system();

                    if _main_show_death_scene {
                        showDeath();
                        _main_show_death_scene = false;
                    }
                    mainMenuWindowInit();
                } else if MainMenuOption::MAIN_MENU_TIMEOUT as i32 == mainMenuRc {
                    debug!("Main menu timed-out");
                    // FALLTHROUGH
                } else if MainMenuOption::MAIN_MENU_SCREENSAVER as i32 == mainMenuRc {
                    _main_selfrun_play();
                } else if MainMenuOption::MAIN_MENU_OPTIONS as i32 == mainMenuRc {
                    mainMenuWindowHide(true);
                    doPreferences(true);
                } else if MainMenuOption::MAIN_MENU_CREDITS as i32 == mainMenuRc {
                    mainMenuWindowHide(true);
                    creditsOpen(str_to_c_char("credits.txt"), -1, false);
                } else if MainMenuOption::MAIN_MENU_QUOTES as i32 == mainMenuRc {
                    // NOTE: There is a strange cmp at 0x480C50. Both operands are
                    // zero, set before the loop and do not modify afterwards. For
                    // clarity this condition is omitted.
                    mainMenuWindowHide(true);
                    let quotes_txt: *const c_char =
                        CString::new("quotes.txt").unwrap().as_ptr() as *const c_char;
                    creditsOpen(quotes_txt, -1, true);
                } else if MainMenuOption::MAIN_MENU_EXIT as i32 == mainMenuRc || -1 == mainMenuRc {
                    done = true;
                    mainMenuWindowHide(true);
                    mainMenuWindowFree();
                    backgroundSoundDelete();
                } else if MainMenuOption::MAIN_MENU_SELFRUN as i32 == mainMenuRc {
                    _main_selfrun_record();
                }
            }
        }

        // NOTE: Uninline.
        main_exit_system();

        autorunMutexClose();
    }
    return 0;
}

// 0x480CC0
fn falloutInit(argv: Vec<String>) -> bool {
    unsafe {
        let mut cmdLineArgs: Vec<*mut c_char> = argv
            .iter()
            .map(|x| CString::new(x.clone()).unwrap().into_raw())
            .collect();
        if gameInitWithOptions(
            str_to_c_char("FALLOUT II"),
            false,
            0,
            0,
            argv.len() as i32,
            cmdLineArgs.as_mut_ptr(),
        ) == -1
        {
            return false;
        }

        if _main_selfrun_list.len() >= 1 {
            _main_selfrun_exit();
        }

        if selfrunInitFileList(
            // TODO this compiles but I don't believe, that this will work
            vec_str_to_mut_mut_mut_c_char(_main_selfrun_list),
            &mut _main_selfrun_count as *mut i32,
        ) == 0
        {
            _main_selfrun_index = 0;
        }
    }
    return true;
}

// NOTE: Inlined.
//
// 0x480D0C
fn main_reset_system() -> u32 {
    unsafe {
        gameReset();
    }
    return 1;
}

// NOTE: Inlined.
//
// 0x480D18
fn main_exit_system() {
    unsafe {
        backgroundSoundDelete();

        // NOTE: Uninline.
        _main_selfrun_exit();

        gameExit();
    }
}

// 0x480D4C
fn _main_load_new(mapFileName: &str) -> i32 {
    unsafe {
        _game_user_wants_to_quit = false;
        _main_show_death_scene = false;
        gDude::flags &= /*~*/!ObjectFlags::OBJECT_FLAT;
        objectShow(gDude, NULL);
        mouseHideCursor();

        let mut win: i32 = windowCreate(
            0,
            0,
            screenGetWidth(),
            screenGetHeight(),
            _colorTable[0] as i32,
            WindowFlags::WINDOW_MODAL as i32 | WindowFlags::WINDOW_MOVE_ON_TOP as i32,
        );
        windowRefresh(win);

        rs_loadColorPalette("color.pal");
        paletteFadeTo(_cmap);
        _map_init();
        gameMouseSetCursor(MouseCursorType::MOUSE_CURSOR_NONE as i32);
        mouseShowCursor();
        mapLoadByName(str_to_mut_c_char(mapFileName));
        wmMapMusicStart();
        paletteFadeTo(gPaletteWhite.as_mut_ptr());
        windowDestroy(win);
        rs_loadColorPalette("color.pal");
        paletteFadeTo(_cmap);
    }
    return 0;
}

// NOTE: Inlined.
//
// 0x480DF8
fn main_loadgame_new() -> i32 {
    unsafe {
        _game_user_wants_to_quit = false;
        _main_show_death_scene = false;

        gDude::flags &= /*~*/!ObjectFlags::OBJECT_FLAT;

        objectShow(gDude, NULL);
        mouseHideCursor();

        _map_init();

        gameMouseSetCursor(MouseCursorType::MOUSE_CURSOR_NONE as i32);
        mouseShowCursor();
    }
    return 0;
}

// 0x480E34
fn main_unload_new() {
    unsafe {
        objectHide(gDude, NULL);
        _map_exit();
    }
}

// 0x480E48
fn mainLoop() {
    unsafe {
        let mut cursorWasHidden: bool = cursorIsHidden();
        if cursorWasHidden {
            mouseShowCursor();
        }

        _main_game_paused = false;

        scriptsEnable();

        while !_game_user_wants_to_quit {
            sharedFpsLimiter.mark();

            let keyCode: i32 = inputGetInput();

            // SFALL: MainLoopHook.
            sfall_gl_scr_process_main();

            gameHandleKey(keyCode, false);

            scriptsHandleRequests();

            mapHandleTransition();

            if _main_game_paused {
                _main_game_paused = false;
            }

            if (gDude::/*->*/data.critter.combat.results
                & (Dam::DAM_DEAD as i32 | Dam::DAM_KNOCKED_OUT as i32))
                != 0
            {
                endgameSetupDeathEnding(ENDGAME_DEATH_ENDING_REASON_DEATH);
                _main_show_death_scene = true;
                _game_user_wants_to_quit = true; //2;
            }

            renderPresent();
            sharedFpsLimiter.throttle();
        }

        scriptsDisable();

        if cursorWasHidden {
            mouseHideCursor();
        }
    }
}

// 0x480F38
fn _main_selfrun_exit() {
    unsafe {
        if !_main_selfrun_list.is_empty() {
            _main_selfrun_list.clear();
        }

        _main_selfrun_count = 0;
        _main_selfrun_index = 0;
    }
}

// 0x480F64
fn _main_selfrun_record() {
    unsafe {
        let selfrunData: SelfrunData;
        let mut ready: bool = false;

        let mut fileList: Vec<String>;
        let mut fileListLength: u32 = fileNameListInit("maps\\*.map", &fileList, 0, 0);
        if fileListLength != 0 {
            let selectedFileIndex: i32 = _win_list_select(
                str_to_c_char("Select Map"),
                fileList,
                fileListLength,
                0,
                80,
                80,
                0x10000 | 0x100 | 4,
            );
            if selectedFileIndex != -1 {
                // NOTE: It's size is likely 13 chars (on par with SelfrunData
                // fields), but due to the padding it takes 16 chars on stack.
                // char recordingName[SELFRUN_RECORDING_FILE_NAME_LENGTH];
                // recordingName[0] = '\0';
                let mut recordingName: String = String::from("");
                if _win_get_str(
                    recordingName,
                    recordingName.len() - 2,
                    "Enter name for recording (8 characters max, no extension):",
                    100,
                    100,
                ) == 0
                {
                    // memset(&selfrunData, 0, selfrunData. sizeof(selfrunData)); -- initialization of memory
                    if selfrunPrepareRecording(
                        str_to_c_char(recordingName.as_str()),
                        fileList.get(selectedFileIndex),
                        selfrunData,
                    ) == 0
                    {
                        ready = true;
                    }
                }
            }
            fileNameListFree(vec_str_to_mut_mut_mut_c_char(fileList), 0);
        }

        if ready {
            mainMenuWindowHide(true);
            mainMenuWindowFree();
            backgroundSoundDelete();
            randomSeedPrerandom(0xBEEFFEED);

            // NOTE: Uninline.
            main_reset_system();

            _proto_dude_init(str_to_c_char("premade\\combat.gcd"));
            _main_load_new(selfrunData.mapFileName);
            selfrunRecordingLoop(&selfrunData);
            paletteFadeTo(gPaletteWhite.as_mut_ptr());

            // NOTE: Uninline.
            main_unload_new();

            // NOTE: Uninline.
            main_reset_system();

            mainMenuWindowInit();

            if (_main_selfrun_list != NULL) {
                _main_selfrun_exit();
            }

            if (selfrunInitFileList(&_main_selfrun_list, &_main_selfrun_count) == 0) {
                _main_selfrun_index = 0;
            }
        }
    }
}

// 0x48109C
fn _main_selfrun_play() {
    unsafe {
        if !gMainMenuScreensaverCycle && _main_selfrun_count > 0 {
            let selfrunData: SelfrunData;
            if (selfrunPreparePlayback(_main_selfrun_list[_main_selfrun_index], &selfrunData) == 0)
            {
                mainMenuWindowHide(true);
                mainMenuWindowFree();
                backgroundSoundDelete();
                randomSeedPrerandom(0xBEEFFEED);

                // NOTE: Uninline.
                main_reset_system();

                _proto_dude_init(str_to_c_char("premade\\combat.gcd"));
                _main_load_new(selfrunData.mapFileName);
                selfrunPlaybackLoop(&selfrunData);
                paletteFadeTo(gPaletteWhite.as_mut_ptr());

                // NOTE: Uninline.
                main_unload_new();

                // NOTE: Uninline.
                main_reset_system();

                mainMenuWindowInit();
            }

            _main_selfrun_index += 1;
            if _main_selfrun_index >= _main_selfrun_count {
                _main_selfrun_index = 0;
            }
        } else {
            mainMenuWindowHide(true);
            gameMoviePlay(
                GameMovie::MOVIE_INTRO as i32,
                GameMovieFlags::GAME_MOVIE_PAUSE_MUSIC as i32,
            );
        }

        gMainMenuScreensaverCycle = !gMainMenuScreensaverCycle;
    }
}

// 0x48118C
fn showDeath() {
    unsafe {
        artCacheFlush();
        colorCycleDisable();
        gameMouseSetCursor(MouseCursorType::MOUSE_CURSOR_NONE as i32);

        let mut oldCursorIsHidden: bool = cursorIsHidden();
        if oldCursorIsHidden {
            mouseShowCursor();
        }

        let deathWindowX: i32 = (screenGetWidth() - DEATH_WINDOW_WIDTH) / 2;
        let deathWindowY: i32 = (screenGetHeight() - DEATH_WINDOW_HEIGHT) / 2;
        let mut win: i32 = windowCreate(
            deathWindowX,
            deathWindowY,
            DEATH_WINDOW_WIDTH,
            DEATH_WINDOW_HEIGHT,
            0,
            WindowFlags::WINDOW_MOVE_ON_TOP as i32,
        );
        if win != -1 {
            loop {
                let mut windowBuffer: *mut u8 = windowGetBuffer(win);
                if (windowBuffer == NULL) {
                    break;
                }

                // DEATH.FRM
                let mut backgroundFrmImage: FrmImage;
                let fid: i32 = buildFid(ObjectTypes::OBJ_TYPE_INTERFACE as i32, 309, 0, 0, 0);
                if !backgroundFrmImage.lock(fid) {
                    break;
                }

                while mouseGetEvent() != 0 {
                    sharedFpsLimiter.mark();

                    inputGetInput();

                    renderPresent();
                    sharedFpsLimiter.throttle();
                }

                keyboardReset();
                inputEventQueueReset();

                blitBufferToBuffer(
                    backgroundFrmImage.getData(),
                    640,
                    480,
                    640,
                    windowBuffer,
                    640,
                );
                backgroundFrmImage.unlock();

                const deathFileName: *mut c_char = endgameDeathEndingGetFileName();

                if (settings.preferences.subtitles) {
                    let mut text: String;
                    if (_mainDeathGrabTextFile(deathFileName, text) == 0) {
                        debug!("((ShowDeath)): {}", text);

                        // short beginnings[WORD_WRAP_MAX_COUNT];
                        // short count;
                        // if (_mainDeathWordWrap(text, 560, beginnings, &count) == 0) {
                        //     unsigned char* p = windowBuffer + 640 * (480 - fontGetLineHeight() * count - 8);
                        //     bufferFill(p - 602, 564, fontGetLineHeight() * count + 2, 640, 0);
                        //     p += 40;
                        //     for (int index = 0; index < count; index++) {
                        //         fontDrawText(p, text + beginnings[index], 560, 640, _colorTable[32767]);
                        //         p += 640 * fontGetLineHeight();
                        //     }
                        // }
                    }
                }

                windowRefresh(win);

                rs_loadColorPalette("art\\intrface\\death.pal");
                paletteFadeTo(_cmap);

                _main_death_voiceover_done = false;
                speechSetEndCallback(_main_death_voiceover_callback);

                let mut delay: u32;
                if speechLoad(deathFileName, 10, 14, 15) == -1 {
                    delay = 3000;
                } else {
                    delay = UINT_MAX;
                }

                _gsound_speech_play_preloaded();

                // SFALL: Fix the playback of the speech sound file for the death
                // screen.
                inputBlockForTocks(100);

                let mut time: u32 = getTicks();
                let mut keyCode: i32;
                loop {
                    sharedFpsLimiter.mark();

                    keyCode = inputGetInput();

                    renderPresent();
                    sharedFpsLimiter.throttle();
                    if (keyCode > -1 || !_main_death_voiceover_done || getTicksSince(time) > delay)
                    {
                        break;
                    }
                } // while (keyCode == -1 && !_main_death_voiceover_done && getTicksSince(time) < delay);

                speechSetEndCallback(NULL);

                speechDelete();

                while mouseGetEvent() != 0 {
                    sharedFpsLimiter.mark();

                    inputGetInput();

                    renderPresent();
                    sharedFpsLimiter.throttle();
                }

                if keyCode == -1 {
                    inputPauseForTocks(500);
                }

                paletteFadeTo(gPaletteBlack.as_mut_ptr());
                rs_loadColorPalette("color.pal");
            } //while (0);
            windowDestroy(win);
        }

        if oldCursorIsHidden {
            mouseHideCursor();
        }

        gameMouseSetCursor(MouseCursorType::MOUSE_CURSOR_ARROW as i32);

        colorCycleEnable();
    }
}

fn rs_loadColorPalette(name: &str) {
    let color_palette_name = str_to_c_char(name);
    unsafe {
        colorPaletteLoad(color_palette_name);
    }
}

fn vec_str_to_mut_mut_mut_c_char(list: Vec<String>) -> *mut *mut *mut c_char {
    let mut converted_main_selfrun_list: Vec<*mut c_char> = _main_selfrun_list
        .iter()
        .map(|x| CString::new(x.clone()).unwrap().into_raw())
        .collect();
    converted_main_selfrun_list.as_mut_ptr() as *mut *mut *mut c_char
}

fn str_to_c_char(name: &str) -> *const c_char {
    let color_palette_name: *const c_char = CString::new(name).unwrap().as_ptr() as *const c_char;
    color_palette_name
}

fn str_to_mut_c_char(name: &str) -> *mut c_char {
    let color_palette_name: *mut c_char = CString::new(name).unwrap().as_ptr() as *mut c_char;
    color_palette_name
}

// 0x4814A8
fn _main_death_voiceover_callback() {
    _main_death_voiceover_done = true;
}

// Read endgame subtitle.
//
// 0x4814B4
fn _mainDeathGrabTextFile(fileName: String, dest: String) -> i32 {
    unsafe {
        let mut p: str = strrchr(fileName, '\\');
        if (p == NULL) {
            return -1;
        }

        let path: String = String::new();
        snprintf(
            path,
            sizeof(path),
            "text\\%s\\cuts\\%s%s",
            settings.system.language.c_str(),
            p + 1,
            ".TXT",
        );

        /*File**/
        let mut stream: File = fileOpen(path, "rt");
        if (stream == NULL) {
            return -1;
        }

        loop {
            if false {
                break;
            }
            let mut c: i32 = fileReadChar(stream);
            if c == -1 {
                break;
            }
            if c == '\n' {
                c = ' ';
            }
        }

        fileClose(stream);

        *dest = '\0';
    }
    return 0;
}

// 0x481598
fn _mainDeathWordWrap(
    text: String,
    width: i32,
    /*short* */ beginnings: i8,
    /*short* */ count: i8,
) -> i32 {
    loop {
        if false {
            break;
        }
        let mut sep: str = strchr(text, ':');
        if (sep == NULL) {
            break;
        }
        if sep - 1 < text {
            break;
        }
        sep[0] = ' ';
        sep[-1] = ' ';
    }
    unsafe {
        if (wordWrap(text, width, beginnings, count) == -1) {
            return -1;
        }
    }
    // TODO: Probably wrong.
    *count -= 1;

    // for (int index = 1; index < *count; index++) {
    //     char* p = text + beginnings[index];
    //     while (p >= text && *p != ' ') {
    //         p--;
    //         beginnings[index]--;
    //     }

    //     if (p != NULL) {
    //         *p = '\0';
    //         beginnings[index]++;
    //     }
    // }

    return 0;
}

fn main() {
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
        .unwrap();
    let _handle = log4rs::init_config(config).unwrap();
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    falloutMain(args.len() as u32, args);
}
