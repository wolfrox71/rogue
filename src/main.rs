use crossterm_cursor::cursor;

pub mod player;
pub mod board;

fn main() {
    let mut board = board::Board::new(5,5);
    let player = player::Player::new(&board);
    board.board[1][1] = board::materials::Material::new("test",'t',false);
    board.output();
}