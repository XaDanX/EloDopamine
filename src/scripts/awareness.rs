use std::borrow::Cow;
use glium::texture::{ClientFormat, RawImage2d};
use imgui::{Condition, DrawListMut, Ui};
use imgui_glium_renderer::Texture;
use crate::GameManager;

pub struct Awareness {
}

impl Awareness {

    pub fn new() -> Awareness {


        return Awareness{
        };
    }

    pub unsafe fn update(&mut self, game_manager: &mut GameManager, draw_list: &DrawListMut) {



    }


    pub fn gui(&mut self, ui: &Ui) {
        ui.window("Awareness")
            .size([400.0, 600.0], Condition::FirstUseEver)
            .position([400.0, 0.0], Condition::Appearing)
            .resizable(false)
            .build(|| {


            });
    }
}