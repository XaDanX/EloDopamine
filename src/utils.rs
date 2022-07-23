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

pub fn is_on_screen(pos: &Vec2, offset_x: f32, offset_y: f32, width: f32, height: f32) -> bool {
    return -offset_y < pos.x && pos.x < (width + offset_x) && -offset_y < pos.y &&  pos.y < (height + offset_y)
}