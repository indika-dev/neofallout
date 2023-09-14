#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::fs::File;

use neofallout::{
    autorunMutexCreate, colorPaletteLoad, debugPrint, gameMoviePlay, lsgLoadGame,
    mainMenuWindowFree, mainMenuWindowHandleEvents, mainMenuWindowHide, paletteFadeTo,
    screenGetHeight, screenGetWidth, sfall_gl_scr_exec_start_proc, windowCreate, windowDestroy,
    Dam, LoadSaveMode, MainMenuOption, MouseCursorType, ObjectFlags, WindowFlags, _map_exit,
    _map_init, autorunMutexClose, backgroundSoundDelete, creditsOpen, cursorIsHidden,
    doPreferences, endgameSetupDeathEnding, fileNameListInit, gameHandleKey, gameInitWithOptions,
    gameMouseSetCursor, inputGetInput, mainMenuWindowFree, mapHandleTransition, mouseHideCursor,
    mouseShowCursor, objectHide, objectShow, renderPresent, scriptsDisable, scriptsEnable,
    scriptsHandleRequests, selfrunFreeFileList, selfrunInitFileList, sfall_gl_scr_process_main,
    wmMapMusicStart, SelfrunData, _proto_dude_init, _win_list_select, artCacheFlush,
    backgroundSoundDelete, blitBufferToBuffer, colorCycleDisable, endgameDeathEndingGetFileName,
    inputEventQueueReset, keyboardReset, main_reset_system, memset, randomSeedPrerandom,
    selfrunPreparePlayback, selfrunPrepareRecording, selfrunRecordingLoop, windowGetBuffer,
    FrmImage, _gsound_speech_play_preloaded, bufferFill, characterSelectorOpen, colorCycleEnable,
    fileClose, fileOpen, fileReadChar, fontDrawText, fontGetLineHeight, free, gameExit,
    gameMouseSetCursor, gameReset, getTicks, inputBlockForTocks, inputPauseForTocks, mapLoadByName,
    mouseGetEvent, renderPresent, sizeof, snprintf, speechDelete, speechLoad, speechSetEndCallback,
    speechSetEndCallback, strrchr, windowRefresh, wordWrap, SFALL_CONFIG_KEYS,
};

const DEATH_WINDOW_WIDTH: u8 = 640;
const DEATH_WINDOW_HEIGHT: u8 = 480;

// 0x5194C8
const _mainMap: &str = "artemple.map";

// 0x5194D8
static _main_game_paused: bool = false;

// 0x5194DC
// static char** _main_selfrun_list = NULL;
static _main_selfrun_list: Vec<str> = Vec::new();

// 0x5194E0
static _main_selfrun_count: u32 = 0;

// 0x5194E4
static _main_selfrun_index: u32 = 0;

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

