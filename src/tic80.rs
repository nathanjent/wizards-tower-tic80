#![allow(unused)]

use std::ffi::{CString, NulError};
use std::os::raw::c_char;

use crate::tic80_error::Tic80Error;

//struct MouseData {
//    short x; short y;
//    byte scrollx; byte scrolly;
//    bool left; bool middle; bool right;
//}
pub struct MouseData {
    x: i16,
    y: i16,
    scrollx: i8,
    scrolly: i8,
    left: bool,
    middle: bool,
    right: bool,
}

pub const WIDTH: u32 = 240;
pub const HEIGHT: u32 = 136;

// These are pointers with bounded arrays.
//
// VRAM bank 0 screen area
//const FRAMEBUFFER_PTR = cast(ubyte*)0;
//ubyte[] FRAMEBUFFER() { return (cast(ubyte*)0)[0..16319]; }
pub static mut FRAMEBUFFER_PTR: *mut [u8; 16320] = 0x00 as *mut [u8; 16320];

//const TILES_PTR = cast(ubyte*)0x4000;
//ubyte[] TILES() { return (cast(ubyte*)0x4000)[0..8191]; }
pub static mut TILES: *mut [u8; 8192] = 0x4000 as *mut [u8; 8192];

//const SPRITES_PTR = cast(ubyte*)0x6000;
//ubyte[] SPRITES() { return (cast(ubyte*)0x4000)[0..8191]; }
pub static mut SPRITES: *mut [u8; 8192] = 0x6000 as *mut [u8; 8192];

//const MAP_PTR = cast(ubyte*)0x8000;
//ubyte[] MAP() { return (cast(ubyte*)0x8000)[0..32639]; }
pub static mut MAP: *mut [u8; 32640] = 0x8000 as *mut [u8; 32640];

//const GAMEPADS_PTR = cast(ubyte*)0xFF80;
//ubyte[] GAMEPADS() { return (cast(ubyte*)0xFF80)[0..3]; }
pub static mut GAMEPADS: *mut [u8; 4] = 0xFF80 as *mut [u8; 4];

//const MOUSE_PTR = cast(ubyte*)0xFF84;
//ubyte[] MOUSE() { return (cast(ubyte*)0xFF84)[0..3]; }
pub static mut MOUSE: *mut [u8; 4] = 0xFF84 as *mut [u8; 4];

//const KEYBOARD_PTR = cast(ubyte*)0xFF88;
//ubyte[] KEYBOARD() { return (cast(ubyte*)0xFF88)[0..3]; }
pub static mut KEYBOARD: *mut [u8; 4] = 0xFF88 as *mut [u8; 4];

//const SFX_STATE_PTR = cast(ubyte*)0xFF8C;
//ubyte[] SFX_STATE() { return (cast(ubyte*)0xFF8C)[0..15]; }
pub static mut SFX_STATE: *mut [u8; 16] = 0xFF8C as *mut [u8; 16];

//const SOUND_REGISTERS_PTR = cast(ubyte*)0xFF9C;
//ubyte[] SOUND_REGISTERS() { return (cast(ubyte*)0xFF9C)[0..71]; }
pub static mut SOUND_REGISTERS: *mut [u8; 72] = 0xFF9C as *mut [u8; 72];

//const WAVEFORMS_PTR = cast(ubyte*)0xFFE4;
//ubyte[] WAVEFORMS() { return (cast(ubyte*)0xFFE4)[0..255]; }
pub static mut WAVEFORMS: *mut [u8; 256] = 0xFFE4 as *mut [u8; 256];

//const SFX_PTR = cast(ubyte*)0x100E4;
//ubyte[] SFX() { return (cast(ubyte*)0x100E4)[0..4223]; }
pub static mut SFX: *mut [u8; 4224] = 0x100E4 as *mut [u8; 4224];

//const MUSIC_PATTERNS_PTR = cast(ubyte*)0x11164;
//ubyte[] MUSIC_PATTERNS() { return (cast(ubyte*)0x11164)[0..11519]; }
pub static mut MUSIC_PATTERNS: *mut [u8; 11520] = 0x11164 as *mut [u8; 11520];

