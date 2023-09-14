#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::fs::File;

use neofallout::MouseCursorType;

const DEATH_WINDOW_WIDTH: u8 = 640;
const DEATH_WINDOW_HEIGHT: u8 = 480;

// static bool falloutInit(int argc, char** argv);
// static int main_reset_system();
// static void main_exit_system();
// static int _main_load_new(char* fname);
// static int main_loadgame_new();
// static void main_unload_new();
// static void mainLoop();
// static void _main_selfrun_exit();
// static void _main_selfrun_record();
// static void _main_selfrun_play();
// static void showDeath();
// static void _main_death_voiceover_callback();
// static int _mainDeathGrabTextFile(const char* fileName, char* dest);
// static int _mainDeathWordWrap(char* text, int width, short* beginnings, short* count);

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

enum SFALL_CONFIG_KEYS {
    SFALL_CONFIG_FILE_NAME = "ddraw.ini",
    SFALL_CONFIG_MISC_KEY = "Misc",
    SFALL_CONFIG_SCRIPTS_KEY = "Scripts",
    SFALL_CONFIG_DUDE_NATIVE_LOOK_JUMPSUIT_MALE_KEY = "MaleDefaultModel",
    SFALL_CONFIG_DUDE_NATIVE_LOOK_JUMPSUIT_FEMALE_KEY = "FemaleDefaultModel",
    SFALL_CONFIG_DUDE_NATIVE_LOOK_TRIBAL_MALE_KEY = "MaleStartModel",
    SFALL_CONFIG_DUDE_NATIVE_LOOK_TRIBAL_FEMALE_KEY = "FemaleStartModel",
    SFALL_CONFIG_START_YEAR = "StartYear",
    SFALL_CONFIG_START_MONTH = "StartMonth",
    SFALL_CONFIG_START_DAY = "StartDay",
    SFALL_CONFIG_MAIN_MENU_BIG_FONT_COLOR_KEY = "MainMenuBigFontColour",
    SFALL_CONFIG_MAIN_MENU_CREDITS_OFFSET_X_KEY = "MainMenuCreditsOffsetX",
    SFALL_CONFIG_MAIN_MENU_CREDITS_OFFSET_Y_KEY = "MainMenuCreditsOffsetY",
    SFALL_CONFIG_MAIN_MENU_FONT_COLOR_KEY = "MainMenuFontColour",
    SFALL_CONFIG_MAIN_MENU_OFFSET_X_KEY = "MainMenuOffsetX",
    SFALL_CONFIG_MAIN_MENU_OFFSET_Y_KEY = "MainMenuOffsetY",
    SFALL_CONFIG_SKIP_OPENING_MOVIES_KEY = "SkipOpeningMovies",
    SFALL_CONFIG_STARTING_MAP_KEY = "StartingMap",
    SFALL_CONFIG_KARMA_FRMS_KEY = "KarmaFRMs",
    SFALL_CONFIG_KARMA_POINTS_KEY = "KarmaPoints",
    SFALL_CONFIG_DISPLAY_KARMA_CHANGES_KEY = "DisplayKarmaChanges",
    SFALL_CONFIG_OVERRIDE_CRITICALS_MODE_KEY = "OverrideCriticalTable",
    SFALL_CONFIG_OVERRIDE_CRITICALS_FILE_KEY = "OverrideCriticalFile",
    SFALL_CONFIG_REMOVE_CRITICALS_TIME_LIMITS_KEY = "RemoveCriticalTimelimits",
    SFALL_CONFIG_BOOKS_FILE_KEY = "BooksFile",
    SFALL_CONFIG_ELEVATORS_FILE_KEY = "ElevatorsFile",
    SFALL_CONFIG_CONSOLE_OUTPUT_FILE_KEY = "ConsoleOutputPath",
    SFALL_CONFIG_PREMADE_CHARACTERS_FILE_NAMES_KEY = "PremadePaths",
    SFALL_CONFIG_PREMADE_CHARACTERS_FACE_FIDS_KEY = "PremadeFIDs",
    SFALL_CONFIG_BURST_MOD_ENABLED_KEY = "ComputeSprayMod",
    SFALL_CONFIG_BURST_MOD_CENTER_MULTIPLIER_KEY = "ComputeSpray_CenterMult",
    SFALL_CONFIG_BURST_MOD_CENTER_DIVISOR_KEY = "ComputeSpray_CenterDiv",
    SFALL_CONFIG_BURST_MOD_TARGET_MULTIPLIER_KEY = "ComputeSpray_TargetMult",
    SFALL_CONFIG_BURST_MOD_TARGET_DIVISOR_KEY = "ComputeSpray_TargetDiv",
    SFALL_CONFIG_DYNAMITE_MIN_DAMAGE_KEY = "Dynamite_DmgMin",
    SFALL_CONFIG_DYNAMITE_MAX_DAMAGE_KEY = "Dynamite_DmgMax",
    SFALL_CONFIG_PLASTIC_EXPLOSIVE_MIN_DAMAGE_KEY = "PlasticExplosive_DmgMin",
    SFALL_CONFIG_PLASTIC_EXPLOSIVE_MAX_DAMAGE_KEY = "PlasticExplosive_DmgMax",
    SFALL_CONFIG_EXPLOSION_EMITS_LIGHT_KEY = "ExplosionsEmitLight",
    SFALL_CONFIG_MOVIE_TIMER_ARTIMER1 = "MovieTimer_artimer1",
    SFALL_CONFIG_MOVIE_TIMER_ARTIMER2 = "MovieTimer_artimer2",
    SFALL_CONFIG_MOVIE_TIMER_ARTIMER3 = "MovieTimer_artimer3",
    SFALL_CONFIG_MOVIE_TIMER_ARTIMER4 = "MovieTimer_artimer4",
    SFALL_CONFIG_CITY_REPUTATION_LIST_KEY = "CityRepsList",
    SFALL_CONFIG_UNARMED_FILE_KEY = "UnarmedFile",
    SFALL_CONFIG_DAMAGE_MOD_FORMULA_KEY = "DamageFormula",
    SFALL_CONFIG_BONUS_HTH_DAMAGE_FIX_KEY = "BonusHtHDamageFix",
    SFALL_CONFIG_DISPLAY_BONUS_DAMAGE_KEY = "DisplayBonusDamage",
    SFALL_CONFIG_USE_LOCKPICK_FRM_KEY = "Lockpick",
    SFALL_CONFIG_USE_STEAL_FRM_KEY = "Steal",
    SFALL_CONFIG_USE_TRAPS_FRM_KEY = "Traps",
    SFALL_CONFIG_USE_FIRST_AID_FRM_KEY = "FirstAid",
    SFALL_CONFIG_USE_DOCTOR_FRM_KEY = "Doctor",
    SFALL_CONFIG_USE_SCIENCE_FRM_KEY = "Science",
    SFALL_CONFIG_USE_REPAIR_FRM_KEY = "Repair",
    SFALL_CONFIG_SCIENCE_REPAIR_TARGET_TYPE_KEY = "ScienceOnCritters",
    SFALL_CONFIG_GAME_DIALOG_FIX_KEY = "DialogueFix",
    SFALL_CONFIG_TWEAKS_FILE_KEY = "TweaksFile",
    SFALL_CONFIG_GAME_DIALOG_GENDER_WORDS_KEY = "DialogGenderWords",
    SFALL_CONFIG_TOWN_MAP_HOTKEYS_FIX_KEY = "TownMapHotkeysFix",
    SFALL_CONFIG_EXTRA_MESSAGE_LISTS_KEY = "ExtraGameMsgFileList",
    SFALL_CONFIG_NUMBERS_IS_DIALOG_KEY = "NumbersInDialogue",
    SFALL_CONFIG_INI_CONFIG_FOLDER = "IniConfigFolder",
    SFALL_CONFIG_GLOBAL_SCRIPT_PATHS = "GlobalScriptPaths",
    SFALL_CONFIG_AUTO_QUICK_SAVE = "AutoQuickSave",
    SFALL_CONFIG_BURST_MOD_DEFAULT_CENTER_MULTIPLIER = 1,
    SFALL_CONFIG_BURST_MOD_DEFAULT_CENTER_DIVISOR = 3,
    SFALL_CONFIG_BURST_MOD_DEFAULT_TARGET_MULTIPLIER = 1,
    SFALL_CONFIG_BURST_MOD_DEFAULT_TARGET_DIVISOR = 2,
}

