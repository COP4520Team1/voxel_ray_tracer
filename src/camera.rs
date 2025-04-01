use crate::ray_tracer::types::Ray;
use glam::Vec3A;

pub struct Camera {
    img_height: usize,
    img_width: usize,
    vertical_fov: f32,
    lookfrom: Vec3A,
    lookat: Vec3A,
    cam_up: Vec3A,
    focus_dist: f32,
    center: Vec3A,
    pixel00_loc: Vec3A,
    pixel_delta_u: Vec3A,
    pixel_delta_v: Vec3A,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            7680,
            4320,
            90.0,
            240.0 * Vec3A::ONE,
            Vec3A::ZERO,
            Vec3A::Y,
            10.0,
        )
    }
}

impl Camera {
    pub fn from_res_and_pos(width: usize, height: usize, pos: Vec3A) -> Self {
        Self::new(width, height, 90.0, pos, Vec3A::ZERO, Vec3A::Y, 10.0)
    }

    pub fn new(
        img_width: usize,
        img_height: usize,
        vertical_fov: f32,
        lookfrom: Vec3A,
        lookat: Vec3A,
        cam_up: Vec3A,
        focus_dist: f32,
    ) -> Self {
        let center = lookfrom;

        let theta = Self::degrees_to_radians(vertical_fov);
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * img_width as f32 / img_height as f32;

        let w = (lookfrom - lookat).normalize();
        let u = cam_up.cross(w).normalize();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / img_width as f32;
        let pixel_delta_v = viewport_v / img_height as f32;

        let pixel00_loc = center - (focus_dist * w) - (0.5 * viewport_u) - (0.5 * viewport_v)
            + (0.5 * (pixel_delta_u + pixel_delta_v));

        Self {
            img_height,
            img_width,
            vertical_fov,
            lookfrom,
            lookat,
            cam_up,
            focus_dist,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn get_ray(&self, i: usize, j: usize) -> Ray {
        let pixel_sample = self.pixel00_loc
            + ((i as f32) * self.pixel_delta_u)
            + ((j as f32) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = (pixel_sample - ray_origin).normalize();

        Ray::new(ray_origin, ray_direction)
    }

    // Helper functions
    fn degrees_to_radians(degrees: f32) -> f32 {
        degrees * std::f32::consts::PI / 180.0
    }
}
