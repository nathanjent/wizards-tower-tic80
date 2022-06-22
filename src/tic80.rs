#![allow(unused)]

use derive_builder::Builder;
use heapless::Vec as Vector;

use std::ffi::{CStr, CString, NulError};
use std::intrinsics::transmute;
use std::ops::{Add, Deref};
use std::os::raw::c_char;

use crate::tic80_error::Tic80Error;

pub const WIDTH: u32 = 240;
pub const HEIGHT: u32 = 136;

// These are pointers with bounded arrays.

// VRAM bank 0 screen area
pub static mut FRAMEBUFFER_PTR: *mut [u8; 16320] = 0x00 as *mut [u8; 16320];
pub static mut TILES: *mut [u8; 8192] = 0x4000 as *mut [u8; 8192];
pub static mut SPRITES: *mut [u8; 8192] = 0x6000 as *mut [u8; 8192];
pub static mut MAP: *mut [u8; 32640] = 0x8000 as *mut [u8; 32640];
pub static mut GAMEPADS: *mut [u8; 4] = 0xFF80 as *mut [u8; 4];
pub static mut MOUSE: *mut [u8; 4] = 0xFF84 as *mut [u8; 4];
pub static mut KEYBOARD: *mut [u8; 4] = 0xFF88 as *mut [u8; 4];
pub static mut SFX_STATE: *mut [u8; 16] = 0xFF8C as *mut [u8; 16];
pub static mut SOUND_REGISTERS: *mut [u8; 72] = 0xFF9C as *mut [u8; 72];
pub static mut WAVEFORMS: *mut [u8; 256] = 0xFFE4 as *mut [u8; 256];
pub static mut SFX: *mut [u8; 4224] = 0x100E4 as *mut [u8; 4224];
pub static mut MUSIC_PATTERNS: *mut [u8; 11520] = 0x11164 as *mut [u8; 11520];
pub static mut MUSIC_TRACKS: *mut [u8; 408] = 0x13E64 as *mut [u8; 408];
pub static mut SOUND_STATE: *mut [u8; 4] = 0x13FFC as *mut [u8; 4];
pub static mut STEREO_VOLUME: *mut [u8; 4] = 0x14000 as *mut [u8; 4];
pub static mut PERSISTENT_MEMORY: *mut [u8; 1024] = 0x14004 as *mut [u8; 1024];
pub static mut SPRITE_FLAGS: *mut [u8; 512] = 0x14404 as *mut [u8; 512];
pub static mut SYSTEM_FONT: *mut [u8; 2048] = 0x14604 as *mut [u8; 2048];
pub static mut WASM_FREE_RAM: *mut [u8; 163840] = 0x18000 as *mut [u8; 163840];

/// [btn](https://github.com/nesbox/TIC-80/wiki/btn)
/// Returns true if the given button is pressed in the current frame.
pub fn btn(id: i32) -> bool {
    unsafe { extern_btn(id) > 0 }
}
/// [btn](https://github.com/nesbox/TIC-80/wiki/btn)
/// Returns the bits of the pressed buttons in the current frame.
/// For example if up pressed is 0b01 and down pressed is 0b10
/// then up and down pressed is 0b11.
pub fn btn_bits() -> i32 {
    unsafe { extern_btn(-1) }
}
extern "C" {
    #[link_name = "btn"]
    fn extern_btn(id: i32) -> i32;
}

#[derive(Builder)]
#[builder(name = "Btnp", build_fn(private))]
pub struct BtnpArgs {
    #[builder(setter(into), default = "-1")]
    pub id: i32,
    #[builder(setter(into), default = "-1")]
    pub hold: i32,
    #[builder(setter(into), default = "-1")]
    pub period: i32,
}

impl Btnp {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// [btnp](https://github.com/nesbox/TIC-80/wiki/btnp)
    /// Returns true if the given button was pressed in the previous frame.
    pub fn btnp(&self) -> bool {
        // Okay to unwrap because of default field values
        let args = self.build().unwrap();
        unsafe { extern_btnp(args.id, args.hold, args.period) > 0 }
    }

