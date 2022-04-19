pub mod materials;
pub struct Board {
    pub board: Vec<Vec<dyn Drawable>>,
    width: usize,
    height: usize,
}
impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        Board {
            board: vec![vec![materials::Material::empty();width];height],
            width,
            height,
        }
    }
    pub fn output(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let square = &self.board[y][x];
                print!("{}", square.symbol);
            }
            println!("");
        }
    }
}