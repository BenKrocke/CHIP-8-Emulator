extern crate rand;
extern crate sdl;

use sdl::event::Event;

mod chip8;
mod input;
mod display;


fn main() {
    let mut chip = chip8::init_chip();
    //chip.load_rom(std::string::String::from("BC_test.ch8"));
    chip.load_rom(std::string::String::from("Space Invaders [David Winter].ch8"));

    sdl::init(&[sdl::InitFlag::Video, sdl::InitFlag::Audio, sdl::InitFlag::Timer]);

    'main : loop {
        'event : loop {
            match sdl::event::poll_event() {
                Event::Quit                  => break 'main,
                Event::None                  => break 'event,
                Event::Key(key, state, _, _) => chip.input.press(key, state),
                _                            => {}
            }
        }

        chip.cycle();
        chip.display.draw_screen();
    }

    sdl::quit();
} 