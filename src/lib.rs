#[cfg(feature = "buddy-alloc")]
mod alloc;
mod tic80;
mod tic80_error;

use std::cell::Cell;
use std::cell::RefCell;

use tic80::*;
use tic80_error::Tic80Error;

const X: Cell<i32> = Cell::new(96);
const Y: Cell<i32> = Cell::new(24);
const T: Cell<i32> = Cell::new(0);
const M: &str = "HELLO WORLD FROM RUST!";
const ENTITY_COUNT: usize = 3;

#[derive(Default, Clone, Copy)]
pub struct Entity {
    pub id: usize,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub max_vx: f32,
    pub max_vy: f32,
    pub ax: f32,
    pub ay: f32,
    pub sprite: Option<usize>,
    pub flip: bool,
}

thread_local! {
    static ENTITIES: RefCell<[Entity; ENTITY_COUNT]> = RefCell::new([Default::default(); ENTITY_COUNT]);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn BOOT() {}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn TIC() {
    // The standard demo.
    tic().expect("Error");
}

fn tic() -> Result<(), Tic80Error> {
    //trace("hi", None).expect("trace");
    if btn(0) {
    //    trace("btn 0", None).expect("trace btn");
        Y.set(Y.get() - 1);
    }
    if btn(1) {
        //    trace("btn 1", None).expect("trace btn");
        Y.set(Y.get() + 1);
    }
    if btn(2) {
    //    trace("btn 2", None).expect("trace btn");
        X.set(X.get() - 1);
    }
    if btn(3) {
    //    trace("btn 3", None).expect("trace btn");
        X.set(X.get() + 1);
    }

    cls(13);
    //map(None, None, None, None, None, None, None, None, None, None);

    spr(
        1 + T.get() % 60 / 30 * 2,
        X.get(),
        Y.get(),
        Some(&[14]),
        3,
        0,
        0,
        2,
        2,
    )?;
    //print(M, 60, 84, 15, true, 1, false).expect("print");
    T.set(T.get() + 1);
    Ok(())
}
