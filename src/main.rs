#![feature(globs)]

extern crate "sdl2" as sdl;
use sdl::sdl::{init, quit, INIT_JOYSTICK};
use sdl::event::*;
use sdl::joystick::ll::*;

fn main() {
    init(INIT_JOYSTICK);
    // The bindings don't currently have a safe way of initialising the joystick.
    // The functions provided by the ll module can be used to perform the initialisation manually.
    unsafe {
        let joysticks = SDL_NumJoysticks();
        if joysticks < 1 {
            println!("Please plug a joystick in.");
            return;
        }
        // Enable joystick event polling.
        SDL_JoystickEventState(1);
        // Open the first joystick device, ignoring any other ones.
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
