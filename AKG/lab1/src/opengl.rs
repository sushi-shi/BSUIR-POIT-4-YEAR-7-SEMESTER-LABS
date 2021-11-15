use sdl2::pixels::Color;
use sdl2::IntegerOrSdlError;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2;

use std::time::Duration;

use crate::types::State;

const BUTTON_MOVE_DELTA: f64 = 0.2;
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

pub fn on_mousewheel(state: &mut State, dx: f64, dy: f64) -> () {

    redraw(state)
}

pub fn on_keyboard(state: &mut State, dx: f64, dy: f64) -> () {

    redraw(state)
}

pub fn redraw(state: &State) -> () {

    todo!()
}



pub fn run(state: State) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "Floating Lab", 
            WINDOW_WIDTH, 
            WINDOW_HEIGHT,
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;

    'running: loop {
        // i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'running,
                Event::KeyDown { keycode, .. } => 
                    match keycode {
                        Some(Keycode::Escape) => break 'running,
                        Some(Keycode::Down)   => break 'running,
                        Some(Keycode::Up)     => break 'running,
                        Some(Keycode::Right)  => break 'running,
                        Some(Keycode::Left)   => break 'running,
                        _ => {}
                    }
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
