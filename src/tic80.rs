#![allow(unused)]

//struct MouseData {
//    short x; short y;
//    byte scrollx; byte scrolly;
//    bool left; bool middle; bool right;
//}

pub const WIDTH: u32 = 240;
pub const HEIGHT: u32 = 136;

// These are pointers.
pub static mut FRAMEBUFFER_PTR: *mut [u8; 6400] = 0x00 as *mut [u8; 6400];
//const TILES_PTR = cast(ubyte*)0x4000;
//const SPRITES_PTR = cast(ubyte*)0x6000;
//const MAP_PTR = cast(ubyte*)0x8000;
//const GAMEPADS_PTR = cast(ubyte*)0xFF80;
//const MOUSE_PTR = cast(ubyte*)0xFF84;
//const KEYBOARD_PTR = cast(ubyte*)0xFF88;
//const SFX_STATE_PTR = cast(ubyte*)0xFF8C;
//const SOUND_REGISTERS_PTR = cast(ubyte*)0xFF9C;
//const WAVEFORMS_PTR = cast(ubyte*)0xFFE4;
//const SFX_PTR = cast(ubyte*)0x100E4;
//const MUSIC_PATTERNS_PTR = cast(ubyte*)0x11164;
//const MUSIC_TRACKS_PTR = cast(ubyte*)0x13E64;
//const SOUND_STATE_PTR = cast(ubyte*)0x13FFC;
//const STEREO_VOLUME_PTR = cast(ubyte*)0x14000;
//const PERSISTENT_MEMORY_PTR = cast(ubyte*)0x14004;
//const SPRITE_FLAGS_PTR = cast(ubyte*)0x14404;
//const SYSTEM_FONT_PTR = cast(ubyte*)0x14604;
//const WASM_FREE_RAM_PTR = cast(ubyte*)0x18000;
//
//// These are bounded arrays.
//ubyte[] FRAMEBUFFER() { return (cast(ubyte*)0)[0..16319]; } // VRAM bank 0 screen area
//ubyte[] TILES() { return (cast(ubyte*)0x4000)[0..8191]; }
//ubyte[] SPRITES() { return (cast(ubyte*)0x4000)[0..8191]; }
//ubyte[] MAP() { return (cast(ubyte*)0x8000)[0..32639]; }
//ubyte[] GAMEPADS() { return (cast(ubyte*)0xFF80)[0..3]; }
//ubyte[] MOUSE() { return (cast(ubyte*)0xFF84)[0..3]; }
//ubyte[] KEYBOARD() { return (cast(ubyte*)0xFF88)[0..3]; }
//ubyte[] SFX_STATE() { return (cast(ubyte*)0xFF8C)[0..15]; }
//ubyte[] SOUND_REGISTERS() { return (cast(ubyte*)0xFF9C)[0..71]; }
//ubyte[] WAVEFORMS() { return (cast(ubyte*)0xFFE4)[0..255]; }
//ubyte[] SFX() { return (cast(ubyte*)0x100E4)[0..4223]; }
//ubyte[] MUSIC_PATTERNS() { return (cast(ubyte*)0x11164)[0..11519]; }
//ubyte[] MUSIC_TRACKS() { return (cast(ubyte*)0x13E64)[0..407]; }
//ubyte[] SOUND_STATE() { return (cast(ubyte*)0x13FFC)[0..3]; }
//ubyte[] STEREO_VOLUME() { return (cast(ubyte*)0x14000)[0..3]; }
//ubyte[] PERSISTENT_MEMORY() { return (cast(ubyte*)0x14004)[0..1023]; }
//ubyte[] SPRITE_FLAGS() { return (cast(ubyte*)0x14404)[0..511]; }
//ubyte[] SYSTEM_FONT() { return (cast(ubyte*)0x14604)[0..2047]; }
//ubyte[] WASM_FREE_RAM() { return (cast(ubyte*)0x18000)[0..163839]; } // 160kb

//int btn(int id);
pub fn btn(id: i32) -> bool {
    unsafe { extern_btn(id) > 0 }
}
extern "C" {
    #[link_name = "btn"]
    fn extern_btn(id: i32) -> i32;
}