//const MUSIC_TRACKS_PTR = cast(ubyte*)0x13E64;
//ubyte[] MUSIC_TRACKS() { return (cast(ubyte*)0x13E64)[0..407]; }
pub static mut MUSIC_TRACKS: *mut [u8; 408] = 0x13E64 as *mut [u8; 408];

//const SOUND_STATE_PTR = cast(ubyte*)0x13FFC;
//ubyte[] SOUND_STATE() { return (cast(ubyte*)0x13FFC)[0..3]; }
pub static mut SOUND_STATE: *mut [u8; 4] = 0x13FFC as *mut [u8; 4];

//const STEREO_VOLUME_PTR = cast(ubyte*)0x14000;
//ubyte[] STEREO_VOLUME() { return (cast(ubyte*)0x14000)[0..3]; }
pub static mut STEREO_VOLUME: *mut [u8; 4] = 0x14000 as *mut [u8; 4];

//const PERSISTENT_MEMORY_PTR = cast(ubyte*)0x14004;
//ubyte[] PERSISTENT_MEMORY() { return (cast(ubyte*)0x14004)[0..1023]; }
pub static mut PERSISTENT_MEMORY: *mut [u8; 1024] = 0x14004 as *mut [u8; 1024];

//const SPRITE_FLAGS_PTR = cast(ubyte*)0x14404;
//ubyte[] SPRITE_FLAGS() { return (cast(ubyte*)0x14404)[0..511]; }
pub static mut SPRITE_FLAGS: *mut [u8; 512] = 0x14404 as *mut [u8; 512];

//const SYSTEM_FONT_PTR = cast(ubyte*)0x14604;
//ubyte[] SYSTEM_FONT() { return (cast(ubyte*)0x14604)[0..2047]; }
pub static mut SYSTEM_FONT: *mut [u8; 2048] = 0x14604 as *mut [u8; 2048];

//const WASM_FREE_RAM_PTR = cast(ubyte*)0x18000;
//ubyte[] WASM_FREE_RAM() { return (cast(ubyte*)0x18000)[0..163839]; } // 160kb
pub static mut WASM_FREE_RAM: *mut [u8; 163840] = 0x18000 as *mut [u8; 163840];

//int btn(int id);
pub fn btn(id: i32) -> bool {
    unsafe { extern_btn(id) > 0 }
}
extern "C" {
    #[link_name = "btn"]
    fn extern_btn(id: i32) -> i32;
}

//bool btnp(int id, int hold, int period);
pub fn btnp(id: i32, hold: i32, period: i32) -> bool {
    unsafe { extern_btnp(id, hold, period) > 0 }
}
extern "C" {
    #[link_name = "btnp"]
    fn extern_btnp(id: i32, hold: i32, period: i32) -> i32;
}

//void circ(int x, int y, int radius, int color);
pub fn circ(x: i32, y: i32, radius: i32, color: i32) {
    unsafe { extern_circ(x, y, radius, color) }
}
extern "C" {
    #[link_name = "circ"]
    fn extern_circ(x: i32, y: i32, radius: i32, color: i32);
}

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
pub fn map(
    x: Option<i32>,
    y: Option<i32>,
    w: Option<i32>,
    h: Option<i32>,
    sx: Option<i32>,
    sy: Option<i32>,
    transcolors: Option<&[u8]>,
    scale: Option<i8>,
    remap: Option<i32>,
) -> Result<(), Tic80Error> {
    unsafe {
        let colorcount = transcolors.map_or_else(|| 0, |t| t.len());
        extern_map(
            x.unwrap_or(0),
            y.unwrap_or(0),
            w.unwrap_or(1),
            h.unwrap_or(1),
            sx.unwrap_or(0),
            sy.unwrap_or(0),
            transcolors.map_or_else(|| std::ptr::null(), |c| c.as_ptr()),
            colorcount.try_into()?,
            scale.unwrap_or(0),
            remap.unwrap_or(0),
        )
    }
    Ok(())
}