    /// [btnp](https://github.com/nesbox/TIC-80/wiki/btn)
    /// Returns the bits of the pressed buttons in the previous frame.
    pub fn btnp_bits(self) -> i32 {
        // Okay to unwrap because of default field values
        let args = self.build().unwrap();
        unsafe { extern_btnp(-1, args.hold, args.period) }
    }
}

extern "C" {
    #[link_name = "btnp"]
    fn extern_btnp(id: i32, hold: i32, period: i32) -> i32;
}

#[derive(Builder)]
#[builder(name = "Clip", build_fn(private))]
pub struct ClipArgs {
    #[builder(setter(into), default = "-1")]
    pub x: i32,
    #[builder(setter(into), default = "-1")]
    pub y: i32,
    #[builder(setter(into), default = "-1")]
    pub w: i32,
    #[builder(setter(into), default = "-1")]
    pub h: i32,
}

impl Clip {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// [clip](https://github.com/nesbox/TIC-80/wiki/clip)
    /// Unsets the clipping region.
    pub fn clip_reset() {
        unsafe { extern_clip(-1, -1, -1, -1) }
    }
    /// [clip](https://github.com/nesbox/TIC-80/wiki/clip)
    /// Sets the clipping region
    pub fn clip<T>(self) {
        // Okay to unwrap because of default field values
        let args = self.build().unwrap();
        unsafe { extern_clip(args.x, args.y, args.w, args.h) }
    }
}
extern "C" {
    #[link_name = "clip"]
    fn extern_clip(x: i32, y: i32, w: i32, h: i32);
}

/// [cls](https://github.com/nesbox/TIC-80/wiki/cls)
/// Clears the screen with the default color.
pub fn cls_default() {
    unsafe { extern_cls(-1) }
}
/// Clears the screen with color.
pub fn cls(color: i8) {
    unsafe { extern_cls(color) }
}
extern "C" {
    #[link_name = "cls"]
    fn extern_cls(color: i8);
}

/// [circ](https://github.com/nesbox/TIC-80/wiki/circ)
/// Draws circle with center at x,y.
pub fn circ(x: i32, y: i32, radius: i32, color: i8) {
    unsafe { extern_circ(x, y, radius, color) }
}
extern "C" {
    #[link_name = "circ"]
    fn extern_circ(x: i32, y: i32, radius: i32, color: i8);
}

/// [circb](https://github.com/nesbox/TIC-80/wiki/circb)
/// Draws circle border with center at x,y.
pub fn circb(x: i32, y: i32, radius: i32, color: i8) {
    unsafe { extern_circb(x, y, radius, color) }
}
extern "C" {
    #[link_name = "circb"]
    fn extern_circb(x: i32, y: i32, radius: i32, color: i8);
}

/// [elli](https://github.com/nesbox/TIC-80/wiki/elli)
/// Draws ellipse with center at x,y.
pub fn elli(x: i32, y: i32, a: i32, b: i32, color: i8) {
    unsafe { extern_elli(x, y, a, b, color) }
}
extern "C" {
    #[link_name = "elli"]
    fn extern_elli(x: i32, y: i32, a: i32, b: i32, color: i8);
}

/// [ellib](https://github.com/nesbox/TIC-80/wiki/ellib)
/// Draws ellipse border with center at x,y.
pub fn ellib(x: i32, y: i32, a: i32, b: i32, color: i8) {
    unsafe { extern_ellib(x, y, a, b, color) }
}
extern "C" {
    #[link_name = "ellib"]
    fn extern_ellib(x: i32, y: i32, a: i32, b: i32, color: i8);
}

/// [exit](https://github.com/nesbox/TIC-80/wiki/exit)
/// Exit to console after current TIC function ends.
pub fn exit() {
    unsafe { extern_exit() }
}
extern "C" {
    #[link_name = "exit"]
    fn extern_exit();
}

