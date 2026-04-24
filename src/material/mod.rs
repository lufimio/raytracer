pub mod dielectric;
pub mod lambertian;
pub mod light;
pub mod metal;

use crate::{
    geometry::{Color, Point3, Ray},
    hittable::HitRecord,
    material::{dielectric::Dielectric, lambertian::Lambertian, light::DiffuseLight, metal::Metal},
};
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait Scatter {
    fn scatter(&self, _r: Ray, _rec: &HitRecord) -> Option<(Ray, Color)> {
        Option::None
    }

    fn emitted(&self, _u: f32, _v: f32, _p: Point3) -> Color {
        Color::ZERO
    }
}

#[enum_dispatch(Scatter)]
#[derive(Debug)]
pub enum Material {
    Lambertian,
    Metal,
    Dielectric,
    DiffuseLight,
}
