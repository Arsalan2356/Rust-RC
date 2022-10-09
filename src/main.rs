mod board;

use std::env;
extern crate sdl2;

use board::Board;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

const DIM: u32 = 512;
const BOARD_SIZE: u32 = 8;
const SQUARE_SIZE: u32 = DIM / BOARD_SIZE;

fn main() {
    let mut fen: String = "".to_string();
    let mut player_one = true;
    let mut player_two = false;
    let mut side = 0;

    let args: Vec<String> = env::args().collect();
    if args.len() != 1 {
        let mut it = args.iter().position(|x| x == "-f");
        match it {
            Some(index) => {
                for i in 1..7 {
                    fen += &args[index + i];
                }
            }
            None => (),
        }
        it = args.iter().position(|x| x == "-p");
        match it {
            None => (),
            Some(index) => match args[index + 1].as_str() {
                "w" => {
                    player_one = true;
                    player_two = false;
                }
                "b" => {
                    player_one = false;
                    player_two = true;
                }
                "n" => {
                    player_one = false;
                    player_two = false;
                }
                "y" => {
                    player_one = true;
                    player_two = true;
                }
                _ => (),
            },
        }
        it = args.iter().position(|x| x == "-s");
        match it {
            Some(index) => match args[index + 1].as_str() {
                "b" => side = 1,
                "w" => side = 0,
                _ => (),
            },
            None => (),
        }
    }

    if fen.is_empty() {
        fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
    }

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Chess", DIM, DIM)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut events = sdl_context.event_pump().unwrap();
    let mut running = true;
    let square_selected = (-1, -1);

    let board: board::Board = Board::new(fen);

    while running {}
}
