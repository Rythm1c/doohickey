use crate::src::{lights::PointLight, object::Object, shaders::Program};
use std::collections::HashMap;

pub struct Assets {
    pub objects: HashMap<String, Object>,
    /// point lights
    pub lights: Vec<PointLight>,
    pub shaders: HashMap<String, Program>,
}

impl Assets {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            lights: Vec::new(),
            shaders: HashMap::new(),
        }
    }
    // by deafult all get functions return a mutable reference

    //____________________________________________________________________________________
    // functions for managing objects
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
    //____________________________________________________________________________________
    // functions for managing shaders
    pub fn add_shader(&mut self, n: &str, s: Program) {
        if self.shaders.contains_key(&String::from(n)) {
            println!("name already exists!");
        } else {
            self.shaders.insert(String::from(n), s);
        }
    }

    pub fn get_shader(&mut self, n: &str) -> &mut Program {
        self.shaders.get_mut(&String::from(n)).unwrap()
    }

    pub fn remove_shader(&mut self, n: &str) {
        if self.shaders.contains_key(&String::from(n)) {
            self.shaders.remove(&String::from(n));
        } else {
            println!("no shader with such name exists!");
        }
    }
    //____________________________________________________________________________________
    // functions for managing point lights
    pub fn add_pointlight(&mut self, pl: PointLight) {
        self.lights.push(pl);
    }
    pub fn remove_pointlight(&mut self, index: usize) {
        if index >= self.lights.len() {
            print!("out of bounds, point light does not exist!");
        } else {
            self.lights.remove(index);
        }
    }
}
