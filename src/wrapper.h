#include <stddef.h>
#include <stdio.h>

#include <zlib.h>

namespace fallout {

/*
    from autorun.h
*/
bool autorunMutexCreate();
void autorunMutexClose();

/*
    from color.h
*/
bool colorPaletteLoad(const char* path);

/*
    from palette.h
*/
void paletteFadeTo(unsigned char* palette);

/*
    from debug.h
*/
int debugPrint(const char* format, ...);

/*
from game_movie.h
*/
typedef enum GameMovieFlags {
    GAME_MOVIE_FADE_IN = 0x01,
    GAME_MOVIE_FADE_OUT = 0x02,
    GAME_MOVIE_STOP_MUSIC = 0x04,
    GAME_MOVIE_PAUSE_MUSIC = 0x08,
} GameMovieFlags;

typedef enum GameMovie {
    MOVIE_IPLOGO,
    MOVIE_INTRO,
    MOVIE_ELDER,
    MOVIE_VSUIT,
    MOVIE_AFAILED,
    MOVIE_ADESTROY,
    MOVIE_CAR,
    MOVIE_CARTUCCI,
    MOVIE_TIMEOUT,
    MOVIE_TANKER,
    MOVIE_ENCLAVE,
    MOVIE_DERRICK,
    MOVIE_ARTIMER1,
    MOVIE_ARTIMER2,
    MOVIE_ARTIMER3,
    MOVIE_ARTIMER4,
    MOVIE_CREDITS,
    MOVIE_COUNT,
} GameMovie;

int gameMoviePlay(int movie, int flags);

/*
from loadsave.h
*/
typedef enum LoadSaveMode {
    // Special case - loading game from main menu.
    LOAD_SAVE_MODE_FROM_MAIN_MENU,

    // Normal (full-screen) save/load screen.
    LOAD_SAVE_MODE_NORMAL,

    // Quick load/save.
    LOAD_SAVE_MODE_QUICK,
} LoadSaveMode;

int lsgLoadGame(int mode);

/*
from mainmenu.h
*/
typedef enum MainMenuOption {
    MAIN_MENU_INTRO,
    MAIN_MENU_NEW_GAME,
    MAIN_MENU_LOAD_GAME,
    MAIN_MENU_SCREENSAVER,
    MAIN_MENU_TIMEOUT,
    MAIN_MENU_CREDITS,
    MAIN_MENU_QUOTES,
    MAIN_MENU_EXIT,
    MAIN_MENU_SELFRUN,
    MAIN_MENU_OPTIONS,
} MainMenuOption;

void mainMenuWindowFree();
void mainMenuWindowHide(bool animate);
int mainMenuWindowHandleEvents();
int mainMenuWindowInit();
void mainMenuWindowUnhide(bool animate);

/*
from svga.h
*/
int screenGetWidth();
int screenGetHeight();
void renderPresent();

/*
from fps_limiter.h
*/
class FpsLimiter {
public:
    FpsLimiter(unsigned int fps = 60);
    void mark();
    void throttle() const;

private:
    const unsigned int _fps;
    unsigned int _ticks;
};

/*
from sfall_global_scripts.h
*/
void sfall_gl_scr_exec_start_proc();
void sfall_gl_scr_process_main();

/*
from window_manager.h
*/
typedef enum WindowFlags {
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
} WindowFlags;

int windowCreate(int x, int y, int width, int height, int color, int flags);
void windowDestroy(int win);
unsigned char* windowGetBuffer(int win);
void windowRefresh(int win);

/*
from obj_types.h
*/
typedef enum Dam {
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
} Dam;

typedef enum ObjectFlags {
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
    OBJECT_FLAG_0xFC000 = OBJECT_TRANS_ENERGY | OBJECT_TRANS_STEAM | OBJECT_TRANS_GLASS | OBJECT_TRANS_WALL | OBJECT_TRANS_NONE | OBJECT_TRANS_RED,
    OBJECT_OPEN_DOOR = OBJECT_SHOOT_THRU | OBJECT_LIGHT_THRU | OBJECT_NO_BLOCK,
} ObjectFlags;

typedef struct Object Object;

typedef struct InventoryItem {
    Object* item;
    int quantity;
} InventoryItem;

// Represents inventory of the object.
typedef struct Inventory {
    int length;
    int capacity;
    InventoryItem* items;
} Inventory;
typedef struct WeaponObjectData {
    int ammoQuantity; // obj_pudg.pudweapon.cur_ammo_quantity
    int ammoTypePid; // obj_pudg.pudweapon.cur_ammo_type_pid
} WeaponObjectData;

typedef struct AmmoItemData {
    int quantity; // obj_pudg.pudammo.cur_ammo_quantity
} AmmoItemData;

typedef struct MiscItemData {
    int charges; // obj_pudg.pudmisc_item.curr_charges
} MiscItemData;

typedef struct KeyItemData {
    int keyCode; // obj_pudg.pudkey_item.cur_key_code
} KeyItemData;

typedef union ItemObjectData {
    WeaponObjectData weapon;
    AmmoItemData ammo;
    MiscItemData misc;
    KeyItemData key;
} ItemObjectData;

typedef struct CritterCombatData {
    int maneuver; // obj_pud.combat_data.maneuver
    int ap; // obj_pud.combat_data.curr_mp
    int results; // obj_pud.combat_data.results
    int damageLastTurn; // obj_pud.combat_data.damage_last_turn
    int aiPacket; // obj_pud.combat_data.ai_packet
    int team; // obj_pud.combat_data.team_num
    union {
        Object* whoHitMe; // obj_pud.combat_data.who_hit_me
        int whoHitMeCid;
    };
} CritterCombatData;

typedef struct CritterObjectData {
    int field_0; // obj_pud.reaction_to_pc
    CritterCombatData combat; // obj_pud.combat_data
    int hp; // obj_pud.curr_hp
    int radiation; // obj_pud.curr_rad
    int poison; // obj_pud.curr_poison
} CritterObjectData;

typedef struct DoorSceneryData {
    int openFlags; // obj_pudg.pudportal.cur_open_flags
} DoorSceneryData;

typedef struct StairsSceneryData {
    int destinationMap; // obj_pudg.pudstairs.destMap
    int destinationBuiltTile; // obj_pudg.pudstairs.destBuiltTile
} StairsSceneryData;

typedef struct ElevatorSceneryData {
    int type;
    int level;
} ElevatorSceneryData;

typedef struct LadderSceneryData {
    int destinationMap;
    int destinationBuiltTile;
} LadderSceneryData;

typedef union SceneryObjectData {
    DoorSceneryData door;
    StairsSceneryData stairs;
    ElevatorSceneryData elevator;
    LadderSceneryData ladder;
} SceneryObjectData;

typedef struct MiscObjectData {
    int map;
    int tile;
    int elevation;
    int rotation;
} MiscObjectData;

typedef struct ObjectData {
    Inventory inventory;
    union {
        CritterObjectData critter;
        struct {
            int flags;
            union {
                ItemObjectData item;
                SceneryObjectData scenery;
                MiscObjectData misc;
            };
        };
    };
} ObjectData;

typedef struct Object {
    int id; // obj_id
    int tile; // obj_tile_num
    int x; // obj_x
    int y; // obj_y
    int sx; // obj_sx
    int sy; // obj_sy
    int frame; // obj_cur_frm
    int rotation; // obj_cur_rot
    int fid; // obj_fid
    int flags; // obj_flags
    int elevation; // obj_elev
    union {
        int field_2C_array[14];
        ObjectData data;
    };
    int pid; // obj_pid
    int cid; // obj_cid
    int lightDistance; // obj_light_distance
    int lightIntensity; // obj_light_intensity
    int outline; // obj_outline
    int sid; // obj_sid
    Object* owner;
    int field_80;
} Object;

/*
from game_mouse.h
*/
typedef enum MouseCursorType {
    MOUSE_CURSOR_NONE,
    MOUSE_CURSOR_ARROW,
    MOUSE_CURSOR_SMALL_ARROW_UP,
    MOUSE_CURSOR_SMALL_ARROW_DOWN,
    MOUSE_CURSOR_SCROLL_NW,
    MOUSE_CURSOR_SCROLL_N,
    MOUSE_CURSOR_SCROLL_NE,
    MOUSE_CURSOR_SCROLL_E,
    MOUSE_CURSOR_SCROLL_SE,
    MOUSE_CURSOR_SCROLL_S,
    MOUSE_CURSOR_SCROLL_SW,
    MOUSE_CURSOR_SCROLL_W,
    MOUSE_CURSOR_SCROLL_NW_INVALID,
    MOUSE_CURSOR_SCROLL_N_INVALID,
    MOUSE_CURSOR_SCROLL_NE_INVALID,
    MOUSE_CURSOR_SCROLL_E_INVALID,
    MOUSE_CURSOR_SCROLL_SE_INVALID,
    MOUSE_CURSOR_SCROLL_S_INVALID,
    MOUSE_CURSOR_SCROLL_SW_INVALID,
    MOUSE_CURSOR_SCROLL_W_INVALID,
    MOUSE_CURSOR_CROSSHAIR,
    MOUSE_CURSOR_PLUS,
    MOUSE_CURSOR_DESTROY,
    MOUSE_CURSOR_USE_CROSSHAIR,
    MOUSE_CURSOR_WATCH,
    MOUSE_CURSOR_WAIT_PLANET,
    MOUSE_CURSOR_WAIT_WATCH,
    MOUSE_CURSOR_TYPE_COUNT,
    FIRST_GAME_MOUSE_ANIMATED_CURSOR = MOUSE_CURSOR_WAIT_PLANET,
} MouseCursorType;

int gameMouseSetCursor(int cursor);
int mouseGetEvent();

/*
from mouse.h
*/
void mouseShowCursor();
void mouseHideCursor();
bool cursorIsHidden();

/*
from map.h
*/
void _map_init();
void _map_exit();
int mapHandleTransition();

/*
from game_sound.h
*/
typedef void(SoundEndCallback)();

void backgroundSoundDelete();
int _gsound_speech_play_preloaded();
void speechDelete();
int speechLoad(const char* fname, int a2, int a3, int a4);
void speechSetEndCallback(SoundEndCallback* callback);
int _gsound_background_play_level_music(const char* a1, int a2);

/*
from credits.h
*/
void creditsOpen(const char* path, int fid, bool useReversedStyle);

/*
from preferences.h
*/
int doPreferences(bool animated);

/*
from endgame.h
*/
typedef enum EndgameDeathEndingReason {
    // Dude died.
    ENDGAME_DEATH_ENDING_REASON_DEATH = 0,

    // 13 years passed.
    ENDGAME_DEATH_ENDING_REASON_TIMEOUT = 2,
} EndgameDeathEndingReason;

void endgameSetupDeathEnding(int reason);
char* endgameDeathEndingGetFileName();

/*
from dfile.h
*/
typedef struct DBase DBase;
typedef struct DBaseEntry DBaseEntry;
typedef struct DFile DFile;

// A representation of .DAT file.
typedef struct DBase {
    // The path of .DAT file that this structure represents.
    char* path;

    // The offset to the beginning of data section of .DAT file.
    int dataOffset;

    // The number of entries.
    int entriesLength;

    // The array of entries.
    DBaseEntry* entries;

    // The head of linked list of open file handles.
    DFile* dfileHead;
} DBase;

typedef struct DBaseEntry {
    char* path;
    unsigned char compressed;
    int uncompressedSize;
    int dataSize;
    int dataOffset;
} DBaseEntry;

// A handle to open entry in .DAT file.
typedef struct DFile {
    DBase* dbase;
    DBaseEntry* entry;
    int flags;

    // The stream of .DAT file opened for reading in binary mode.
    //
    // This stream is not shared across open handles. Instead every [DFile]
    // opens it's own stream via [fopen], which is then closed via [fclose] in
    // [dfileClose].
    FILE* stream;

    // The inflate stream used to decompress data.
    //
    // This value is NULL if entry is not compressed.
    z_streamp decompressionStream;

    // The decompression buffer of size [DFILE_DECOMPRESSION_BUFFER_SIZE].
    //
    // This value is NULL if entry is not compressed.
    unsigned char* decompressionBuffer;

    // The last ungot character.
    //
    // See [DFILE_HAS_UNGETC] notes.
    int ungotten;

    // The last ungot compressed character.
    //
    // This value is used when reading compressed text streams to detect
    // Windows end of line sequence \r\n.
    int compressedUngotten;

    // The number of bytes read so far from compressed stream.
    //
    // This value is only used when reading compressed streams. The range is
    // 0..entry->dataSize.
    int compressedBytesRead;

    // The position in read stream.
    //
    // This value is tracked in terms of uncompressed data (even in compressed
    // streams). The range is 0..entry->uncompressedSize.
    long position;

    // Next [DFile] in linked list.
    //
    // [DFile]s are stored in [DBase] in reverse order, so it's actually a
    // previous opened file, not next.
    DFile* next;
} DFile;

/*
from file.h
*/
typedef enum XFileType {
    XFILE_TYPE_FILE,
    XFILE_TYPE_DFILE,
    XFILE_TYPE_GZFILE,
} XFileType;

typedef struct XFile {
    XFileType type;
    union {
        FILE* file;
        DFile* dfile;
        gzFile gzfile;
    };
} XFile;

/*
from db.h
*/
typedef struct File File;
int fileNameListInit(const char* pattern, char*** fileNames, int a3, int a4);
void fileNameListFree(char*** fileNames, int a2);
int fileClose(File* stream);
File* fileOpen(const char* filename, const char* mode);
int fileReadChar(File* stream);

/*
from game.h
*/
int gameHandleKey(int eventCode, bool isInCombatMode);
int gameInitWithOptions(const char* windowTitle, bool isMapper, int a3, int a4, int argc, char** argv);
void gameExit();
void gameReset();

/*
from input.h
*/
int inputGetInput();
void inputEventQueueReset();
unsigned int getTicks();
void inputPauseForTocks(unsigned int ms);
void inputBlockForTocks(unsigned int ms);

/*
from geometry.h
*/
typedef struct Rect {
    int left;
    int top;
    int right;
    int bottom;
} Rect;

/*
from object.h
*/
int objectHide(Object* obj, Rect* rect);
int objectShow(Object* obj, Rect* rect);

/*
from scripts.h
*/
int scriptsEnable();
int scriptsDisable();
int scriptsHandleRequests();

/*
from selfrun.h
*/
#define SELFRUN_RECORDING_FILE_NAME_LENGTH 13
#define SELFRUN_MAP_FILE_NAME_LENGTH 13
typedef struct SelfrunData {
    char recordingFileName[SELFRUN_RECORDING_FILE_NAME_LENGTH];
    char mapFileName[SELFRUN_MAP_FILE_NAME_LENGTH];
    int stopKeyCode;
} SelfrunData;

int selfrunInitFileList(char*** fileListPtr, int* fileListLengthPtr);
int selfrunFreeFileList(char*** fileListPtr);
int selfrunPreparePlayback(const char* fileName, SelfrunData* selfrunData);
int selfrunPrepareRecording(const char* recordingName, const char* mapFileName, SelfrunData* selfrunData);
void selfrunRecordingLoop(SelfrunData* selfrunData);
void selfrunPlaybackLoop(SelfrunData* selfrunData);

/*
from worldmap.h
*/
int wmMapMusicStart();
int mapLoadByName(char* fileName);

/*
from proto.h
*/
int _proto_dude_init(const char* path);

/*
from window_manager_private.h
*/
typedef void(ListSelectionHandler)(char** items, int index);

int _win_list_select(const char* title, char** fileList, int fileListLength, ListSelectionHandler* callback, int x, int y, int color);

/*
from cache.h
*/
typedef struct CacheEntry {
    int key;
    int size;
    unsigned char* data;
    unsigned int referenceCount;

    // Total number of hits that this cache entry received during it's
    // lifetime.
    unsigned int hits;

    unsigned int flags;

    // The most recent hit in terms of cache hit counter. Used to track most
    // recently used entries in eviction strategy.
    unsigned int mru;

    int heapHandleIndex;
} CacheEntry;

/*
from art.h
*/
class FrmImage {
public:
    FrmImage();
    ~FrmImage();

