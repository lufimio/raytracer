mod camera;
mod geometry;
mod hittable;
mod material;
mod texture;

use crate::{
    camera::Camera,
    geometry::{Color, Point3, Vec3},
    hittable::{
        HittableList,
        bvh::BVHNode,
        constant_density_medium::ConstantDensityMedium,
        quad::{Quad, make_box},
        sphere::Sphere,
        transformations::{Rotation, Translation},
    },
    material::{dielectric::Dielectric, lambertian::Lambertian, light::DiffuseLight, metal::Metal},
    texture::{Texture, checker::Checker, image::Image},
};
use glam::{EulerRot, Quat};
use rand::{random, random_range};
use std::{f32, sync::Arc};

fn setup_scattered_balls(image_width: u32, samples_per_pixel: u32) {
    let mut world = HittableList::empty();

    let ground_texture = Arc::new(
        Checker::new(
            0.32,
            Arc::new(Color::new(0.2, 0.3, 0.1).into()),
            Arc::new(Color::new(0.9, 0.9, 0.9).into()),
        )
        .into(),
    );
    let ground_material = Arc::new(Lambertian::new(Arc::clone(&ground_texture)).into());
    world.add(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        Arc::clone(&ground_material),
    ));

    for a in -11..=11 {
        for b in -11..=11 {
            let mat = random_range(0..20);
            let center = Point3::new(
                a as f32 + 0.9 * random::<f32>(),
                0.2,
                b as f32 + 0.9 * random::<f32>(),
            );

            if Vec3::length(center - Point3::new(4., 0.2, 0.)) > 0.9 {
                let sphere_material = Arc::new(if mat < 16 {
                    let albedo: Arc<Texture> =
                        Arc::new((random::<Color>() * random::<Color>()).into());
                    Lambertian::new(albedo).into()
                } else if mat < 19 {
                    let albedo = 0.5 + random::<Color>() * 0.5;
                    let fuzz = random_range(0.0..0.5);
                    Metal::new(albedo, fuzz).into()
                } else {
                    Dielectric::new(1.5).into()
                });
                world.add(Sphere::new(center, 0.2, sphere_material));
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5).into());
    world.add(Sphere::new(
        Point3::new(0., 1., 0.),
        1.0,
        Arc::clone(&material1),
    ));

    // let material2 = Arc::new(Dielectric::new(1.0 / 1.5).into());
    // world.add(Sphere::new(
    //     Point3::new(0., 1., 0.),
    //     0.8,
    //     Arc::clone(&material2),
    // ));

    let material3 = Arc::new(Lambertian::new(Arc::new(Color::new(0.4, 0.2, 0.1).into())).into());
    world.add(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.0,
        Arc::clone(&material3),
    ));

    let material4 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.1).into());
    world.add(Sphere::new(
        Point3::new(4., 1., 0.),
        1.0,
        Arc::clone(&material4),
    ));

    let world = BVHNode::from_hittable_list(world);

    let camera = Camera::new(
        16. / 9.,                  // aspect ratio
        image_width,               // image width
        samples_per_pixel,         // samples per pixel
        50,                        // max depth
        20.0,                      // fov
        Point3::new(13., 2., 3.),  // look from
        Point3::new(0., 0., 0.),   // look at
        Vec3::new(0., 1., 0.),     // camera up
        Color::new(0.7, 0.8, 1.0), // background color
        0.6,                       // defocus angle
        10.0,                      // focus distance
    );

    camera.render(&world, "output/render.png");
}

fn setup_quads() {
    let mut world = HittableList::empty();

    let left_red = Arc::new(Lambertian::new(Arc::new(Color::new(1.0, 0.2, 0.2).into())).into());
    world.add(Quad::new(
        Point3::new(-3., -2., 5.),
        Vec3::new(0., 0., -4.),
        Vec3::new(0., 4., 0.),
        Arc::clone(&left_red),
    ));

    let back_green = Arc::new(Lambertian::new(Arc::new(Color::new(0.2, 1.0, 0.2).into())).into());
    world.add(Quad::new(
        Point3::new(-2., -2., 0.),
        Vec3::new(4., 0., 0.),
        Vec3::new(0., 4., 0.),
        Arc::clone(&back_green),
    ));

    let right_blue = Arc::new(Lambertian::new(Arc::new(Color::new(0.2, 0.2, 1.0).into())).into());
    world.add(Quad::new(
        Point3::new(3., -2., 1.),
        Vec3::new(0., 0., 4.),
        Vec3::new(0., 4., 0.),
        Arc::clone(&right_blue),
    ));

    let upper_orange = Arc::new(Lambertian::new(Arc::new(Color::new(1.0, 0.5, 0.0).into())).into());
    world.add(Quad::new(
        Point3::new(-2., 3., 1.),
        Vec3::new(4., 0., 0.),
        Vec3::new(0., 0., 4.),
        Arc::clone(&upper_orange),
    ));

    let lower_teal = Arc::new(Lambertian::new(Arc::new(Color::new(0.2, 0.8, 0.8).into())).into());
    world.add(Quad::new(
        Point3::new(-2., -3., 5.),
        Vec3::new(4., 0., 0.),
        Vec3::new(0., 0., -4.),
        Arc::clone(&lower_teal),
    ));

    let earth_texture = Arc::new(Image::new("assets/earthmap.jpg").into());
    let earth_surface = Arc::new(Lambertian::new(earth_texture).into());
    world.add(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.25, earth_surface));

    let camera = Camera::new(
        1.0,                       // aspect ratio
        600,                       // image width
        100,                       // samples per pixel
        50,                        // max depth
        70.0,                      // fov
        Point3::new(0., 0., 9.),   // look from
        Point3::new(0., 0., 0.),   // look at
        Vec3::new(0., 1., 0.),     // camera up
        Color::new(0.7, 0.8, 1.0), // background color
        0.0,                       // defocus angle
        10.0,                      // focus distance
    );

    camera.render(&world, "output/render.png");
}

