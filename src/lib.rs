#[cfg(feature = "buddy-alloc")]
mod alloc;
mod tic80;
mod tic80_error;

use std::cell::RefCell;

use tic80::*;
use tic80_error::Tic80Error;

struct Game {
    tic: i32,
    player: Player,
}

struct Player {
    x: i32,
    y: i32,
}

thread_local! {
    static GAME: RefCell<Game> = RefCell::new(
        Game {
            tic: 0,
        player: Player {
            x: 96,
            y: 24,
        },
    });
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn BOOT() {}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn TIC() {
    // The standard demo.
    tic().unwrap_or_else(|e| {
        trace(format!("Error: \"{}\"\0", e), None);
    });
}

fn tic() -> Result<(), Tic80Error> {
    GAME.with(|game| {
        let mut game = game.borrow_mut();
        game.tic += 1;

        if Btnp::default().id(0).hold(6).period(30).btnp() {
            trace("btn 0\0", None);
            game.player.y -= 16;
        }

        if Btnp::default().id(1).hold(6).period(30).btnp() {
            trace("btn 1\0", None);
            game.player.y += 16;
        }

        if Btnp::default().id(2).hold(6).period(30).btnp() {
            trace("btn 2\0", None);
            game.player.x -= 16;
        }

        if Btnp::default().id(3).hold(6).period(30).btnp() {
            trace("btn 3\0", None);
            game.player.x += 16;
        }
    });

    cls(13);
    map_default();

    GAME.with(|game| {
        let game = game.borrow();

        Spr::default()
            .transparent_colors(&Colors::with_color(14u8))
            .width(2)
            .height(2)
            .spr(1 + game.tic % 60 / 30 * 2, game.player.x, game.player.y);

        Print::default()
            .x(84)
            .y(84)
            .print("HELLO WORLD FROM RUST!\0");
    });

    Ok(())
}
