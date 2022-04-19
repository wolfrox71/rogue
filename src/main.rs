use crossterm_cursor::cursor;
pub mod player;
pub mod board;

fn main() {
    let board : board::Board = board::Board::new(5,5);
    let player = player::Player::new(&board);
}