pub struct Material {
    name: String,
    solid: bool,
}

impl Material {
    pub fn new (name: String, solid: bool) -> Material{
        Material {
            name,
            solid,
        }
    }
}