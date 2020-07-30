use valora::prelude::*;

#[derive(Debug)]
pub struct Background {
    pub color: LinSrgb,
    pub width: f32,
    pub height: f32,
    pub hypotenuse: f32,
}

impl Background {
    pub fn new(color_val: (f32, f32, f32), width: f32, height: f32) -> Self {
        Self {
            color: LinSrgb::new(color_val.0, color_val.1, color_val.2),
            width: width,
            height: height,
            hypotenuse: Background::calculate_hypotenuse(height, width),
        }
    }

    fn calculate_hypotenuse(height: f32, width: f32) -> f32 {
        (height.powf(2.0) + width.powf(2.0)).sqrt()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Polygon {
    pub color: LinSrgb,
    pub ngon: Ngon,
    pub angle: Angle,
    pub scale_factor: f32,
}

impl Polygon {
    pub fn new(
        color_val: f32,
        center_point: P2,
        radius: f32,
        number_vertex: usize,
        angle: f32,
        scale: f32,
    ) -> Self {
        Self {
            color: LinSrgb::new(color_val, color_val, color_val),
            ngon: Ngon::new(center_point, number_vertex, radius),
            angle: Angle::degrees(angle),
            scale_factor: scale,
        }
    }

    pub fn scale_rotate_figure(&mut self) {
        Polygon::rotate(self);
        self.ngon = self.ngon.scale(self.scale_factor);
    }

    pub fn rotate(&mut self) {
        self.ngon = self.ngon.rotate(self.ngon.center, self.angle);
    }

    pub fn change_color_to(&mut self, new_color: f32) {
        self.color.red = new_color;
        self.color.green = new_color;
        self.color.blue = new_color;
    }
}
