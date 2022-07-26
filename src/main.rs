extern crate core;

use std::borrow::BorrowMut;
use imgui::{Condition, StyleColor, StyleVar};
use crate::drawing::draw_circle_at;
use crate::game_object::GameObject;
use crate::memory::{Memory};
use crate::game_manager::GameManager;
use crate::offsets::{HERO_INTERFACE};
use random_string::generate;
mod memory;
mod game_object;
mod vector;
mod offsets;
mod game_manager;
mod overlay;
mod game;
mod math;
mod utils;
mod drawing;
mod scripts;
mod unit_data;

use crate::game::Game;
use crate::math::w2s;
use crate::utils::{bypass_lol_sig, is_on_screen, rgba, SmoothRgb};
use winconsole::console::set_title;
use crate::overlay::{SHOW_MENU};
use crate::scripts::awareness::Awareness;
use crate::scripts::DrawingScript;
use crate::unit_data::UnitData;



fn main() {

    bypass_lol_sig();
    set_title(generate(16, "abcdefghijklmnopqrstuvwxyz").as_str()).unwrap();

    let memory: Memory = Memory::new("League of Legends (TM) Client");
    let mut game_manager = GameManager::new(memory);
    game_manager.update();

    let mut smooth = SmoothRgb::new();


    let system = overlay::init(generate(16, "abcdefghijklmnopqrstuvwxyz").as_str(), game_manager.game.width as f32, game_manager.game.height as f32);


    let mut drawing_script: DrawingScript = DrawingScript::new();
    let mut awareness: Awareness = Awareness::new();


    system.main_loop(move |_, ui| unsafe {

        let color: [f32; 4] = rgba([6.0, 92.0, 29.0, 255.0]);
        let color_active: [f32; 4] = rgba([6.0, 142.0, 29.0, 255.0]);

        let style = ui.push_style_var(StyleVar::WindowTitleAlign([0.5, 0.5]));
        let style = ui.push_style_color(StyleColor::WindowBg, rgba([22.0, 22.0, 22.0, 255.0]));
        let style = ui.push_style_color(StyleColor::TitleBg, color);
        let style = ui.push_style_color(StyleColor::TitleBgActive, color_active);
        let style = ui.push_style_color(StyleColor::TitleBgCollapsed, color);
        let style = ui.push_style_color(StyleColor::Header, color);
        let style = ui.push_style_color(StyleColor::HeaderActive, color_active);
        let style = ui.push_style_color(StyleColor::HeaderHovered, color);
        let style = ui.push_style_color(StyleColor::CheckMark, color_active);

        let bg_draw_list = ui.get_background_draw_list();
        bg_draw_list.add_text([250.0, 3.0], rgba(smooth.get_color(255.0)), "EloDopamine"); //rgba([200.0, 0.0, 200.0, 255.0])

        game_manager.update();

        drawing_script.update(game_manager.borrow_mut(), &bg_draw_list);
        awareness.update(game_manager.borrow_mut(), &bg_draw_list);


        if SHOW_MENU {
            drawing_script.gui(&ui);
            awareness.gui(&ui);
        }
    });







}