enum MainMenuOption {
    MAIN_MENU_INTRO = 0,
    MAIN_MENU_NEW_GAME = 1,
    MAIN_MENU_LOAD_GAME = 2,
    MAIN_MENU_SCREENSAVER = 3,
    MAIN_MENU_TIMEOUT = 4,
    MAIN_MENU_CREDITS = 5,
    MAIN_MENU_QUOTES = 6,
    MAIN_MENU_EXIT = 7,
    MAIN_MENU_SELFRUN = 9,
    MAIN_MENU_OPTIONS = 10,
}

enum WindowFlags {
    // Use system window flags which are set during game startup and does not
    // change afterwards.
    WINDOW_USE_DEFAULTS = 0x1,
    WINDOW_DONT_MOVE_TOP = 0x2,
    WINDOW_MOVE_ON_TOP = 0x4,
    WINDOW_HIDDEN = 0x8,
    // Sfall calls this Exclusive.
    WINDOW_MODAL = 0x10,
    WINDOW_TRANSPARENT = 0x20,
    WINDOW_FLAG_0x40 = 0x40,

    /// Specifies that the window is draggable by clicking and moving anywhere
    /// in its background.
    WINDOW_DRAGGABLE_BY_BACKGROUND = 0x80,
    WINDOW_MANAGED = 0x100,
}

