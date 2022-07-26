use std::arch::asm;
use rand::Rng;
use crate::vector::Vec2;

pub fn rgba(color: [f64; 4]) -> [f32; 4] {
    let mut new = [0.0; 4];
    for x in 0..color.len() {
        new[x] = (color[x] / 255.0) as f32;
    }
    return new;
}

pub fn bypass_lol_sig() {
    let rng: u32 = rand::thread_rng().gen_range(0..2137);

    for _i in 0..rng {
        unsafe {
            asm!(
            "add eax, 1",
            );
        }
    }

}

pub fn is_on_screen(pos: &Vec2, offset_x: f32, offset_y: f32, width: i32, height: i32) -> bool {
    return -offset_y < pos.x && pos.x < (width as f32 + offset_x) && -offset_y < pos.y &&  pos.y < (height as f32 + offset_y)
}

const STEP: f64 = 0.7;
pub struct SmoothRgb {
    state: i32,
    r: f64,
    g: f64,
    b: f64
}

impl SmoothRgb {
    pub fn new() -> SmoothRgb {
        return SmoothRgb{ state: 0, r: 255.0, g: 0.0, b: 0.0 }
    }
    pub fn get_color(&mut self, alpha: f64) -> [f64; 4] {
        if self.state == 0 {
            self.g += STEP;
            if self.g >= 254.0 {
                self.state = 1;
            }
        }
        if self.state == 1 {
            self.r -= STEP;
            if self.r <= 1.0 {
                self.state = 2;
            }
        }
        if self.state == 2 {
            self.b += STEP;
            if self.b >= 254.0 {
                self.state = 3;
            }
        }
        if self.state == 3 {
            self.g -= STEP;
            if self.g <= 1.0 {
                self.state = 4;
            }
        }
        if self.state == 4 {
            self.r += STEP;
            if self.r >= 254.0 {
                self.state = 5;
            }
        }
        if self.state == 5 {
            self.b -= STEP;
            if self.b <= 1.0 {
                self.state = 0;
            }
        }
        return [self.r, self.g, self.b, alpha];
    }
}