/// [fget](https://github.com/nesbox/TIC-80/wiki/fget)
/// Returns `true` if the specified flag of the sprite is set.
pub fn fget(id: i32, flag: i8) {
    unsafe { extern_fget(id, flag) }
}
extern "C" {
    #[link_name = "fget"]
    fn extern_fget(id: i32, flag: i8);
}

/// [fset](https://github.com/nesbox/TIC-80/wiki/fset)
/// Sets the sprite flag to the given value.
pub fn fset(id: i32, flag: i8, value: bool) {
    unsafe { extern_fset(id, flag, value) }
}
extern "C" {
    #[link_name = "fset"]
    fn extern_fset(id: i32, flag: i8, value: bool);
}

pub struct Colors {
    colors: Vector<u8, 15>,
}

impl Colors {
    pub fn new() -> Self {
        Self {
            colors: Vector::new(),
        }
    }

    pub fn with_color(value: u8) -> Self {
        let mut colors = Vector::new();
        colors.push(value);
        Self { colors }
    }

    pub fn and_color(&mut self, value: u8) -> &mut Self {
        self.colors.push(value);
        self
    }
}

impl<'a> Into<&'a [u8]> for &'a Colors {
    fn into(self) -> &'a [u8] {
        self.colors.as_ref()
    }
}

impl<'a> Into<Colors> for &'a [u8] {
    fn into(self) -> Colors {
        let mut colors = Vector::new();
        colors.extend_from_slice(self);
        Colors { colors }
    }
}

#[derive(Builder)]
#[builder(name = "Font", build_fn(private))]
struct FontArgs<'a> {
    #[builder(setter(into), default = "&[]")]
    transparent_colors: &'a [u8],
    #[builder(setter(into), default = "-1")]
    width: i8,
    #[builder(setter(into), default = "-1")]
    height: i8,
    #[builder(setter(into), default = "false")]
    fixed: bool,
    #[builder(setter(into), default = "-1")]
    scale: i8,
}

impl<'a> Font<'a> {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// [font](https://github.com/nesbox/TIC-80/wiki/font)
    /// Draw text to the screen using the foreground spritesheet as the font.
    pub fn font<T>(self, text: T, x: i32, y: i32) -> Result<i32, Tic80Error>
    where
        T: Into<Vec<u8>>,
    {
        // Okay to unwrap because of default field values
        let args = self.build().unwrap();
        unsafe {
            let text = CString::new(text)?;
            let colorcount = args.transparent_colors.len().try_into().unwrap_or(-1);
            let transparent_colors = args.transparent_colors.as_ptr();
            let text_width = extern_font(
                text.as_ptr(),
                x,
                y,
                transparent_colors,
                colorcount,
                args.width,
                args.height,
                args.fixed,
                args.scale,
                false,
            );
            Ok(text_width)
        }
    }
}
extern "C" {
    #[link_name = "font"]
    fn extern_font(
        text: *const c_char,
        x: i32,
        y: i32,
        transcolors: *const u8,
        colorcount: i8,
        w: i8,
        h: i8,
        fixed: bool,
        scale: i8,
        alt: bool,
    ) -> i32;
}

/// [key](https://github.com/nesbox/TIC-80/wiki/key)
/// Returns `true` if the key denoted by `keycode` is pressed in the current frame.
pub fn key(keycode: i32) -> bool {
    unsafe { extern_key(keycode) > 0 }
}
/// [key](https://github.com/nesbox/TIC-80/wiki/key)
/// Returns the bits of the pressed keys in the current frame.
pub fn key_bit() -> i32 {
    unsafe { extern_key(-1) }
}
extern "C" {
    #[link_name = "key"]
    fn extern_key(keycode: i32) -> i32;
}

#[derive(Builder)]
#[builder(name = "Keyp", build_fn(private))]
pub struct KeypArgs {
    #[builder(setter(into), default = "-1")]
    id: i32,
    #[builder(setter(into), default = "-1")]
    hold: i32,
    #[builder(setter(into), default = "-1")]
    period: i32,
}

