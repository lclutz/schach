use super::*;

use crc::SquareState;
use crc::CRC;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::WindowCanvas;

const WINDOW_BG: Color = Color::RGB(0x18, 0x18, 0x18);
const DARK_SQUARE_BG: Color = Color::RGB(0xb5, 0x88, 0x63);
const LIGHT_SQUARE_BG: Color = Color::RGB(0xf0, 0xd9, 0xb5);

pub fn render(canvas: &mut WindowCanvas, engine: &Engine, texture: &Texture) {
    canvas.set_draw_color(WINDOW_BG);
    canvas.clear();

    render_chess_board(canvas, &engine.position_as_crc(), texture);

    canvas.present();
}

pub fn render_attack_mask(canvas: &mut WindowCanvas, mask: u64) {
    canvas.set_draw_color(Color::RGB(0xff, 0x00, 0x00));
    for index in 0..64 {
        if (0x8000000000000000 >> index) & mask != 0 {
            canvas.fill_rect(Rect::new(
                (index % 8) * WIDTH as i32 / 8 + WIDTH as i32 / 32,
                (index / 8) * HEIGHT as i32 / 8 + HEIGHT as i32 / 32,
                WIDTH / 8 - WIDTH / 16,
                HEIGHT / 8 - WIDTH / 16,
            )).unwrap();
        }
    }
    canvas.present();
}

fn render_chess_board(canvas: &mut WindowCanvas, crc: &CRC, texture: &Texture) {
    canvas.set_draw_color(DARK_SQUARE_BG);
    canvas.fill_rect(Rect::new(0, 0, WIDTH, HEIGHT)).unwrap();
    canvas.set_draw_color(LIGHT_SQUARE_BG);

    for row in 0..8 {
        for col in 0..8 {
            if (row + col) % 2 == 0 {
                canvas
                    .fill_rect(Rect::new(
                        col * WIDTH as i32 / 8,
                        row * HEIGHT as i32 / 8,
                        WIDTH / 8,
                        HEIGHT / 8,
                    ))
                    .unwrap();
            }
        }
    }

    let w: f32 = texture.query().width as f32 / 6.0;
    let h: f32 = texture.query().height as f32 / 2.0;
    let mut y: i32;
    let mut x: i32;

    for (index, square) in crc.iter().enumerate() {
        match square {
            SquareState::Unoccupied => continue,
            SquareState::WhiteKing => {
                x = 0;
                y = 0;
            }
            SquareState::WhiteQueen => {
                x = (1.0 * w) as i32;
                y = 0;
            }
            SquareState::WhiteBishop => {
                x = (2.0 * w) as i32;
                y = 0;
            }
            SquareState::WhiteKnight => {
                x = (3.0 * w) as i32;
                y = 0;
            }
            SquareState::WhiteRook => {
                x = (4.0 * w) as i32;
                y = 0;
            }
            SquareState::WhitePawn => {
                x = (5.0 * w) as i32;
                y = 0;
            }
            SquareState::BlackKing => {
                x = 0;
                y = h as i32;
            }
            SquareState::BlackQueen => {
                x = (1.0 * w) as i32;
                y = h as i32;
            }
            SquareState::BlackBishop => {
                x = (2.0 * w) as i32;
                y = h as i32;
            }
            SquareState::BlackKnight => {
                x = (3.0 * w) as i32;
                y = h as i32;
            }
            SquareState::BlackRook => {
                x = (4.0 * w) as i32;
                y = h as i32;
            }
            SquareState::BlackPawn => {
                x = (5.0 * w) as i32;
                y = h as i32;
            }
        };

        canvas
            .copy(
                texture,
                Rect::new(x, y, w as u32, h as u32),
                Rect::new(
                    (index as i32 % 8) * (WIDTH as i32 / 8),
                    (index as i32 / 8) * (HEIGHT as i32 / 8),
                    WIDTH / 8,
                    HEIGHT / 8,
                ),
            )
            .unwrap();
    }
}
