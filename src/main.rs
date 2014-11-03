#![feature(globs)]

extern crate "sdl2" as sdl;
use sdl::sdl::{init, quit, INIT_JOYSTICK};
use sdl::event::*;
use sdl::joystick::ll::*;

fn main() {
    init(INIT_JOYSTICK);
    unsafe {
        let joysticks = SDL_NumJoysticks();
        if joysticks < 1 {
            println!("Please plug a joystick in.");
            return;
        }
        SDL_JoystickEventState(1);
        SDL_JoystickOpen(0);
    }
    loop {
        match wait_event().unwrap() {
            JoyAxisMotionEvent(_, _, axis, value) => println!("axis({},{})", axis, value),
            JoyBallMotionEvent(_, _, ball, x, y) => println!("ball({},{},{})", ball, x, y),
            JoyHatMotionEvent(_, _, hat, state) => println!("hat({},{})", hat, state.bits()),
            JoyButtonDownEvent(_, _, button) => println!("button({})", button),
            QuitEvent(_) | AppTerminatingEvent(_) => break,
            _ => {}
        }
    }
    quit();
}
