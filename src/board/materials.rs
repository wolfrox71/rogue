#[derive(Clone)]
pub struct Material {
    name: String,
    pub symbol: char,
    pub solid: bool,
}

impl Drawable for Material {
    fn new (name: &str, symbol: char, solid: bool) -> Material{
        Material {
            name: String::from(name),
            symbol,
            solid,
        }
    }
    fn empty() -> Material {
        Material {
            name: String::from("Empty"),
            symbol: '+',
            solid: true,
        }
    }
    fn draw(&self) {
        print!("{}", self.symbol);
    }
}