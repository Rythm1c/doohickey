use crate::src::{lights::PointLight, object::Object};
use std::collections::HashMap;

pub struct Assets {
    pub objects: HashMap<String, Object>,
    pub point_lights: Vec<PointLight>,
}

impl Assets {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            point_lights: Vec::new(),
        }
    }

    pub fn add_object(&mut self, n: &str, o: Object) {
        if self.objects.contains_key(&String::from(n)) {
            println!("name already exists!");
        } else {
            self.objects.insert(String::from(n), o);
        }
    }
    pub fn get_object(&mut self, n: &str) -> &mut Object {
        self.objects.get_mut(&String::from(n)).unwrap()
    }

    pub fn remove_object(&mut self, n: String) {
        if self.objects.contains_key(&n) {
            self.objects.remove(&n);
        } else {
            println!("no character with such name exists!")
        }
    }

    pub fn add_pointlight(&mut self, pl: PointLight) {
        self.point_lights.push(pl);
    }
    pub fn remove_pointlight(&mut self, index: usize) {
        if index >= self.point_lights.len() {
            print!("out of bounds, point light does not exist!");
        } else {
            self.point_lights.remove(index);
        }
    }
}