    bool isLocked() const { return _key != nullptr; }
    bool lock(unsigned int fid);
    void unlock();

    int getWidth() const { return _width; }
    int getHeight() const { return _height; }
    unsigned char* getData() const { return _data; }

private:
    CacheEntry* _key;
    unsigned char* _data;
    int _width;
    int _height;
};

int artCacheFlush();

/*
from draw.h
*/
void blitBufferToBuffer(unsigned char* src, int width, int height, int srcPitch, unsigned char* dest, int destPitch);
void bufferFill(unsigned char* buf, int width, int height, int pitch, int a5);

/*
from cycle.h
*/
void colorCycleDisable();
bool colorCycleEnabled();
void colorCycleEnable();

/*
from kb.h
*/
void keyboardReset();

/*
from random.h
*/
void randomSeedPrerandom(int seed);

/*
from character_selector.h
*/
int characterSelectorOpen();

/*
from word_wrap.h
*/
int wordWrap(const char* string, int width, short* breakpoints, short* breakpointsLengthPtr);

/*
from config.h
*/
typedef int(DictionaryReadProc)(FILE* stream, void* buffer, unsigned int size, int a4);
typedef int(DictionaryWriteProc)(FILE* stream, void* buffer, unsigned int size, int a4);
// NOTE: Last unnamed fields are likely seek, tell, and filelength.
typedef struct DictionaryIO {
    DictionaryReadProc* readProc;
    DictionaryWriteProc* writeProc;
    int field_8;
    int field_C;
    int field_10;
} DictionaryIO;

// A tuple containing individual key-value pair of a dictionary.
typedef struct DictionaryEntry {
    char* key;
    void* value;
} DictionaryEntry;

// A collection of key/value pairs.
//
// The keys in dictionary are always strings. Internally dictionary entries
// are kept sorted by the key. Both keys and values are copied when new entry
// is added to dictionary. For this reason the size of the value's type is
// provided during dictionary initialization.
typedef struct Dictionary {
    int marker;

    // The number of key/value pairs in the dictionary.
    int entriesLength;

    // The capacity of key/value pairs in [entries] array.
    int entriesCapacity;

    // The size of the dictionary values in bytes.
    size_t valueSize;

    // IO callbacks.
    DictionaryIO io;

    // The array of key-value pairs.
    DictionaryEntry* entries;
} Dictionary;

typedef Dictionary Config;
bool configGetString(Config* config, const char* sectionKey, const char* key, char** valuePtr);
bool configGetInt(Config* config, const char* sectionKey, const char* key, int* valuePtr, unsigned char base = 0);

}