impl KeypArgs {
    /// [keyp](https://github.com/nesbox/TIC-80/wiki/keyp)
    /// Returns `true` if the key denoted by `keycode` is pressed in the previous frame.
    pub fn keyp(self) -> bool {
        unsafe { extern_keyp(self.id, self.hold, self.period) > 0 }
    }
    /// [keyp](https://github.com/nesbox/TIC-80/wiki/keyp)
    /// Returns the bits of the pressed keys in the previous frame.
    pub fn keyp_bit(self) -> i32 {
        unsafe { extern_keyp(-1, self.hold, self.period) }
    }
}
extern "C" {
    #[link_name = "keyp"]
    fn extern_keyp(id: i32, hold: i32, period: i32) -> i32;
}

/// [line](https://github.com/nesbox/TIC-80/wiki/line)
/// Draws a straight line from point (x0,y0) to point (x1,y1) in the specified color.
pub fn line(x0: f32, y0: f32, x1: f32, y1: f32, color: i8) {
    unsafe { extern_line(x0, y0, x1, y1, color) }
}
extern "C" {
    #[link_name = "line"]
    fn extern_line(x0: f32, y0: f32, x1: f32, y1: f32, color: i8);
}

/// [map](https://github.com/nesbox/TIC-80/wiki/map)
/// Draw the desired area of the map to a specified screen position.
pub fn map_default() {
    unsafe { extern_map(-1, -1, -1, -1, -1, -1, std::ptr::null(), 0, -1, 0) }
}

#[derive(Builder)]
#[builder(name = "Map", build_fn(private))]
pub struct MapArgs<'a> {
    #[builder(setter(into), default = "-1")]
    x: i32,
    #[builder(setter(into), default = "-1")]
    y: i32,
    #[builder(setter(into), default = "-1")]
    w: i32,
    #[builder(setter(into), default = "-1")]
    h: i32,
    #[builder(setter(into), default = "-1")]
    sx: i32,
    #[builder(setter(into), default = "-1")]
    sy: i32,
    #[builder(setter(into), default = "&[]")]
    transparent_colors: &'a [u8],
    #[builder(setter(into), default = "-1")]
    scale: i8,
    #[builder(setter(into), default = "-1")]
    remap: i32,
}

