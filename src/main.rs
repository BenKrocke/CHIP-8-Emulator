extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

extern crate rand;

mod chip8;
mod input;
mod display;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window = init_piston(opengl);
    let mut chip = chip8::init_chip();
    chip.load_rom(std::string::String::from("pong"));

    let mut piston = display::init_display(opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            piston.render(&r);
        }

        if let Some(u) = e.update_args() {
            piston.update(&u);
        }

        //chip.cycle();
        chip.graphics.draw_screen();
    }
}

// Create an Glutin window.
fn init_piston(opengl: glutin_window::OpenGL) -> Window {
    let mut window: Window = WindowSettings::new(
            "spinning-square",
            [200, 200]
        )
    .opengl(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

    return window;
}