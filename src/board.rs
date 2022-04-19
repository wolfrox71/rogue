pub mod material::Material;

pub struct Board {
    board: Vec<Vec<Material>>,
}
impl Board {
    pub fn new(width: u8, height: u8) -> Board {
        Board {
            board = vec![vec![Material;width];height],
        }
    }
}