enum ObjectFlags {
    OBJECT_HIDDEN = 0x01,

    // Specifies that the object should not be saved to the savegame file.
    //
    // This flag is used in these situations:
    //  - To prevent saving of system objects like dude (which has separate
    // saving routine), egg, mouse cursors, etc.
    //  - To prevent saving of temporary objects (projectiles, explosion
    // effects, etc.).
    //  - To prevent saving of objects which cannot be removed for some reason,
    // like objects trying to delete themselves from scripting engine (used
    // together with `OBJECT_HIDDEN` to prevent affecting game world).
    OBJECT_NO_SAVE = 0x04,
    OBJECT_FLAT = 0x08,
    OBJECT_NO_BLOCK = 0x10,
    OBJECT_LIGHTING = 0x20,

    // Specifies that the object should not be removed (freed) from the game
    // world for whatever reason.
    //
    // This flag is used to prevent freeing of system objects like dude, egg,
    // mouse cursors, etc.
    OBJECT_NO_REMOVE = 0x400,
    OBJECT_MULTIHEX = 0x800,
    OBJECT_NO_HIGHLIGHT = 0x1000,
    OBJECT_QUEUED = 0x2000, // set if there was/is any event for the object
    OBJECT_TRANS_RED = 0x4000,
    OBJECT_TRANS_NONE = 0x8000,
    OBJECT_TRANS_WALL = 0x10000,
    OBJECT_TRANS_GLASS = 0x20000,
    OBJECT_TRANS_STEAM = 0x40000,
    OBJECT_TRANS_ENERGY = 0x80000,
    OBJECT_IN_LEFT_HAND = 0x1000000,
    OBJECT_IN_RIGHT_HAND = 0x2000000,
    OBJECT_WORN = 0x4000000,
    OBJECT_WALL_TRANS_END = 0x10000000,
    OBJECT_LIGHT_THRU = 0x20000000,
    OBJECT_SEEN = 0x40000000,
    OBJECT_SHOOT_THRU = 0x80000000,

    OBJECT_IN_ANY_HAND = OBJECT_IN_LEFT_HAND | OBJECT_IN_RIGHT_HAND,
    OBJECT_EQUIPPED = OBJECT_IN_ANY_HAND | OBJECT_WORN,
    OBJECT_FLAG_0xFC000 = OBJECT_TRANS_ENERGY
        | OBJECT_TRANS_STEAM
        | OBJECT_TRANS_GLASS
        | OBJECT_TRANS_WALL
        | OBJECT_TRANS_NONE
        | OBJECT_TRANS_RED,
    OBJECT_OPEN_DOOR = OBJECT_SHOOT_THRU | OBJECT_LIGHT_THRU | OBJECT_NO_BLOCK,
}

enum LoadSaveMode {
    // Special case - loading game from main menu.
    LOAD_SAVE_MODE_FROM_MAIN_MENU = 0,

    // Normal (full-screen) save/load screen.
    LOAD_SAVE_MODE_NORMAL = 1,

    // Quick load/save.
    LOAD_SAVE_MODE_QUICK = 2,
}

