use std::path::Path;

mod bitboard;
mod crc;
mod engine;
mod position;
mod renderer;

use engine::Engine;

use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use sdl2::surface::Surface;

const WIDTH: u32 = 800;
const HEIGHT: u32 = WIDTH;
const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Schach", WIDTH, HEIGHT)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_logical_size(WIDTH, HEIGHT).unwrap();

    let surface = Surface::load_bmp(Path::new("assets/pieces.bmp")).unwrap();

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();

    let engine = Engine::from_fen(START_FEN);

    renderer::render(&mut canvas, &engine, &texture);

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        let event = event_pump.wait_event();
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => break 'running,
            Event::Window {
                win_event: WindowEvent::Resized(..),
                ..
            } => {
                renderer::render(&mut canvas, &engine, &texture);
            }
            // Event::MouseButtonDown { x, y, .. } => {
            //     let file = x / (WIDTH as i32 / 8);
            //     let rank = y / (HEIGHT as i32 / 8);
            //     let index = (8 * rank + file) as usize;

            //     render(&mut canvas, &game_state, &texture);
            // }
            _ => {}
        };
    }
}
