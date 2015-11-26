mod game;
mod view;

extern crate sdl2;
use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::render::Renderer;
use game::{play, Board, Square};
use view::*;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let window = video.window("Tic-tac-toe", 800, 600)
        .position_centered().opengl()
        .build().unwrap();

    let mut renderer = window.renderer()
        .accelerated()
        .build().unwrap();

    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();

    let mut board_view = BoardView {cross: &render_texture_for_cross(&mut renderer), nought: &render_texture_for_nought(&mut renderer)};
    let mut board = Board::new();
    let computer_move = play(&board).unwrap();
    board.squares[computer_move] = Square::Cross;
    draw_board(&board_view, &mut renderer, &board);
    renderer.present();

    let mut keep_going = true;
    while keep_going {
        match pump_events(&mut event_pump) {
            Action::Quit => keep_going = false,
            Action::Play {index} => {
                if board.squares[index] == Square::Empty {
                    board.squares[index] = Square::Nought;

                    let computer_move = play(&board).unwrap();
                    board.squares[computer_move] = Square::Cross;
                    draw_board(&board_view, &mut renderer, &board);
                    renderer.present();
                }
            }
        }
    }
}

fn draw_board(board_view: &BoardView, renderer: &mut Renderer, board: &Board) {
    let mut x = 0;
    let mut y = 0;
    for sq in board.squares.iter() {
        match *sq {
            Square::Cross => board_view.draw_cross_at_position(renderer, x, y),
            Square::Nought => board_view.draw_nought_at_position(renderer, x, y),
            _ => {}
        }
        x += 1;
        if x == 3 {
            x = 0;
            y += 1;
        }
    }
    BoardView::draw_frame(renderer);
}

enum Action {
    Quit,
    Play { index: usize }
}

fn pump_events(pump: &mut EventPump) -> Action {
    loop {
        for event in pump.poll_iter() {
            use sdl2::event::Event::*;
            match event {
                Quit { .. } => {
                    return Action::Quit;
                },
                KeyDown { keycode, ..} => {
                    use ::sdl2::keyboard::Keycode::Escape;
                    match keycode {
                        Some(Escape) => return Action::Quit,
                        _ => {}
                    }
                }
                MouseButtonDown { x, y, .. } => {
                    if let Some(index) = BoardView::coords_to_index(x, y) {
                        return Action::Play {index: index};
                    }
                },
                _ => {}
            }
        }
    }
}
