pub struct Material {
    pub base_color: [f32; 3],
    pub metallic_factor: f32,
    pub roughness: f32,
}

impl Material {
    pub fn default() -> Self {
        Self {
            roughness: 0.0,
            base_color: [1.0; 3],
            metallic_factor: 0.0,
        }
    }
}
