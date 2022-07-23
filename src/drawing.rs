use std::f32::consts::PI;
use imgui::{DrawListMut};
use crate::{GameManager, w2s};
use crate::vector::Vec3;

pub unsafe fn draw_circle_at(ui: &DrawListMut, game_manager: &GameManager, position: &Vec3, radius: f32, thickness: f32, filled: bool, color: [f32; 4]) {
    let mut points = vec![[0.0 as f32; 2]; 100 as usize];

    let step: f32 = PI * 2.0 / 100.0;
    let mut theta: f32 = 0.0;
    let mut i = 0;

    while theta < (PI * 2.0) {
        let pos = Vec3::new(position.x + (radius * theta.cos()), position.y, position.z - (radius * theta.sin()));
        let screen_pos = w2s(&pos, &game_manager.game.view_proj_matrix);
        if !screen_pos.is_none() {
            points[i] = screen_pos.unwrap().list();
        }
        i += 1;
        theta += step;
    }
    points.push(points[0]);
    ui.add_polyline(points, color).thickness(thickness).filled(filled).build();
}