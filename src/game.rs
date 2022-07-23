use crate::math::multiply_matrices;
use crate::Memory;
use crate::offsets::{GAME_WINDOW_HEIGHT, GAME_WINDOW_WIDTH, RENDERER, VIEW_PROJ_MATRICES};

pub struct Game {
    pub time: f32,
    pub view_proj_matrix: Vec<f32>,
    pub width: f32,
    pub height: f32,
    renderer: u32
}

impl Game {
    pub fn new() -> Game {
        return Game{ time: 0.0, view_proj_matrix: vec![0.0; 16], width: 0.0, height: 0.0, renderer: 0 };
    }

    pub fn update(&mut self, mut memory: Memory) {
        let mut view_matrix: Vec<f32> = vec![0.0; 16];
        let mut proj_matrix: Vec<f32> = vec![0.0; 16];
        self.view_proj_matrix.clear();
        view_matrix.clear();
        proj_matrix.clear();

        for i in 0..16 {
            let view_matrix_f: f32 = memory.read::<f32>(memory.base_address + VIEW_PROJ_MATRICES + (i*4));
            let proj_matrix_f: f32 = memory.read::<f32>(memory.base_address + VIEW_PROJ_MATRICES + 64 + (i*4));
            view_matrix.push(view_matrix_f);
            proj_matrix.push(proj_matrix_f);
        }

        self.view_proj_matrix = multiply_matrices(view_matrix, proj_matrix);

        if self.renderer == 0 {
            self.renderer = memory.read::<u32>(memory.base_address + RENDERER);
        }
        self.width = memory.read::<f32>(self.renderer + GAME_WINDOW_WIDTH);
        self.height = memory.read::<f32>(self.renderer + GAME_WINDOW_HEIGHT);


    }

}