// 0x48099C
fn falloutMain(argv: Vec<str>) -> u32 {
    unsafe {
        if (!autorunMutexCreate()) {
            return 1;
        }

        if (!falloutInit(argc, argv)) {
            return 1;
        }

        // SFALL: Allow to skip intro movies
        let mut skipOpeningMovies: i32 = 0;
        configGetInt(
            &gSfallConfig,
            SFALL_CONFIG_KEYS::SFALL_CONFIG_MISC_KEY,
            SFALL_CONFIG_KEYS::SFALL_CONFIG_SKIP_OPENING_MOVIES_KEY,
            &mut skipOpeningMovies as *mut _,
        );
        if (skipOpeningMovies < 1) {
            gameMoviePlay(MOVIE_IPLOGO, GAME_MOVIE_FADE_IN);
            gameMoviePlay(MOVIE_INTRO, 0);
            gameMoviePlay(MOVIE_CREDITS, 0);
        }

        if (mainMenuWindowInit() == 0) {
            let mut done: bool = false;
            while (!done) {
                keyboardReset();
                _gsound_background_play_level_music("07desert", 11);
                mainMenuWindowUnhide(1);

                mouseShowCursor();
                let mainMenuRc: i8 = mainMenuWindowHandleEvents();
                mouseHideCursor();

                match mainMenuRc {
                    MainMenuOption::MAIN_MENU_INTRO => {
                        mainMenuWindowHide(true);
                        gameMoviePlay(MOVIE_INTRO, GAME_MOVIE_PAUSE_MUSIC);
                        gameMoviePlay(MOVIE_CREDITS, 0);
                    }
                    MainMenuOption::MAIN_MENU_NEW_GAME => {
                        mainMenuWindowHide(true);
                        mainMenuWindowFree();
                        if (characterSelectorOpen() == 2) {
                            gameMoviePlay(MOVIE_ELDER, GAME_MOVIE_STOP_MUSIC);
                            randomSeedPrerandom(-1);

                            // SFALL: Override starting map.
                            let mut mapName: str = String::new();
                            if (configGetString(
                                &gSfallConfig,
                                SFALL_CONFIG_KEYS::SFALL_CONFIG_MISC_KEY,
                                SFALL_CONFIG_KEYS::SFALL_CONFIG_STARTING_MAP_KEY,
                                &mapName,
                            )) {
                                if (mapName.is_empty()) {
                                    mapName = _mainMap;
                                }
                            }

                            _main_load_new(mapName);
                            free(mapNameCopy);

                            // SFALL: AfterNewGameStartHook.
                            sfall_gl_scr_exec_start_proc();

                            mainLoop();
                            paletteFadeTo(gPaletteWhite);

                            // NOTE: Uninline.
                            main_unload_new();

                            // NOTE: Uninline.
                            main_reset_system();

                            if (_main_show_death_scene != 0) {
                                showDeath();
                                _main_show_death_scene = 0;
                            }
                        }

                        mainMenuWindowInit();
                    }
                    MainMenuOption::MAIN_MENU_LOAD_GAME => {
                        if (1) {
                            let mut win: i32 = windowCreate(
                                0,
                                0,
                                screenGetWidth(),
                                screenGetHeight(),
                                _colorTable[0],
                                WindowFlags::WINDOW_MODAL | WindowFlags::WINDOW_MOVE_ON_TOP,
                            );
                            mainMenuWindowHide(true);
                            mainMenuWindowFree();

                            // NOTE: Uninline.
                            main_loadgame_new();

                            colorPaletteLoad("color.pal");
                            paletteFadeTo(_cmap);
                            let mut loadGameRc: i32 =
                                lsgLoadGame(LoadSaveMode::LOAD_SAVE_MODE_FROM_MAIN_MENU);
                            if (loadGameRc == -1) {
                                debugPrint("\n ** Error running LoadGame()! **\n");
                            } else if (loadGameRc != 0) {
                                windowDestroy(win);
                                win = -1;
                                mainLoop();
                            }
                            paletteFadeTo(gPaletteWhite);
                            if (win != -1) {
                                windowDestroy(win);
                            }

                            // NOTE: Uninline.
                            main_unload_new();

                            // NOTE: Uninline.
                            main_reset_system();

                            if (_main_show_death_scene != 0) {
                                showDeath();
                                _main_show_death_scene = 0;
                            }
                            mainMenuWindowInit();
                        }
                    }
                    MainMenuOption::MAIN_MENU_TIMEOUT => {
                        debugPrint("Main menu timed-out\n");
                        // FALLTHROUGH
                    }
                    MainMenuOption::MAIN_MENU_SCREENSAVER => {
                        _main_selfrun_play();
                    }
                    MainMenuOption::MAIN_MENU_OPTIONS => {
                        mainMenuWindowHide(true);
                        doPreferences(true);
                    }
                    MainMenuOption::MAIN_MENU_CREDITS => {
                        mainMenuWindowHide(true);
                        creditsOpen("credits.txt", -1, false);
                    }
                    MainMenuOption::MAIN_MENU_QUOTES => {
                        // NOTE: There is a strange cmp at 0x480C50. Both operands are
                        // zero, set before the loop and do not modify afterwards. For
                        // clarity this condition is omitted.
                        mainMenuWindowHide(true);
                        creditsOpen("quotes.txt", -1, true);
                    }
                    MainMenuOption::MAIN_MENU_EXIT | -1 => {
                        done = true;
                        mainMenuWindowHide(true);
                        mainMenuWindowFree();
                        backgroundSoundDelete();
                    }
                    MainMenuOption::MAIN_MENU_SELFRUN => {
                        _main_selfrun_record();
                    }
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
fn falloutInit(argc: u32, argv: Vec<str>) -> bool {
    unsafe {
        if (gameInitWithOptions("FALLOUT II", false, 0, 0, argc, argv) == -1) {
            return false;
        }

        if (_main_selfrun_list != NULL) {
            _main_selfrun_exit();
        }

        if (selfrunInitFileList(&_main_selfrun_list, &_main_selfrun_count) == 0) {
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
fn _main_load_new(mapFileName: str) -> i32 {
    unsafe {
        _game_user_wants_to_quit = 0;
        _main_show_death_scene = 0;
        gDude::flags &= /*~*/!ObjectFlags::OBJECT_FLAT;
        objectShow(gDude, NULL);
        mouseHideCursor();

        let mut win: i32 = windowCreate(
            0,
            0,
            screenGetWidth(),
            screenGetHeight(),
            _colorTable[0],
            WindowFlags::WINDOW_MODAL | WindowFlags::WINDOW_MOVE_ON_TOP,
        );
        windowRefresh(win);

        colorPaletteLoad("color.pal");
        paletteFadeTo(_cmap);
        _map_init();
        gameMouseSetCursor(MouseCursorType::MOUSE_CURSOR_NONE);
        mouseShowCursor();
        mapLoadByName(mapFileName);
        wmMapMusicStart();
        paletteFadeTo(gPaletteWhite);
        windowDestroy(win);
        colorPaletteLoad("color.pal");
        paletteFadeTo(_cmap);
    }
    return 0;
}

// NOTE: Inlined.
//
// 0x480DF8
fn main_loadgame_new() -> i32 {
    unsafe {
        _game_user_wants_to_quit = 0;
        _main_show_death_scene = 0;

        gDude::flags &= /*~*/!ObjectFlags::OBJECT_FLAT;

        objectShow(gDude, NULL);
        mouseHideCursor();

        _map_init();

        gameMouseSetCursor(MouseCursorType::MOUSE_CURSOR_NONE);
        mouseShowCursor();
    }
    return 0;
}

// 0x480E34
fn main_unload_new() {
    objectHide(gDude, NULL);
    _map_exit();
}

// 0x480E48
fn mainLoop() {
    unsafe {
        let mut cursorWasHidden: bool = cursorIsHidden();
        if (cursorWasHidden) {
            mouseShowCursor();
        }

        _main_game_paused = 0;

        scriptsEnable();

        while (_game_user_wants_to_quit == 0) {
            sharedFpsLimiter.mark();

            let keyCode: i32 = inputGetInput();

            // SFALL: MainLoopHook.
            sfall_gl_scr_process_main();

            gameHandleKey(keyCode, false);

            scriptsHandleRequests();

            mapHandleTransition();

            if (_main_game_paused != 0) {
                _main_game_paused = 0;
            }

            if ((gDude::/*->*/data.critter.combat.results & (Dam::DAM_DEAD | Dam::DAM_KNOCKED_OUT))
                != 0)
            {
                endgameSetupDeathEnding(ENDGAME_DEATH_ENDING_REASON_DEATH);
                _main_show_death_scene = 1;
                _game_user_wants_to_quit = 2;
            }

            renderPresent();
            sharedFpsLimiter.throttle();
        }

        scriptsDisable();

        if (cursorWasHidden) {
            mouseHideCursor();
        }
    }
}

// 0x480F38
fn _main_selfrun_exit() {
    unsafe {
        if (_main_selfrun_list != NULL) {
            selfrunFreeFileList(&_main_selfrun_list);
        }

        _main_selfrun_count = 0;
        _main_selfrun_index = 0;
        _main_selfrun_list = NULL;
    }
}

// 0x480F64
fn _main_selfrun_record() {
    unsafe {
        let selfrunData: SelfrunData;
        let mut ready: bool = false;

        let mut fileList: Vec<str>;
        let mut fileListLength: u32 = fileNameListInit("maps\\*.map", &fileList, 0, 0);
        if (fileListLength != 0) {
            let selectedFileIndex: u32 = _win_list_select(
                "Select Map",
                fileList,
                fileListLength,
                0,
                80,
                80,
                0x10000 | 0x100 | 4,
            );
            if (selectedFileIndex != -1) {
                // NOTE: It's size is likely 13 chars (on par with SelfrunData
                // fields), but due to the padding it takes 16 chars on stack.
                // char recordingName[SELFRUN_RECORDING_FILE_NAME_LENGTH];
                // recordingName[0] = '\0';
                let mut recordingName: str = String::new();
                if (_win_get_str(
                    recordingName,
                    sizeof(recordingName) - 2,
                    "Enter name for recording (8 characters max, no extension):",
                    100,
                    100,
                ) == 0)
                {
                    memset(&selfrunData, 0, sizeof(selfrunData));
                    if (selfrunPrepareRecording(
                        recordingName,
                        fileList[selectedFileIndex],
                        &selfrunData,
                    ) == 0)
                    {
                        ready = true;
                    }
                }
            }
            fileNameListFree(&fileList, 0);
        }

        if (ready) {
            mainMenuWindowHide(true);
            mainMenuWindowFree();
            backgroundSoundDelete();
            randomSeedPrerandom(0xBEEFFEED);

            // NOTE: Uninline.
            main_reset_system();

            _proto_dude_init("premade\\combat.gcd");
            _main_load_new(selfrunData.mapFileName);
            selfrunRecordingLoop(&selfrunData);
            paletteFadeTo(gPaletteWhite);

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
        if (!gMainMenuScreensaverCycle && _main_selfrun_count > 0) {
            let selfrunData: SelfrunData;
            if (selfrunPreparePlayback(_main_selfrun_list[_main_selfrun_index], &selfrunData) == 0)
            {
                mainMenuWindowHide(true);
                mainMenuWindowFree();
                backgroundSoundDelete();
                randomSeedPrerandom(0xBEEFFEED);

                // NOTE: Uninline.
                main_reset_system();

                _proto_dude_init("premade\\combat.gcd");
                _main_load_new(selfrunData.mapFileName);
                selfrunPlaybackLoop(&selfrunData);
                paletteFadeTo(gPaletteWhite);

                // NOTE: Uninline.
                main_unload_new();

                // NOTE: Uninline.
                main_reset_system();

                mainMenuWindowInit();
            }

            _main_selfrun_index += 1;
            if (_main_selfrun_index >= _main_selfrun_count) {
                _main_selfrun_index = 0;
            }
        } else {
            mainMenuWindowHide(true);
            gameMoviePlay(MOVIE_INTRO, GAME_MOVIE_PAUSE_MUSIC);
        }

        gMainMenuScreensaverCycle = !gMainMenuScreensaverCycle;
    }
}

// 0x48118C
fn showDeath() {
    unsafe {
        artCacheFlush();
        colorCycleDisable();
        gameMouseSetCursor(MOUSE_CURSOR_NONE);

        let mut oldCursorIsHidden: bool = cursorIsHidden();
        if (oldCursorIsHidden) {
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
            WINDOW_MOVE_ON_TOP,
        );
        if (win != -1) {
            loop {
                let mut windowBuffer: *const char = windowGetBuffer(win);
                if (windowBuffer == NULL) {
                    break;
                }

                // DEATH.FRM
                let mut backgroundFrmImage: FrmImage;
                let fid: i32 = buildFid(OBJ_TYPE_INTERFACE, 309, 0, 0, 0);
                if (!backgroundFrmImage.lock(fid)) {
                    break;
                }

                while (mouseGetEvent() != 0) {
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

                const deathFileName: str = endgameDeathEndingGetFileName();

                if (settings.preferences.subtitles) {
                    let mut text: str;
                    if (_mainDeathGrabTextFile(deathFileName, text) == 0) {
                        debugPrint("\n((ShowDeath)): %s\n", text);

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

                colorPaletteLoad("art\\intrface\\death.pal");
                paletteFadeTo(_cmap);

                _main_death_voiceover_done = false;
                speechSetEndCallback(_main_death_voiceover_callback);

                let mut delay: u32;
                if (speechLoad(deathFileName, 10, 14, 15) == -1) {
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

                while (mouseGetEvent() != 0) {
                    sharedFpsLimiter.mark();

                    inputGetInput();

                    renderPresent();
                    sharedFpsLimiter.throttle();
                }

                if (keyCode == -1) {
                    inputPauseForTocks(500);
                }

                paletteFadeTo(gPaletteBlack);
                colorPaletteLoad("color.pal");
            } //while (0);
            windowDestroy(win);
        }

        if (oldCursorIsHidden) {
            mouseHideCursor();
        }

        gameMouseSetCursor(MOUSE_CURSOR_ARROW);

        colorCycleEnable();
    }
}

// 0x4814A8
fn _main_death_voiceover_callback() {
    _main_death_voiceover_done = true;
}

// Read endgame subtitle.
//
// 0x4814B4
fn _mainDeathGrabTextFile(fileName: str, dest: str) -> i32 {
    unsafe {
        let mut p: str = strrchr(fileName, '\\');
        if (p == NULL) {
            return -1;
        }

        let path: str = String::new();
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

        while (true) {
            let mut c: i32 = fileReadChar(stream);
            if (c == -1) {
                break;
            }

            if (c == '\n') {
                c = ' ';
            }

            // *dest++ = (c & 0xFF);
        }

        fileClose(stream);

        *dest = '\0';
    }
    return 0;
}

// 0x481598
fn _mainDeathWordWrap(
    text: st,
    width: i32,
    /*short* */ beginnings: i8,
    /*short* */ count: i8,
) -> i32 {
    while (true) {
        let mut sep: str = strchr(text, ':');
        if (sep == NULL) {
            break;
        }

        if (sep - 1 < text) {
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
    println!("Hello, world!");
}