//bool btnp(int id, int hold, int period);
//void circ(int x, int y, int radius, int color);
//void circb(int x, int y, int radius, int color);
//void clip(int x, int y, int w, int h);

//void cls(int color);
pub fn cls(color: i8) {
    unsafe { extern_cls(color) }
}
extern "C" {
    #[link_name = "cls"]
    fn extern_cls(color: i8);
}

//void exit();
//void elli(int x, int y, int a, int b, int color);
//void ellib(int x, int y, int a, int b, int color);
//bool fget(int id, ubyte flag);
//int font(char* text, int x, int y, ubyte transcolors, int colorcount, int width, int height, bool fixed, int scale);
//bool fset(int id, ubyte flag, bool value);
//bool key(int keycode);
//bool keyp(int keycode, int hold, int period);
//void line(float x0, float y0, float x1, float y1, byte color);
//void map(int x, int y, int w, int h, int sx, int sy, ubyte transcolors, int colorcount, int scale, int remap);
//void memcpy(uint copyto, uint copyfrom, uint length);
//void memset(uint addr, ubyte value, uint length);
//int mget(int x, int y);
//void mouse(MouseData* data);
//void mset(int x, int y, bool value);
//void music(int track, int frame, int row, bool loop, bool sustain, int tempo, int speed);
//ubyte peek(int addr, int bits);
//ubyte peek4(uint addr4);
//ubyte peek2(uint addr2);
//ubyte peek1(uint bitaddr);
//void pix(int x, int y, int color);
//uint pmem(uint index, uint value);
//void poke(int addr, byte value, byte bits);
//void poke4(int addr4, byte value);
//void poke2(int addr2, byte value);
//void poke1(int bitaddr, byte value);

//int print(const char* txt, int x, int y, int color, int fixed, int scale, int alt);
pub fn print<T: AsRef<str>>(
    txt: T,
    x: i32,
    y: i32,
    color: i8,
    fixed: bool,
    scale: i8,
    smallfont: bool,
) -> i32 {
    let txt_ref = txt.as_ref();
    let fixed = if fixed { 1 } else { 0 };
    let smallfont = if smallfont { 1 } else { 0 };
    unsafe { extern_print(txt_ref.as_ptr(), x, y, color, fixed, scale, smallfont) }
}
extern "C" {
    #[link_name = "print"]
    fn extern_print(
        txt: *const u8,
        x: i32,
        y: i32,
        color: i8,
        fixed: i8,
        scale: i8,
        smallfont: i8,
    ) -> i32;
}

//void rect(int x, int y, int w, int h, int color);
//void rectb(int x, int y, int w, int h, int color);
//void reset();
//void sfx(int id, int note, int octave, int duration, int channel, int volumeLeft, int volumeRight, int speed);

//void spr(int id, int x, int y, uint* transcolors, uint colorcount, int scale, int flip, int rotate, int w, int h);
pub fn spr(
    id: i32,
    x: i32,
    y: i32,
    transcolors: Option<&[u8]>,
    colorcount: i8,
    scale: i32,
    flip: i32,
    rotate: i32,
    w: i32,
    h: i32,
) {
    let empty = Vec::new();
    let transcolors = transcolors.unwrap_or(empty.as_ref());
    unsafe {
        extern_spr(
            id,
            x,
            y,
            transcolors.as_ptr(),
            colorcount,
            scale,
            flip,
            rotate,
            w,
            h,
        )
    }
}
extern "C" {
    #[link_name = "spr"]
    fn extern_spr(
        id: i32,
        x: i32,
        y: i32,
        transcolors: *const u8,
        colorcount: i8,
        scale: i32,
        flip: i32,
        rotate: i32,
        w: i32,
        h: i32,
    );
}

//void sync(int mask, int bank, bool tocart);
//void trace(const char* txt, int color);
//void ttri(float x1, float y1, float x2, float y2, float x3, float y3, float u1, float v1, float u2, float v2, float u3, float v3, int texsrc, ubyte transcolors, int colorcount, float z1, float z2, float z3, bool persp);
//void tri(float x1, float y1, float x2, float y2, float x3, float y3, int color);
//void trib(float x1, float y1, float x2, float y2, float x3, float y3, int color);
//float time();
//int tstamp();
//int vbank(int bank);
