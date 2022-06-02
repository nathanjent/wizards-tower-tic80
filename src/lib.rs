#[cfg(feature = "buddy-alloc")]
mod alloc;
mod tic80;

use tic80::*;

static mut X: i32 = 96;
static mut Y: i32 = 24;
static mut T: i32 = 24;
const M: &str = "HELLO WORLD FROM RUST!";

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn BOOT() {}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn TIC() {
    cls(13);

    // The standard demo.
    if btn(0) {
        unsafe {
            Y -= 1;
        }
    }
    if btn(1) {
        unsafe {
            Y += 1;
        }
    }
    if btn(2) {
        unsafe {
            X -= 1;
        }
    }
    if btn(3) {
        unsafe {
            X += 1;
        }
    }

    unsafe {
        spr(1 + T % 60 / 30 * 2, X, Y, None, 0, 3, 0, 0, 2, 2);
    }
    print(M, 60, 84, 15, true, 1, false);
    unsafe {
        T += 1;
    }
}
