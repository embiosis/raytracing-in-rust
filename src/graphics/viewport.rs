use bmp::Image;
use super::{vec3::Vec3, ray::Ray};

pub struct Viewport {
    pub image_height: u32,
    pub image_width: u32,
    pub focal_length: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub camera_center: Vec3,
    pub viewport_center: Vec3,
    pub viewport_i: Vec3,
    pub viewport_j: Vec3,
    pub pixel_delta_i: Vec3,
    pub pixel_delta_j: Vec3,
    pub viewport_upper_left: Vec3,
}

impl Viewport {
    // TODO: Error checking to ensure pixel coord is within bounds?
    pub fn get_pixel_coord(&self, x: u32, y: u32) -> Vec3 {
        self.viewport_upper_left + (0.5 + x as f64) * self.pixel_delta_i + (0.5 + y as f64) * self.pixel_delta_j
    }

    // TODO: Refactor this to make it less messy (reuse previous values)
    pub fn new(image_width: u32, focal_length: f64, viewport_height: f64, aspect_ratio: f64, camera_center: Vec3) -> Viewport {
        Viewport {
            image_height: std::cmp::max((image_width as f64 / aspect_ratio) as u32, 1),
            image_width,
            focal_length,
            viewport_height,
            viewport_width: viewport_height * aspect_ratio,
            camera_center,
            viewport_center: Vec3::new(0.0, 0.0, focal_length),
            viewport_i: Vec3::new(viewport_height, 0.0, 0.0),
            viewport_j: Vec3::new(0.0, -viewport_height, 0.0), // invert y-axis due to orientation of viewport coordinates
            pixel_delta_i: Vec3::new(viewport_height, 0.0, 0.0) / image_width as f64 / aspect_ratio,
            pixel_delta_j: Vec3::new(0.0, -viewport_height, 0.0) / (image_width as f64),
            viewport_upper_left: camera_center 
                                - Vec3::new(0.0, 0.0, focal_length) 
                                - Vec3::new(viewport_height, 0.0, 0.0) / 2.0 
                                - Vec3::new(0.0, -viewport_height, 0.0) / 2.0
        }
    }

    pub fn render(&self, save_path: String) {
        let mut img = Image::new(self.image_width, self.image_height);

        for y in 0..self.image_height {
            for x in 0..self.image_width {
                let pixel_center = self.get_pixel_coord(x, y);
                let ray_dir = pixel_center - self.camera_center;
                let ray = Ray { origin: self.camera_center, direction: ray_dir };
                img.set_pixel(x, y, ray.lerp()); 
            }
        }

        let _ = img.save(save_path);
    }
}