fn setup_earth() {
    let earth_texture = Arc::new(Image::new("assets/earthmap.jpg").into());
    let earth_surface = Arc::new(Lambertian::new(earth_texture).into());
    let globe = Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface);

    let camera = Camera::new(
        16.0 / 9.0,                // aspect ratio
        1200,                      // image width
        100,                       // samples per pixel
        50,                        // max depth
        20.0,                      // fov
        Point3::new(-4., 12., 5.), // look from
        Point3::new(0., 0., 0.),   // look at
        Vec3::new(0., 1., 0.),     // camera up
        Color::new(0.7, 0.8, 1.0), // background color
        0.0,                       // defocus angle
        10.0,                      // focus distance
    );

    camera.render(&globe, "output/render.png");
}

fn setup_cornell_box(image_width: u32, samples_per_pixel: u32) {
    let mut world = HittableList::empty();

    let red = Arc::new(Lambertian::new(Arc::new(Color::new(0.65, 0.05, 0.05).into())).into());
    let white = Arc::new(Lambertian::new(Arc::new(Color::new(0.73, 0.73, 0.73).into())).into());
    let green = Arc::new(Lambertian::new(Arc::new(Color::new(0.12, 0.45, 0.15).into())).into());
    let light = Arc::new(DiffuseLight::new(Arc::new(Color::new(15.0, 15.0, 15.0).into())).into());

    world.add(Quad::new(
        Point3::new(555., 0., 0.),
        Vec3::new(0., 555., 0.),
        Vec3::new(0., 0., 555.),
        Arc::clone(&green),
    ));

    world.add(Quad::new(
        Point3::new(0., 0., 0.),
        Vec3::new(0., 555., 0.),
        Vec3::new(0., 0., 555.),
        Arc::clone(&red),
    ));

    world.add(Quad::new(
        Point3::new(343., 554., 332.),
        Vec3::new(-130., 0., 0.),
        Vec3::new(0., 0., -105.),
        Arc::clone(&light),
    ));

    world.add(Quad::new(
        Point3::new(0., 0., 0.),
        Vec3::new(555., 0., 0.),
        Vec3::new(0., 0., 555.),
        Arc::clone(&white),
    ));

    world.add(Quad::new(
        Point3::new(555., 555., 555.),
        Vec3::new(-555., 0., 0.),
        Vec3::new(0., 0., -555.),
        Arc::clone(&white),
    ));

    world.add(Quad::new(
        Point3::new(0., 0., 555.),
        Vec3::new(555., 0., 0.),
        Vec3::new(0., 555., 0.),
        Arc::clone(&white),
    ));

    let box1 = Translation::new(
        Rotation::new(
            make_box(
                Point3::new(0., 0., 0.),
                Point3::new(165., 330., 165.),
                Arc::clone(&white),
            ),
            Quat::from_rotation_y(15_f32.to_radians()),
        ),
        Vec3::new(265., 0., 295.),
    );

    world.add(ConstantDensityMedium::new(
        box1,
        0.01,
        Arc::new(Color::new(0.0, 0.0, 0.0).into()),
    ));

    let box2 = Translation::new(
        Rotation::new(
            make_box(
                Point3::new(0., 0., 0.),
                Point3::new(165., 165., 165.),
                Arc::clone(&white),
            ),
            Quat::from_rotation_y(-18_f32.to_radians()),
        ),
        Vec3::new(130., 0., 65.),
    );

    world.add(ConstantDensityMedium::new(
        box2,
        0.01,
        Arc::new(Color::new(1.0, 1.0, 1.0).into()),
    ));

    let camera = Camera::new(
        1.0,                            // aspect ratio
        image_width,                    // image width
        samples_per_pixel,              // samples per pixel
        50,                             // max depth
        40.0,                           // fov
        Point3::new(278., 278., -800.), // look from
        Point3::new(278., 278., 0.),    // look at
        Vec3::new(0., 1., 0.),          // camera up
        Color::new(0.0, 0.0, 0.0),      // background color
        0.0,                            // defocus angle
        10.0,                           // focus distance
    );

    camera.render(&BVHNode::from_hittable_list(world), "output/render.png");
}

