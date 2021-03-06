use rogue::Drawable;
use crate::board::Board;
pub struct Player<'a> {
    x: u8,
    y: u8,
    screen: &'a Board,
    symbol: char,
}
impl Drawable for Player<'_> {
    fn new(screen: &Board) -> Player{
        Player {
            x: 1,
            y: 1,
            screen,
            symbol: 'X',
        }
    }
    fn up(&self) {
        // if the user cant move up
        if self.screen.board[self.y+1][self.x].solid {
            // return and not move up
            return;
        }
        self.x += 1;
    }
    fn down(&self) {
    }
}