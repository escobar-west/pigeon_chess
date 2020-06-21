//! The simplest possible example that does something.

use std::path;
use std::path::Path;
use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

#[derive(Copy, Clone, PartialEq)]
struct Player {
    id: u8,
}

#[derive(Copy, Clone, PartialEq)]
enum Token {
    Pawn,
    Queen,
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
        for i in 0..8 {
            board[0][i] = Some(Piece { token: Token::Queen, owner: players[0] });
            board[1][i] = Some(Piece { token: Token::Pawn, owner: players[0] });

            board[7][i] = Some(Piece { token: Token::Queen, owner: players[1] });
            board[6][i] = Some(Piece { token: Token::Pawn, owner: players[1] });
        }
        let s = MainState { turn, players, board };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.turn % 100 == 0 { println!("counter = {}", self.turn); }
        self.turn += 1;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 6.0].into());

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
        let mut path = Path::new("/queen_light.png");
        let mut sprite_img = graphics::Image::new(ctx, path)?;
        let mut q_l_sb = graphics::spritebatch::SpriteBatch::new(sprite_img); 

        path = Path::new("/queen_dark.png");
        sprite_img = graphics::Image::new(ctx, path)?;
        let mut q_d_sb = graphics::spritebatch::SpriteBatch::new(sprite_img); 

        path = Path::new("/pawn_light.png");
        sprite_img = graphics::Image::new(ctx, path)?;
        let mut p_l_sb = graphics::spritebatch::SpriteBatch::new(sprite_img); 

        path = Path::new("/pawn_dark.png");
        sprite_img = graphics::Image::new(ctx, path)?;
        let mut p_d_sb = graphics::spritebatch::SpriteBatch::new(sprite_img); 

        let offset = 5.0;

        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = self.board[row as usize][col as usize] {
                    let row = row as f32;
                    let col = col as f32;

                    if piece.token == Token::Queen && piece.owner ==  self.players[0] {
                        q_l_sb.add((na::Point2::new(col * 75.00 + offset, (7.0-row) * 75.00 + offset),));
                    }
                    else if piece.token == Token::Queen && piece.owner ==  self.players[1] {
                        q_d_sb.add((na::Point2::new(col * 75.00 + offset, (7.0-row) * 75.00 + offset),));
                    }
                    else if piece.token == Token::Pawn && piece.owner ==  self.players[0] {
                        p_l_sb.add((na::Point2::new(col * 75.00 + offset, (7.0-row) * 75.00 + offset),));
                    }
                    else if piece.token == Token::Pawn && piece.owner ==  self.players[1] {
                        p_d_sb.add((na::Point2::new(col * 75.00 + offset, (7.0-row) * 75.00 + offset),));
                    }
                }
            }
        }
        p_l_sb.add((na::Point2::new(4.0 * 75.00 + offset, (7.0 - 1.0) * 75.00 + offset - (self.turn as f32)),));
        graphics::draw(ctx, &q_l_sb, (na::Point2::new(0.0, 0.0),))?;
        graphics::draw(ctx, &q_d_sb, (na::Point2::new(0.0, 0.0),))?;
        graphics::draw(ctx, &p_l_sb, (na::Point2::new(0.0, 0.0),))?;
        graphics::draw(ctx, &p_d_sb, (na::Point2::new(0.0, 0.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./resources");
    
    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
                                   .add_resource_path(resource_dir);

    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
