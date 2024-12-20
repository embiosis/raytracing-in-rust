use bmp::Image;
use super::{colour::*, ray::Ray, vec3::Vec3};

pub struct Viewport {
    pub image_width: u32,
    pub image_height: u32,
    pub focal_length: f64,
    pub viewport_width: f64,
    pub viewport_height: f64,
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
    pub fn new(
        image_width: u32,
        image_height: u32,
        focal_length: f64, 
        viewport_size: f64,
        camera_center: Vec3
    ) -> Viewport {
        // Ensure the image_height is not 0.
        assert_ne!(image_height, 0);
        
        // Calculate the actual aspect ratio.
        let aspect_ratio = image_width as f64 / image_height as f64;
        let viewport_width = viewport_size * aspect_ratio;
        let viewport_height = viewport_size;

        Viewport {
            image_width,
            image_height,
            focal_length,
            viewport_width,
            viewport_height,
            camera_center,
            viewport_center: Vec3::new(0.0, 0.0, focal_length),
            viewport_i: Vec3::new(viewport_width, 0.0, 0.0),
            // invert y-axis due to orientation of viewport coordinates
            viewport_j: Vec3::new(0.0, -viewport_height, 0.0), 
            pixel_delta_i: Vec3::new(viewport_width, 0.0, 0.0) / image_width as f64,
            pixel_delta_j: Vec3::new(0.0, -viewport_height, 0.0) / image_height as f64,
            viewport_upper_left: camera_center 
                                - Vec3::new(0.0, 0.0, focal_length) 
                                - Vec3::new(viewport_width, 0.0, 0.0) / 2.0 
                                - Vec3::new(0.0, -viewport_height, 0.0) / 2.0
        }
    }

    // Chapter 2.2 image
    pub fn test_file_format(&self, save_path: String) {
        let mut img = Image::new(self.image_width, self.image_height);

        for (x, y) in img.coordinates() {
            img.set_pixel(x, y, Colour::new(
                x as f64 / (self.image_width - 1) as f64 * RGB_SCALE,
                y as f64 / (self.image_height - 1) as f64 * RGB_SCALE,
                0.0
            ).into());
        }

        img.save(save_path).expect("An error occurred while saving to {save_path}!")
    }

    // Chapter 4.2 image
    pub fn test_gradient(&self, save_path: String) {
        let mut img = Image::new(self.image_width, self.image_height);

        for (x, y) in img.coordinates() {
            let pixel_center = self.get_pixel_coord(x, y);
            let ray_dir = pixel_center - self.camera_center;
            let ray = Ray { origin: self.camera_center, direction: ray_dir };
            img.set_pixel(x, y, ray.gradient(Colour::white(), Colour::new(128, 179, 255)));
        }

        img.save(save_path).expect("An error occurred while saving to {save_path}!")
    }

    // Test whether the viewport is centred on the camera.
    pub fn test_viewport(&self, save_path: String) {
        let mut img = Image::new(self.image_width, self.image_height);

        for (x, y) in img.coordinates() {
            let pixel_center = self.get_pixel_coord(x, y);
            let ray_dir = pixel_center - self.camera_center;
            let ray = Ray { origin: self.camera_center, direction: ray_dir };
            img.set_pixel(x, y, ray.center_viewport());
        }

        img.save(save_path).expect("An error occurred while saving to {save_path}!")
    }

    // Chapter 5.2 image
    pub fn test_sphere(&self, save_path: String) {
        let mut img = Image::new(self.image_width, self.image_height);

        for (x, y) in img.coordinates() {
            let pixel_center = self.get_pixel_coord(x, y);
            let ray_dir = pixel_center - self.camera_center;
            let ray = Ray { origin: self.camera_center, direction: ray_dir };
            img.set_pixel(x, y, ray.get_colour());
        }

        img.save(save_path).expect("An error occurred while saving to {save_path}!")
    }

    // Chapter 6.1 image
    pub fn test_normals(&self, save_path: String) {
        let mut img = Image::new(self.image_width, self.image_height);

        for (x, y) in img.coordinates() {
            let pixel_center = self.get_pixel_coord(x, y);
            let ray_dir = pixel_center - self.camera_center;
            let ray = Ray { origin: self.camera_center, direction: ray_dir };
            img.set_pixel(x, y, ray.get_normals());
        }

        img.save(save_path).expect("An error occurred while saving to {save_path}!")
    }
}