fn setup_spheres_and_boxes(image_width: u32, samples_per_pixel: u32) {
    let mut boxes1 = HittableList::empty();
    let ground = Arc::new(Lambertian::new(Arc::new(Color::new(0.48, 0.83, 0.53).into())).into());
    for i in 0..20 {
        for j in 0..20 {
            let x0 = -1000.0 + i as f32 * 100.0;
            let y0 = 0.0;
            let z0 = -1000.0 + j as f32 * 100.0;
            boxes1.add(make_box(
                Point3::new(x0, y0, z0),
                Point3::new(x0 + 100.0, random_range(1.0..101.0), z0 + 100.0),
                Arc::clone(&ground),
            ));
        }
    }

    let mut world = HittableList::empty();
    world.add(BVHNode::from_hittable_list(boxes1));

    let light = Arc::new(DiffuseLight::new(Arc::new(Color::new(7., 7., 7.).into())).into());
    world.add(Quad::new(
        Point3::new(123., 554., 147.),
        Vec3::new(300., 0., 0.),
        Vec3::new(0., 0., 265.),
        Arc::clone(&light),
    ));

    world.add(ConstantDensityMedium::new(
        Sphere::new(
            Point3::new(400., 400., 200.),
            50.,
            Arc::new(Dielectric::new(1.5).into()),
        ),
        0.001,
        Arc::new(Color::new(0.53, 0.81, 0.92).into()),
    ));

    world.add(Sphere::new(
        Point3::new(260., 150., 45.),
        50.,
        Arc::new(Dielectric::new(1.5).into()),
    ));

    world.add(Sphere::new(
        Point3::new(0., 150., 145.),
        50.,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0).into()),
    ));

    let boundary = Sphere::new(
        Point3::new(360., 150., 145.),
        70.,
        Arc::new(Dielectric::new(1.5).into()),
    );
    world.add(boundary.clone());
    world.add(ConstantDensityMedium::new(
        boundary,
        0.2,
        Arc::new(Color::new(0.2, 0.4, 0.9).into()),
    ));

    world.add(ConstantDensityMedium::new(
        Sphere::new(
            Point3::new(0., 0., 0.),
            5000.,
            Arc::new(Dielectric::new(1.5).into()),
        ),
        0.0001,
        Arc::new(Color::new(1., 1., 1.).into()),
    ));

    world.add(Translation::new(
        Rotation::new(
            Sphere::new(
                Point3::new(0., 0., 0.),
                80.,
                Arc::new(
                    Lambertian::new(Arc::new(Image::new("assets/earthmap.jpg").into())).into(),
                ),
            ),
            Quat::from_rotation_x(25_f32.to_radians()),
        ),
        Vec3::new(220., 280., 300.),
    ));

    let mut boxes2 = HittableList::empty();

    let white = Arc::new(Lambertian::new(Arc::new(Color::new(0.73, 0.73, 0.73).into())).into());
    for _ in 0..1000 {
        boxes2.add(Sphere::new(
            random::<Point3>() * 165.0,
            10.0,
            Arc::clone(&white),
        ));
    }

    world.add(Translation::new(
        Rotation::new(
            BVHNode::from_hittable_list(boxes2),
            Quat::from_euler(EulerRot::YXZ, 15., 25., 0.),
        ),
        Vec3::new(0., 270., 395.),
    ));

    let camera = Camera::new(
        1.0,                            // aspect ratio
        image_width,                    // image width
        samples_per_pixel,              // samples per pixel
        50,                             // max depth
        40.0,                           // fov
        Point3::new(478., 278., -600.), // look from
        Point3::new(278., 278., 0.),    // look at
        Vec3::new(0., 1., 0.),          // camera up
        Color::new(0.0, 0.0, 0.0),      // background color
        0.0,                            // defocus angle
        10.0,                           // focus distance
    );

    camera.render(&world, "output/idkstuff.png");
}

fn main() {
    match 5 {
        1 => setup_scattered_balls(400, 50),
        2 => setup_quads(),
        3 => setup_earth(),
        4 => setup_cornell_box(300, 2000),
        5 => setup_spheres_and_boxes(200, 100),
        _ => (),
    }
}