enum Dam {
    DAM_KNOCKED_OUT = 0x01,
    DAM_KNOCKED_DOWN = 0x02,
    DAM_CRIP_LEG_LEFT = 0x04,
    DAM_CRIP_LEG_RIGHT = 0x08,
    DAM_CRIP_ARM_LEFT = 0x10,
    DAM_CRIP_ARM_RIGHT = 0x20,
    DAM_BLIND = 0x40,
    DAM_DEAD = 0x80,
    DAM_HIT = 0x100,
    DAM_CRITICAL = 0x200,
    DAM_ON_FIRE = 0x400,
    DAM_BYPASS = 0x800,
    DAM_EXPLODE = 0x1000,
    DAM_DESTROY = 0x2000,
    DAM_DROP = 0x4000,
    DAM_LOSE_TURN = 0x8000,
    DAM_HIT_SELF = 0x10000,
    DAM_LOSE_AMMO = 0x20000,
    DAM_DUD = 0x40000,
    DAM_HURT_SELF = 0x80000,
    DAM_RANDOM_HIT = 0x100000,
    DAM_CRIP_RANDOM = 0x200000,
    DAM_BACKWASH = 0x400000,
    DAM_PERFORM_REVERSE = 0x800000,
    DAM_CRIP_LEG_ANY = DAM_CRIP_LEG_LEFT | DAM_CRIP_LEG_RIGHT,
    DAM_CRIP_ARM_ANY = DAM_CRIP_ARM_LEFT | DAM_CRIP_ARM_RIGHT,
    DAM_CRIP = DAM_CRIP_LEG_ANY | DAM_CRIP_ARM_ANY | DAM_BLIND,
}

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
    if (gameInitWithOptions("FALLOUT II", false, 0, 0, argc, argv) == -1) {
        return false;
    }

    if (_main_selfrun_list != NULL) {
        _main_selfrun_exit();
    }

    if (selfrunInitFileList(&_main_selfrun_list, &_main_selfrun_count) == 0) {
        _main_selfrun_index = 0;
    }

    return true;
}

// NOTE: Inlined.
//
// 0x480D0C
fn main_reset_system() -> u32 {
    gameReset();

    return 1;
}

// NOTE: Inlined.
//
// 0x480D18
fn main_exit_system() {
    backgroundSoundDelete();

    // NOTE: Uninline.
    _main_selfrun_exit();

    gameExit();
}

// 0x480D4C
fn _main_load_new(mapFileName: str) -> i32 {
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
    return 0;
}

// NOTE: Inlined.
//
// 0x480DF8
fn main_loadgame_new() -> i32 {
    _game_user_wants_to_quit = 0;
    _main_show_death_scene = 0;

    gDude::flags &= /*~*/!ObjectFlags::OBJECT_FLAT;

    objectShow(gDude, NULL);
    mouseHideCursor();

    _map_init();

    gameMouseSetCursor(MouseCursorType::MOUSE_CURSOR_NONE);
    mouseShowCursor();

    return 0;
}

// 0x480E34
fn main_unload_new() {
    objectHide(gDude, NULL);
    _map_exit();
}

// 0x480E48
fn mainLoop() {
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

// 0x480F38
fn _main_selfrun_exit() {
    if (_main_selfrun_list != NULL) {
        selfrunFreeFileList(&_main_selfrun_list);
    }

    _main_selfrun_count = 0;
    _main_selfrun_index = 0;
    _main_selfrun_list = NULL;
}

// 0x480F64
fn _main_selfrun_record() {
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

// 0x48109C
fn _main_selfrun_play() {
    if (!gMainMenuScreensaverCycle && _main_selfrun_count > 0) {
        let selfrunData: SelfrunData;
        if (selfrunPreparePlayback(_main_selfrun_list[_main_selfrun_index], &selfrunData) == 0) {
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

// 0x48118C
fn showDeath() {
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
                if (keyCode > -1 || !_main_death_voiceover_done || getTicksSince(time) > delay) {
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

// 0x4814A8
fn _main_death_voiceover_callback() {
    _main_death_voiceover_done = true;
}

// Read endgame subtitle.
//
// 0x4814B4
fn _mainDeathGrabTextFile(fileName: str, dest: str) -> i32 {
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

    if (wordWrap(text, width, beginnings, count) == -1) {
        return -1;
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
