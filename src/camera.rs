use crate::ray_tracer::{types::Ray, RayTracer};
use glam::{IVec3, Vec3A};

pub struct Camera {
    pub img_height: u32,
    pub img_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,

    pub vertical_fov: f32,
    pub lookfrom: IVec3,
    pub lookat: IVec3,
    pub cam_up: Vec3A,

    pub defocus_angle: f32,
    pub focus_dist: f32,

    pixel_samples_scale: f32,
    center: IVec3,
    pixel00_loc: IVec3,

    pixel_delta_u: Vec3A,
    pixel_delta_v: Vec3A,

    u: Vec3A,
    v: Vec3A,
    w: Vec3A,

    defocus_disk_u: Vec3A,
    defocus_disk_v: Vec3A,
}

impl Camera {
    pub fn new(
        img_height: u32,
        img_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        vertical_fov: f32,
        lookfrom: IVec3,
        lookat: IVec3,
        cam_up: IVec3,
        defocus_angle: f32,
        focus_dist: f32,
    ) -> Self {
        let pixel_samples_scale = 1.0 / samples_per_pixel as f32;
        let center = lookfrom;

        let theta = Self::degrees_to_radians(vertical_fov);
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * img_width as f32 / img_height as f32;

        let w = (lookfrom - lookat).as_vec3a().normalize();
        let u = cam_up.as_vec3a().cross(w).normalize();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / img_width as f32;
        let pixel_delta_v = viewport_v / img_height as f32;

        let pixel00_loc =
            (center.as_vec3a() - (focus_dist * w) - (0.5 * viewport_u) - (0.5 * viewport_v)
                + (0.5 * (pixel_delta_u + pixel_delta_v)))
                .as_ivec3();

        let defocus_radius = focus_dist * Self::degrees_to_radians(0.5 * defocus_angle).tan();
        let defocus_disk_u = defocus_radius * u;
        let defocus_disk_v = defocus_radius * v;

        Self {
            img_height: 144,
            img_width: 144,
            samples_per_pixel: 12,
            max_depth: 10,
            vertical_fov: 90.0,
            lookfrom: IVec3::new(0, 0, 0),
            lookat: IVec3::new(0, 0, -1),
            cam_up: Vec3A::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            pixel_samples_scale,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            u,
            v,
            w,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub async fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc.as_vec3a()
            + ((i as f32 + offset.x) * self.pixel_delta_u)
            + ((j as f32 + offset.y) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center.as_vec3a()
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = (pixel_sample - ray_origin).normalize();

        Ray::new(ray_origin, ray_direction)
    }

    // Helper functions
    fn degrees_to_radians(degrees: f32) -> f32 {
        degrees * std::f32::consts::PI / 180.0
    }

    fn sample_square(&self) -> Vec3A {
        Vec3A::new(rand::random::<f32>(), rand::random::<f32>(), 0.0)
    }

    fn defocus_disk_sample(&self) -> Vec3A {
        let p = [rand::random::<f32>(), rand::random::<f32>()];
        self.center.as_vec3a() + (self.defocus_disk_u * p[0]) + (self.defocus_disk_v * p[1])
    }
}