extern "C" {
    #[link_name = "map"]
    fn extern_map(
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        sx: i32,
        sy: i32,
        transcolors: *const u8,
        colorcount: i8,
        scale: i8,
        remap: i32,
    );
}

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
pub fn print<T: Into<Vec<u8>>>(
    txt: T,
    x: i32,
    y: i32,
    color: i8,
    fixed: bool,
    scale: i8,
    smallfont: bool,
) -> Result<i32, Tic80Error> {
    let txt = CString::new(txt)?;
    let fixed = if fixed { 1 } else { 0 };
    let smallfont = if smallfont { 1 } else { 0 };
    unsafe {
        Ok(extern_print(
            txt.as_ptr(),
            x,
            y,
            color,
            fixed,
            scale,
            smallfont,
        ))
    }
}
extern "C" {
    #[link_name = "print"]
    fn extern_print(
        txt: *const c_char,
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
    scale: i32,
    flip: i32,
    rotate: i32,
    w: i32,
    h: i32,
) -> Result<(), Tic80Error> {
    let colorcount = transcolors.map_or_else(|| 0, |t| t.len());
    unsafe {
        extern_spr(
            id,
            x,
            y,
            transcolors.map_or_else(|| std::ptr::null(), |c| c.as_ptr()),
            colorcount.try_into()?,
            scale,
            flip,
            rotate,
            w,
            h,
        );
        Ok(())
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
pub fn trace<T: Into<Vec<u8>>>(message: T, color: Option<i8>) -> Result<(), Tic80Error> {
    let txt = CString::new(message)?;
    unsafe {
        extern_trace(txt.as_ptr(), color.unwrap_or(-1));
    }
    Ok(())
}
extern "C" {
    #[link_name = "trace"]
    fn extern_trace(txt: *const c_char, color: i8);
}

//void ttri(float x1, float y1, float x2, float y2, float x3, float y3, float u1, float v1, float u2, float v2, float u3, float v3, int texsrc, ubyte transcolors, int colorcount, float z1, float z2, float z3, bool persp);
pub fn ttri(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
    u1: f32,
    v1: f32,
    u2: f32,
    v2: f32,
    u3: f32,
    v3: f32,
    texsrc: i32,
    transcolors: Option<&[u8]>,
    z1: f32,
    z2: f32,
    z3: f32,
    depth: bool,
) -> Result<(), Tic80Error> {
    let colorcount = transcolors.map_or_else(|| 0, |t| t.len());
    unsafe {
        extern_ttri(
            x1,
            y1,
            x2,
            y2,
            x3,
            y3,
            u1,
            v1,
            u2,
            v2,
            u3,
            v3,
            texsrc,
            transcolors.map_or_else(|| std::ptr::null(), |c| c.as_ptr()),
            colorcount.try_into()?,
            z1,
            z2,
            z3,
            depth,
        )
    }
    Ok(())
}
extern "C" {
    #[link_name = "ttri"]
    fn extern_ttri(
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        u1: f32,
        v1: f32,
        u2: f32,
        v2: f32,
        u3: f32,
        v3: f32,
        texsrc: i32,
        transcolors: *const u8,
        colorcount: i8,
        z1: f32,
        z2: f32,
        z3: f32,
        depth: bool,
    );
}

//void tri(float x1, float y1, float x2, float y2, float x3, float y3, int color);
pub fn tri(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, color: i8) {
    unsafe { extern_tri(x1, y1, x2, y2, x3, y3, color) }
}
extern "C" {
    #[link_name = "tri"]
    fn extern_tri(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, color: i8);
}

//void trib(float x1, float y1, float x2, float y2, float x3, float y3, int color);
pub fn trib(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, color: i8) {
    unsafe { extern_trib(x1, y1, x2, y2, x3, y3, color) }
}
extern "C" {
    #[link_name = "trib"]
    fn extern_trib(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, color: i8);
}

//float time();
pub fn time() -> f32 {
    unsafe { extern_time() }
}
extern "C" {
    #[link_name = "time"]
    fn extern_time() -> f32;
}

//int tstamp();
pub fn tstamp() -> i32 {
    unsafe { extern_tstamp() }
}
extern "C" {
    #[link_name = "tstamp"]
    fn extern_tstamp() -> i32;
}

//int vbank(int bank);
pub fn vbank(bank: i8) -> i8 {
    unsafe { extern_vbank(bank) }
}
extern "C" {
    #[link_name = "vbank"]
    fn extern_vbank(bank: i8) -> i8;
}
