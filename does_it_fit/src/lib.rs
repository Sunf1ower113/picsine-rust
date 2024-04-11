pub use crate::areas_volumes::*;

pub mod areas_volumes;

pub fn area_fit(
    x: usize,
    y: usize,
    objects: areas_volumes::GeometricalShapes,
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    match objects {
        areas_volumes::GeometricalShapes::Rectangle => {
            times * rectangle_area(a, b) <= rectangle_area(x, y)
        },
        areas_volumes::GeometricalShapes::Square => {
            times * square_area(a) <= rectangle_area(x, y)
        },
        areas_volumes::GeometricalShapes::Triangle => {
            times as f64 * triangle_area(a, b) <= rectangle_area(x, y) as f64
        },
        areas_volumes::GeometricalShapes::Circle => {
            times as f64 * circle_area(a) <= rectangle_area(x, y) as f64
        }
    }
}
pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: areas_volumes::GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
    match objects {
        GeometricalVolumes::Sphere => {
            times as f64 * sphere_volume(a) <=
                parallelepiped_volume(x, y, z) as f64
        },
        GeometricalVolumes::Parallelepiped => {
            times  * parallelepiped_volume(a, b, c) <=
                parallelepiped_volume(x, y, z)
        },
        GeometricalVolumes::Cone => {
            times as f64 * cone_volume(a, b) <=
                parallelepiped_volume(x, y, z) as f64
        },
        GeometricalVolumes::Cube => {
            times * cube_volume(a) <=
                parallelepiped_volume(x, y, z)
        },
        GeometricalVolumes::Pyramid => {
            times as f64 * triangular_pyramid_volume(a as f64, b) <=
                parallelepiped_volume(x, y, z) as f64
        },
    }
}