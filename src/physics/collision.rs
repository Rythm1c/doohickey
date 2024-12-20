use crate::src::math::vec3::*;

use crate::src::shapes::shape::Shape;
use std::collections::HashMap;

struct AABB {
    min: Vec3,
    max: Vec3,
}

fn radius(size: &Vec3) -> f32 {
    (size.x + size.y + size.z) / 3.0
}

struct Collider(String, String);

impl Collider {}


//get a bounding box
fn get_aabb(pos: Vec3, size: Vec3) -> AABB {
    let mut minimum = vec3(0.0, 0.0, 0.0);
    minimum.x = pos.x - size.x;
    minimum.y = pos.y - size.y;
    minimum.z = pos.z - size.z;

    let mut maximum = vec3(0.0, 0.0, 0.0);
    maximum.x = pos.x + size.x;
    maximum.y = pos.y + size.y;
    maximum.z = pos.z + size.z;

    AABB {
        max: maximum,
        min: minimum,
    }
}
//**** collision function definations ****//
// none of the function conserve momentum and are very primitive
// nothing too fancy

pub fn sphere_sphere(s1: String, s2: String, shapes: &mut HashMap<String, Shape>) {
    // get the distance between the two spheres
    let distance = (shapes.get(&s1).unwrap().transform.translation
        - shapes.get(&s2).unwrap().transform.translation)
        .len();
    // get the average of size attributes
    // since the Model struct doesn't have a radius member
    let radius1 = radius(&shapes.get(&s1).unwrap().transform.scaling);
    let radius2 = radius(&shapes.get(&s2).unwrap().transform.scaling);
    let sum_radii = radius1 + radius2;
    // if distance between objects(spheres) is less than the sum of both radii
    // then a collision  has occured
    if distance < sum_radii {
        let s2_pos = shapes.get(&s2).unwrap().transform.translation;
        let s1_pos = shapes.get(&s1).unwrap().transform.translation;
        // |AB| = |OB|-|OA|
        let ab = (s2_pos - s1_pos).unit();
        //update each objects position to outside the others bounds
        shapes.get_mut(&s1).unwrap().transform.translation = s2_pos + (ab * -sum_radii);
        shapes.get_mut(&s2).unwrap().transform.translation = s1_pos + (ab * sum_radii);
        // reflect velocity and reduce magnitude
        let rv1 = reflect(&shapes.get_mut(&s1).unwrap().velocity, &(-ab));
        let rv2 = reflect(&shapes.get_mut(&s2).unwrap().velocity, &ab);
        shapes.get_mut(&s1).unwrap().velocity = rv1 * 0.8;
        shapes.get_mut(&s2).unwrap().velocity = rv2 * 0.8;
    }
}
// check collision betweem sphere and axis aligned bounding box
pub fn sphere_aabb(sphere: String, aabb: String, shapes: &mut HashMap<String, Shape>) {
    let radius = radius(&shapes.get(&sphere).unwrap().transform.scaling);
    let aabb_size = shapes.get(&aabb).unwrap().transform.scaling;

    let sphere_pos = shapes.get(&sphere).unwrap().transform.translation;
    let aabb_pos = shapes.get(&aabb).unwrap().transform.translation;

    // BA = AO + BO = -OB + OA
    let mut difference = sphere_pos - aabb_pos;
    let clamped = clamp_vec3(&difference, &(-aabb_size), &aabb_size);
    let closest_point = clamped + aabb_pos;

    let distance = (closest_point - sphere_pos).len();

    if distance <= radius {
        // BA=AO+BO=-OB+OA
        difference = sphere_pos - closest_point;
        let normal = difference.unit();
        let new_velocity = shapes.get(&sphere).unwrap().velocity * 0.8;

        shapes.get_mut(&sphere).unwrap().transform.translation =
            sphere_pos + normal * radius - difference;
        shapes.get_mut(&sphere).unwrap().velocity = reflect(&new_velocity, &normal);
    }
}
//check and resolve collisions between two axis aligned bounding boxes
pub fn aabb_aabb(aabb1: String, aabb2: String, shapes: &mut HashMap<String, Shape>) {
    let pos1 = shapes.get(&aabb1).unwrap().transform.translation;
    let pos2 = shapes.get(&aabb2).unwrap().transform.translation;
    let size1 = shapes.get(&aabb1).unwrap().transform.scaling;
    let size2 = shapes.get(&aabb2).unwrap().transform.scaling;

    let box1 = get_aabb(pos1.clone(), size1.clone());
    let box2 = get_aabb(pos2.clone(), size2.clone());
    //intersection test
    let intersectionx = (box1.min.x <= box2.max.x) && (box1.max.x >= box2.min.x);
    let intersectiony = (box1.min.y <= box2.max.y) && (box1.max.y >= box2.min.y);
    let intersectionz = (box1.min.z <= box2.max.z) && (box1.max.z >= box2.min.z);

    if intersectionx && intersectiony & intersectionz {
        // if collision detected then resolve
        let difference: Vec3;
        let new_velocity: Vec3;
        let normal: Vec3;
        //get the difference between all possible intersections in each axis
        let dx1 = box1.min.x - box2.max.x;
        let dx2 = box1.max.x - box2.min.x;
        let dy1 = box1.min.y - box2.max.y;
        let dy2 = box1.max.y - box2.min.y;
        let dz1 = box1.min.z - box2.max.z;
        let dz2 = box1.max.z - box2.min.z;
        // get the smallest difference for each axis
        let dx = if dx1.abs() < dx2.abs() { dx1 } else { dx2 };
        let dy = if dy1.abs() < dy2.abs() { dy1 } else { dy2 };
        let dz = if dz1.abs() < dz2.abs() { dz1 } else { dz2 };
        // update position of objects using the smallest distance calculated
        // very primitive at the moment
        // only usefull against slow moving objects and big objects
        // doesn't work well for fast and small particles
        if (dx.abs() < dy.abs()) && (dx.abs() < dz.abs()) {
            // x axis
            normal = vec3(-dx, 0.0, 0.0).unit();
            difference = vec3(-dx, 0.0, 0.0);
        } else if (dy.abs() < dx.abs()) && (dy.abs() < dz.abs()) {
            // y axis
            normal = vec3(0.0, -dy, 0.0).unit();
            difference = vec3(0.0, -dy, 0.0);
        } else {
            // z axis
            normal = vec3(0.0, 0.0, -dz).unit();
            difference = vec3(0.0, 0.0, -dz);
        }
        //finally update
        new_velocity = shapes.get_mut(&aabb1).unwrap().velocity * 0.8;
        shapes.get_mut(&aabb1).unwrap().transform.translation = difference + pos1;
        shapes.get_mut(&aabb1).unwrap().velocity = reflect(&new_velocity, &normal);
    }
}
