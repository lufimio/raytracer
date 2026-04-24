use crate::{
    geometry::{Color, Point3, Ray},
    hittable::HitRecord,
    material::Scatter,
    texture::{Sample, Texture},
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct DiffuseLight {
    tex: Arc<Texture>,
}

impl DiffuseLight {
    pub fn new(tex: Arc<Texture>) -> Self {
        Self { tex }
    }
}

impl Scatter for DiffuseLight {
    fn emitted(&self, u: f32, v: f32, p: Point3) -> Color {
        self.tex.sample(u, v, p)
    }
}
