use crate::{
    geometry::{Color, Interval, Point3, Ray, Vec3, color_to_rgb, random_in_unit_disk},
    hittable::Hittable,
    material::Scatter,
};
use image::{DynamicImage, ImageBuffer, ImageResult, Rgb};
use indicatif::{ProgressBar, ProgressStyle};

pub struct Camera {
    pub aspect_ratio: f32,
    pub fov: f32,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub cameraup: Vec3,
    pub background: Color,
    pub defocus_angle: f32,
    pub focus_distance: f32,
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pixel00_loc: Point3,
    pixel_delta_u: Point3,
    pixel_delta_v: Point3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        fov: f32,
        lookfrom: Point3,
        lookat: Point3,
        cameraup: Vec3,
        background: Color,
        defocus_angle: f32,
        focus_distance: f32,
    ) -> Self {
        let image_height = (image_width as f32 / aspect_ratio) as u32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let viewport_height = 2.0 * f32::tan(fov.to_radians() / 2.0) * focus_distance;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

        let w = (lookfrom - lookat).normalize();
        let u = Vec3::cross(cameraup, w).normalize();
        let v = Vec3::cross(w, u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        let viewport_top_left = lookfrom - focus_distance * w - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_top_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let defocus_radius = focus_distance * f32::tan(f32::to_radians(defocus_angle / 2.0));
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            aspect_ratio,
            fov,
            image_width,
            image_height,
            lookfrom,
            lookat,
            cameraup,
            background,
            defocus_angle,
            focus_distance,
            samples_per_pixel,
            max_depth,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render<T: Hittable + Send + Sync>(&self, world: &T, output_path: &str) {
        let bar = ProgressBar::new(
            (self.image_height * self.image_width * self.samples_per_pixel) as u64,
        )
        .with_style(
            ProgressStyle::with_template(
                "{wide_bar} {pos}/{len} [{elapsed_precise}/-{eta_precise}]",
            )
            .unwrap(),
        );

        let img = DynamicImage::from(ImageBuffer::from_par_fn(
            self.image_width,
            self.image_height,
            |x: u32, y: u32| -> Rgb<u8> { self.sample_pixel(world, &bar, x, y) },
        ));
        if let ImageResult::Err(error) = img.save(output_path) {
            eprintln!("Error writing image: {}", error)
        }

        bar.finish();
    }

    fn sample_pixel<T: Hittable>(&self, world: &T, bar: &ProgressBar, x: u32, y: u32) -> Rgb<u8> {
        let mut pixel_color = Color::ZERO;
        for _sample in 0..self.samples_per_pixel {
            let r = self.get_ray(x, y);
            pixel_color = pixel_color + self.get_ray_color(r, self.max_depth, world);
            bar.inc(1);
        }
        pixel_color = pixel_color / self.samples_per_pixel as f32;
        color_to_rgb(pixel_color)
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = 0.5 * random_in_unit_disk();
        let pixel_center = self.pixel00_loc
            + ((i as f32 + offset.x) * self.pixel_delta_u)
            + ((j as f32 + offset.y) * self.pixel_delta_v);

        Ray::new(
            if self.defocus_angle <= 0.0 {
                self.lookfrom
            } else {
                self.defocus_disk_sample()
            },
            pixel_center - self.lookfrom,
        )
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.lookfrom + p.x * self.defocus_disk_u + p.y * self.defocus_disk_v
    }

    fn get_ray_color<T: Hittable>(&self, r: Ray, depth: u32, world: &T) -> Color {
        if depth <= 0 {
            return Color::ZERO;
        }

        if let Some(rec) = world.hit(r, Interval::new(0.001, f32::INFINITY)) {
            let emitted = rec.mat.emitted(rec.u, rec.v, rec.p);
            if let Some((scattered, attenuation)) = rec.mat.scatter(r, &rec) {
                emitted + attenuation * self.get_ray_color(scattered, depth - 1, world)
            } else {
                emitted
            }
        } else {
            self.background
        }
    }
}