impl<'a> Map<'a> {
    /// [map](https://github.com/nesbox/TIC-80/wiki/map)
    /// Draw the desired area of the map to a specified screen position.
    /// TODO: `remap` may not be supported yet
    pub fn map(self) {
        let args = self.build().unwrap();
        let colorcount = args.transparent_colors.len().try_into().unwrap_or(-1);
        let transparent_colors = args.transparent_colors.as_ptr();
        unsafe {
            extern_map(
                args.x,
                args.y,
                args.w,
                args.h,
                args.sx,
                args.sy,
                transparent_colors,
                colorcount,
                args.scale,
                args.remap,
            )
        }
    }
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

/// [memcpy](https://github.com/nesbox/TIC-80/wiki/memcpy)
/// Copies a continuous block of RAM from one address to another.
pub fn memcpy(to: i32, from: i32, length: i32) {
    unsafe { extern_memcpy(to, from, length) }
}
extern "C" {
    #[link_name = "memcpy"]
    fn extern_memcpy(to: i32, from: i32, length: i32);
}

/// [memset](https://github.com/nesbox/TIC-80/wiki/memset)
/// Sets a continuous block of RAM to the same value.
pub fn memset(address: i32, value: u8, length: u32) {
    unsafe { extern_memset(address, value.into(), length) }
}
extern "C" {
    #[link_name = "memset"]
    fn extern_memset(address: i32, value: i32, length: u32);
}

/// [mget](https://github.com/nesbox/TIC-80/wiki/mget)
/// Returns the tile id at the specified MAP coordinates.
pub fn mget(x: i32, y: i32) -> i32 {
    unsafe { extern_mget(x, y) }
}
extern "C" {
    #[link_name = "mget"]
    fn extern_mget(x: i32, y: i32) -> i32;
}

/// [mset](https://github.com/nesbox/TIC-80/wiki/mset)
/// Change the tile at the specified MAP coordinates.
/// See [`sync`] for persisting changes.
pub fn mset(x: i32, y: i32, tile_id: i32) {
    unsafe { extern_mset(x, y, tile_id) }
}
extern "C" {
    #[link_name = "mset"]
    fn extern_mset(x: i32, y: i32, tile_id: i32);
}

/// [mouse](https://github.com/nesbox/TIC-80/wiki/mouse)
/// Pass to the [`mouse`] function to populate.
#[repr(C)]
#[derive(Default)]
pub struct MouseData {
    x: i16,
    y: i16,
    scrollx: i8,
    scrolly: i8,
    left: bool,
    middle: bool,
    right: bool,
}

/// [mouse](https://github.com/nesbox/TIC-80/wiki/mouse)
/// Returns the mouse coordinates and a boolean value for the state of each mouse button, with true indicating that a button is pressed.
pub fn mouse(data: &mut MouseData) {
    unsafe { extern_mouse(&mut *data) }
}
extern "C" {
    #[link_name = "mouse"]
    fn extern_mouse(data: *mut MouseData);
}

/// [music](https://github.com/nesbox/TIC-80/wiki/music)
/// Starts playing a track created in the Music Editor.
pub fn music<T, U>(track: T, frame: T, row: T, loop_music: U, sustain: U, tempo: T, speed: T)
where
    T: Into<Option<i32>>,
    U: Into<Option<bool>>,
{
    unsafe {
        extern_music(
            track.into().unwrap_or(-1),
            frame.into().unwrap_or(-1),
            row.into().unwrap_or(-1),
            loop_music.into().unwrap_or(true),
            sustain.into().unwrap_or(false),
            tempo.into().unwrap_or(-1),
            speed.into().unwrap_or(-1),
        )
    }
}
extern "C" {
    #[link_name = "music"]
    fn extern_music(
        track: i32,
        frame: i32,
        row: i32,
        loop_music: bool,
        sustain: bool,
        tempo: i32,
        speed: i32,
    );
}

/// [peek](https://github.com/nesbox/TIC-80/wiki/peek)
/// Read directly from RAM specifying the number of bits to read.
pub fn peek<T>(address: i32, bits: T) -> i8
where
    T: Into<Option<i8>>,
{
    unsafe { extern_peek(address, bits.into().unwrap_or(8)) }
}
/// Read a byte (8 bits) directly from RAM.
pub fn peek8(address: i32) -> i8 {
    unsafe { extern_peek(address, -1) }
}
extern "C" {
    #[link_name = "peek"]
    fn extern_peek(address: i32, bits: i8) -> i8;
}

/// [peek4](https://github.com/nesbox/TIC-80/wiki/peek4)
/// Read a nibble (4 bits) directly from RAM.
pub fn peek4(address: i32) -> i8 {
    unsafe { extern_peek4(address) }
}
extern "C" {
    #[link_name = "peek4"]
    fn extern_peek4(address: i32) -> i8;
}

/// [peek2](https://github.com/nesbox/TIC-80/wiki/peek2)
/// Read a 2 bits directly from RAM.
pub fn peek2(address: i32) -> i8 {
    unsafe { extern_peek2(address) }
}
extern "C" {
    #[link_name = "peek2"]
    fn extern_peek2(address: i32) -> i8;
}

/// [peek1](https://github.com/nesbox/TIC-80/wiki/peek1)
/// Read a 1 bit directly from RAM.
pub fn peek1(address: i32) -> i8 {
    unsafe { extern_peek1(address) }
}
extern "C" {
    #[link_name = "peek1"]
    fn extern_peek1(address: i32) -> i8;
}

/// [pix](https://github.com/nesbox/TIC-80/wiki/pix)
/// Draw a pixel in the specified color.
pub fn pix_set(x: i32, y: i32, color: i8) {
    unsafe {
        extern_pix(x, y, color);
    }
}
/// [pix](https://github.com/nesbox/TIC-80/wiki/pix)
/// Retrieve a pixel's color.
pub fn pix_get(x: i32, y: i32) -> u8 {
    unsafe { extern_pix(x, y, -1) }
}
extern "C" {
    #[link_name = "pix"]
    fn extern_pix(x: i32, y: i32, color: i8) -> u8;
}

/// [pmem](https://github.com/nesbox/TIC-80/wiki/pmem)
/// Retrieve value from persistent memory.
pub fn pmem_get(index: i32) -> u32 {
    // -1 disables writing
    unsafe { extern_pmem(index, -1) }
}
/// [pmem](https://github.com/nesbox/TIC-80/wiki/pmem)
/// Save new value to persistent memory, retrieve prior value.
pub fn pmem_set(index: i32, value: i64) -> u32 {
    unsafe { extern_pmem(index, value) }
}
extern "C" {
    #[link_name = "pmem"]
    fn extern_pmem(address: i32, value: i64) -> u32;
}

/// [poke](https://github.com/nesbox/TIC-80/wiki/poke)
/// Write directly to RAM. Specifying the number of bits to write.
pub fn poke<T>(address: i32, value: i8, bits: T)
where
    T: Into<Option<i8>>,
{
    unsafe { extern_poke(address, value, bits.into().unwrap_or(8)) }
}
/// [poke](https://github.com/nesbox/TIC-80/wiki/poke)
/// Write a byte (8 bits) directly to RAM.
pub fn poke8(address: i32, value: i8) {
    unsafe { extern_poke(address, value, -1) }
}
extern "C" {
    #[link_name = "poke"]
    fn extern_poke(address: i32, value: i8, bits: i8);
}

/// [poke4](https://github.com/nesbox/TIC-80/wiki/poke4)
/// Write a nibble (4 bits) directly to RAM.
pub fn poke4(address: i32, value: u8) {
    unsafe { extern_poke4(address, value) }
}
extern "C" {
    #[link_name = "poke4"]
    fn extern_poke4(address: i32, value: u8);
}

/// [poke2](https://github.com/nesbox/TIC-80/wiki/poke2)
/// Write 2 bits directly to RAM.
pub fn poke2(address: i32, value: u8) {
    unsafe { extern_poke2(address, value) }
}
extern "C" {
    #[link_name = "poke2"]
    fn extern_poke2(address: i32, value: u8);
}

/// [poke1](https://github.com/nesbox/TIC-80/wiki/poke1)
/// Write 1 bit directly to RAM.
pub fn poke1(address: i32, value: u8) {
    unsafe { extern_poke1(address, value) }
}
extern "C" {
    #[link_name = "poke1"]
    fn extern_poke1(address: i32, value: u8);
}

#[derive(Builder, Default)]
#[builder(name = "Print", build_fn(private))]
pub struct PrintArgs {
    #[builder(setter(into), default = "-1")]
    x: i32,
    #[builder(setter(into), default = "-1")]
    y: i32,
    #[builder(setter(into), default = "-1")]
    color: i8,
    #[builder(setter(into), default = "false")]
    fixed: bool,
    #[builder(setter(into), default = "-1")]
    scale: i8,
    #[builder(setter(into), default = "false")]
    smallfont: bool,
}

impl Print {
    /// [print](https://github.com/nesbox/TIC-80/wiki/print)
    /// Print text to the screen using the font defined in config.
    pub fn print<S: AsRef<str>>(&self, text: S) -> i32 {
        // Okay to unwrap because of default field values
        let args = self.build().unwrap();
        let text = text.as_ref().as_ptr();
        unsafe {
            extern_print(
                text,
                args.x,
                args.y,
                args.color,
                if args.fixed { 1 } else { 0 },
                args.scale,
                if args.smallfont { 1 } else { 0 },
            )
        }
    }
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

/// [rect](https://github.com/nesbox/TIC-80/wiki/rect)
/// Draws a filled rectangle at the specified position.
pub fn rect(x: i32, y: i32, w: i32, h: i32, color: i32) {
    unsafe { extern_rect(x, y, w, h, color) }
}
extern "C" {
    #[link_name = "rect"]
    fn extern_rect(x: i32, y: i32, w: i32, h: i32, color: i32);
}

/// [rectb](https://github.com/nesbox/TIC-80/wiki/rectb)
/// Draws a one pixel thick rectangle border.
pub fn rectb(x: i32, y: i32, w: i32, h: i32, color: i32) {
    unsafe { extern_rectb(x, y, w, h, color) }
}
extern "C" {
    #[link_name = "rectb"]
    fn extern_rectb(x: i32, y: i32, w: i32, h: i32, color: i32);
}

/// [reset](https://github.com/nesbox/TIC-80/wiki/reset)
/// Resets the TIC virtual "hardware" and immediately restarts the cartridge.
pub fn reset() {
    unsafe { extern_reset() }
}
extern "C" {
    #[link_name = "reset"]
    fn extern_reset();
}

#[derive(Builder)]
#[builder(name = "Sfx", build_fn(private))]
struct SfxArgs {
    #[builder(setter(into), default = "-1")]
    note: i32,
    #[builder(setter(into), default = "-1")]
    octave: i32,
    #[builder(setter(into), default = "-1")]
    duration: i32,
    #[builder(setter(into), default = "-1")]
    channel: i32,
    #[builder(setter(into), default = "-1")]
    volume_left: i32,
    #[builder(setter(into), default = "-1")]
    volume_right: i32,
    #[builder(setter(into), default = "-1")]
    speed: i32,
}

impl Sfx {
    /// [sfx](https://github.com/nesbox/TIC-80/wiki/sfx)
    /// Play the sound with id created in the sfx editor.
    pub fn sfx(self, id: i32) {
        let args = self.build().unwrap();
        unsafe {
            extern_sfx(
                id,
                args.note,
                args.octave,
                args.duration,
                args.channel,
                args.volume_left,
                args.volume_right,
                args.speed,
            )
        }
    }
}
extern "C" {
    #[link_name = "sfx"]
    fn extern_sfx(
        id: i32,
        note: i32,
        octave: i32,
        duration: i32,
        channel: i32,
        volume_left: i32,
        volume_right: i32,
        speed: i32,
    );
}

#[derive(Builder, Clone)]
#[builder(name = "Spr", build_fn(private), pattern = "owned")]
pub struct SprArgs<'a> {
    #[builder(setter(into), default = "&[]")]
    transparent_colors: &'a [u8],
    #[builder(setter(into), default = "-1")]
    scale: i32,
    #[builder(setter(into), default = "-1")]
    flip: i32,
    #[builder(setter(into), default = "-1")]
    rotate: i32,
    #[builder(setter(into), default = "-1")]
    width: i32,
    #[builder(setter(into), default = "-1")]
    height: i32,
}

impl<'a> Spr<'a> {
    /// [spr](https://github.com/nesbox/TIC-80/wiki/spr)
    /// Draws the sprite number index at the x and y coordinate.
    pub fn spr(self, id: i32, x: i32, y: i32) {
        let args = self.build().unwrap();
        let colorcount = args.transparent_colors.len().try_into().unwrap_or(-1);
        let transparent_colors = args.transparent_colors.as_ptr();
        unsafe {
            extern_spr(
                id,
                x,
                y,
                transparent_colors,
                colorcount,
                args.scale,
                args.flip,
                args.rotate,
                args.width,
                args.height,
            );
        }
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

/// [sync](https://github.com/nesbox/TIC-80/wiki/sync)
/// Save cart data modified during runtime.
pub fn sync(mask: Option<i32>, bank: Option<i8>, to_cart: bool) {
    let to_cart = if to_cart { 1 } else { 0 };
    unsafe { extern_sync(mask.unwrap_or(-1), bank.unwrap_or(-1), to_cart) }
}
extern "C" {
    #[link_name = "sync"]
    fn extern_sync(mask: i32, bank: i8, to_cart: i8);
}

/// The source of the triangle's texture in #[ttri] calls.
#[derive(Clone)]
pub enum TextureSource {
    Sprites,
    Map,
}

/// [ttri](https://github.com/nesbox/TIC-80/wiki/ttri)
/// This function draws a triangle filled with texture from either SPRITES or MAP RAM or VBANK.
#[derive(Builder)]
#[builder(name = "Ttri", build_fn(private))]
pub struct TtriArgs<'a> {
    #[builder(setter(into), default = "TextureSource::Sprites")]
    texture_src: TextureSource,
    #[builder(setter(into), default = "&[]")]
    transparent_colors: &'a [u8],
    #[builder(setter(into, strip_option))]
    z1: Option<f32>,
    #[builder(setter(into, strip_option))]
    z2: Option<f32>,
    #[builder(setter(into, strip_option))]
    z3: Option<f32>,
}

impl<'a> Ttri<'a> {
    /// [ttri](https://github.com/nesbox/TIC-80/wiki/ttri)
    /// This function draws a triangle filled with texture from either SPRITES or MAP RAM or VBANK.
    pub fn ttri(
        self,
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
    ) {
        let mut args = self.build().unwrap();
        let colorcount = args.transparent_colors.len().try_into().unwrap_or(-1);
        let transparent_colors = args.transparent_colors.as_ptr();
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
                match args.texture_src {
                    TextureSource::Sprites => 0,
                    TextureSource::Map => 1,
                },
                transparent_colors,
                colorcount,
                args.z1.unwrap_or(0.0),
                args.z2.unwrap_or(0.0),
                args.z3.unwrap_or(0.0),
                args.z1.is_some() || args.z2.is_some() || args.z3.is_some(),
            )
        }
    }
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

/// [time](https://github.com/nesbox/TIC-80/wiki/time)
/// The number of milliseconds elapsed since the game was started.
pub fn time() -> f32 {
    unsafe { extern_time() }
}
extern "C" {
    #[link_name = "time"]
    fn extern_time() -> f32;
}

/// [trace](https://github.com/nesbox/TIC-80/wiki/trace)
/// Print `message` to console.
pub fn trace<T: AsRef<str>>(text: T, color: Option<i8>) {
    let text = text.as_ref().as_ptr();
    unsafe {
        extern_trace(text, color.unwrap_or(-1));
    }
}
extern "C" {
    #[link_name = "trace"]
    fn extern_trace(txt: *const u8, color: i8);
}

/// [tri](https://github.com/nesbox/TIC-80/wiki/tri)
/// Draws a triangle filled with color, using the supplied vertices.
pub fn tri(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, color: i8) {
    unsafe { extern_tri(x1, y1, x2, y2, x3, y3, color) }
}
extern "C" {
    #[link_name = "tri"]
    fn extern_tri(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, color: i8);
}

/// [trib](https://github.com/nesbox/TIC-80/wiki/trib)
/// Draws a triangle border with color, using the supplied vertices.
pub fn trib(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, color: i8) {
    unsafe { extern_trib(x1, y1, x2, y2, x3, y3, color) }
}
extern "C" {
    #[link_name = "trib"]
    fn extern_trib(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, color: i8);
}

/// [tstamp](https://github.com/nesbox/TIC-80/wiki/tstamp)
/// The current Unix timestamp in seconds.
pub fn tstamp() -> u32 {
    unsafe { extern_tstamp() }
}
extern "C" {
    #[link_name = "tstamp"]
    fn extern_tstamp() -> u32;
}

/// [vbank](https://github.com/nesbox/TIC-80/wiki/vbank)
/// Switch VRAM bank (0 or 1).
pub fn vbank(bank: i8) -> i8 {
    unsafe { extern_vbank(bank) }
}
extern "C" {
    #[link_name = "vbank"]
    fn extern_vbank(bank: i8) -> i8;
}
