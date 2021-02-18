extern crate sdl2;

use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::WindowCanvas;
use sdl2::surface::Surface;

use std::path::Path;

const WIDTH: u32 = 800;
const HEIGHT: u32 = WIDTH;
const WINDOW_BG: Color = Color::RGB(0x18, 0x18, 0x18);
const DARK_SQUARE_BG: Color = Color::RGB(0xb5, 0x88, 0x63);
const LIGHT_SQUARE_BG: Color = Color::RGB(0xf0, 0xd9, 0xb5);
const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

#[derive(Copy, Clone)]
enum PieceKind {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Copy, Clone)]
enum PieceColor {
    White,
    Black,
}

#[derive(Copy, Clone)]
struct Piece {
    color: PieceColor,
    kind: PieceKind,
}

struct Move {
    src: usize,
    dst: usize,
}

struct GameState {
    board: [Option<Piece>; 64],
}

impl GameState {
    fn mv(&mut self, mv: &Move) {
        if mv.dst == mv.src {
            return;
        }

        self.board[mv.dst] = self.board[mv.src];
        self.board[mv.src] = None;
    }

    fn read_fen(&mut self, input: &str) {
        let mut file = 0;
        let mut rank = 0;

        for symbol in input.chars() {
            if symbol == '/' {
                file = 0;
                rank += 1;
            } else {
                if symbol.is_digit(10) {
                    file += symbol.to_digit(10).unwrap() as usize;
                } else {
                    let color = if symbol.is_uppercase() {
                        PieceColor::White
                    } else {
                        PieceColor::Black
                    };

                    let kind: Option<PieceKind>;
                    match symbol {
                        'k' | 'K' => kind = Some(PieceKind::King),
                        'n' | 'N' => kind = Some(PieceKind::Knight),
                        'b' | 'B' => kind = Some(PieceKind::Bishop),
                        'r' | 'R' => kind = Some(PieceKind::Rook),
                        'q' | 'Q' => kind = Some(PieceKind::Queen),
                        'p' | 'P' => kind = Some(PieceKind::Pawn),
                        _ => kind = None,
                    };

                    self.board[rank * 8 + file] = Some(Piece {
                        kind: kind.unwrap(),
                        color: color,
                    });

                    file += 1;
                }
            }
        }
    }
}

fn render_chess_board(
    canvas: &mut WindowCanvas,
    game_state: &GameState,
    texture: &Texture,
) -> Result<(), String> {
    canvas.set_draw_color(DARK_SQUARE_BG);
    canvas.fill_rect(Rect::new(0, 0, WIDTH, HEIGHT))?;

    canvas.set_draw_color(LIGHT_SQUARE_BG);
    for rank in 0..8 {
        for file in 0..8 {
            if (rank + file) % 2 == 0 {
                canvas.fill_rect(Rect::new(
                    rank * (WIDTH as i32 / 8),
                    file * (HEIGHT as i32 / 8),
                    WIDTH / 8,
                    HEIGHT / 8,
                ))?;
            }
        }
    }

    let w: u32 = 332;
    let h: u32 = 332;
    let mut y: i32;
    let mut x: i32;

    for (index, piece) in game_state.board.iter().enumerate() {
        if piece.is_none() {
            continue;
        }

        match piece.unwrap().kind {
            PieceKind::King => x = 0,
            PieceKind::Queen => x = 1 * w as i32,
            PieceKind::Bishop => x = 2 * w as i32,
            PieceKind::Knight => x = 3 * w as i32,
            PieceKind::Rook => x = 4 * w as i32,
            PieceKind::Pawn => x = 5 * w as i32,
        };

        match piece.unwrap().color {
            PieceColor::Black => y = h as i32,
            PieceColor::White => y = 0,
        };

        canvas.copy(
            texture,
            Rect::new(x, y, w, h),
            Rect::new(
                (index as i32 % 8) * (WIDTH as i32 / 8),
                (index as i32 / 8) * (HEIGHT as i32 / 8),
                WIDTH / 8,
                HEIGHT / 8,
            ),
        )?;
    }

    Ok(())
}

fn render(
    canvas: &mut WindowCanvas,
    game_state: &GameState,
    texture: &Texture,
) -> Result<(), String> {
    canvas.set_draw_color(WINDOW_BG);
    canvas.clear();

    render_chess_board(canvas, game_state, texture)?;

    canvas.present();

    Ok(())
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Schach", WIDTH, HEIGHT)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas
        .set_logical_size(WIDTH, HEIGHT)
        .map_err(|e| e.to_string())?;

    let surface = Surface::load_bmp(Path::new("assets/pieces.bmp"))?;

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut game_state = GameState { board: [None; 64] };

    game_state.read_fen(START_FEN);

    render(&mut canvas, &game_state, &texture)?;

    let mut mv = Move { src: 0, dst: 0 };

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
                render(&mut canvas, &game_state, &texture)?;
            }
            Event::MouseButtonDown { x, y, .. } => {
                let file = x / (WIDTH as i32 / 8);
                let rank = y / (HEIGHT as i32 / 8);
                let index = 8 * rank + file;
                mv.src = index as usize;
            }
            Event::MouseButtonUp { x, y, .. } => {
                let file = x / (WIDTH as i32 / 8);
                let rank = y / (HEIGHT as i32 / 8);
                let index = 8 * rank + file;
                mv.dst = index as usize;

                game_state.mv(&mv);
                render(&mut canvas, &game_state, &texture)?;
            }
            _ => {}
        };
    }

    Ok(())
}
