//! The simplest possible example that does something.

use std::path;
use std::path::Path;
use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

#[derive(Copy, Clone)]
struct Player {
    id: u8,
}

#[derive(Copy, Clone)]
enum Token {
    Pawn,
    King,
}

#[derive(Copy, Clone)]
struct Piece {
    token: Token,
    owner: Player,
}

#[derive(Copy, Clone)]
struct Coords {
    row: usize,
    col: usize,
}

struct MainState {
    turn: u128,
    players: [Player; 2],
    board: [[Option<Piece>; 8]; 8],
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let turn = 0;
        let players = [Player { id: 0 }, Player { id: 1 }];
        let mut board: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];
        for i in 0..7 {
            board[0][i] = Some(Piece { token: Token::King, owner: players[0] });
            board[1][i] = Some(Piece { token: Token::Pawn, owner: players[0] });

            board[7][i] = Some(Piece { token: Token::King, owner: players[1] });
            board[6][i] = Some(Piece { token: Token::Pawn, owner: players[1] });
        }
        let s = MainState { turn, players, board };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        println!("counter = {}", self.turn);
        self.turn += 1;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 6.0].into());

        //let path = Path::new("/cb.jpg");
        //let background = graphics::Image::new(ctx, path)?;
        //graphics::draw(ctx, &background, (na::Point2::new(0.0, 0.0),))?;

        for row in 0..8 {
            for col in 0..8 {
                let rec = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new((row as f32) * 75.0, (col as f32) * 75.0, 75.0, 75.0),
                    if (row + col) % 2 == 0 {graphics::WHITE} else {graphics::BLACK},
                )?;
        
                graphics::draw(ctx, &rec, (na::Point2::new(0.0, 0.0),))?;
            }
        }
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./resources");
    
    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
                                   .add_resource_path(resource_dir);

    let (ctx, event_loop) = &mut cb.build()?;
    //graphics::set_screen_coordinates(ctx, 0.0, 0.0, 1_000.0, 1_000.0)?;

    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
