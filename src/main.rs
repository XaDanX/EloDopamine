extern crate core;

use std::ops::BitXor;
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

use crate::game::Game;
use crate::math::w2s;
use crate::utils::{bypass_lol_sig, is_on_screen, rgba};
use winconsole::console::set_title;
use crate::vector::Vec2;

const AI_MGR: i32 = 0x2C7C;


/*
fn get_ai_mgr(base: i32, mut memory: Memory) -> i32 {
    let v1 = memory.read::<i32>((base + AI_MGR) as u32);
    if v1 == 0 {
        return 0;
    }
    let v2 = base + AI_MGR - 8;
    let v4 = memory.read::<i32>((v2 + 1) as u32);
    if v4 == 0 {
        return 0;
    }
    let v7 = memory.read::<i32>((v2 + 4) as u32);
    if v7 == 0 {
        return 0;
    }
    let mut v13 = memory.read::<i32>((v2 + (4 * v1 + 12)) as u32);
    if v13 == 0 {
        return 0;
    }
    v13 = v13.bitxor(v7.reverse_bits());
    if v13 <= 0 || (v13 >= 0x7FFFFFFF) {
        return 0;
    }
    let ret = memory.read::<i32>((v13 + 8) as u32);
    if ret <= 0 || ret >= 0x7FFFFFFF {
        return 0
    } else {
        return ret;
    }

}

 */


fn main() {
    bypass_lol_sig();
    set_title(generate(16, "abcdefghijklmnopqrstuvwxyz").as_str()).unwrap();

    let memory: Memory = Memory::new("League of Legends (TM) Client");
    let mut game_manager = GameManager::new(memory);
    game_manager.update();


    let system = overlay::init(generate(16, "abcdefghijklmnopqrstuvwxyz").as_str(), game_manager.game.width, game_manager.game.height);

    system.main_loop(move |_, ui| unsafe {
        let bg_draw_list = ui.get_background_draw_list();
        bg_draw_list.add_text([250.0, 3.0], rgba([200.0, 0.0, 200.0, 255.0]), "EloDopamine");

        let mut player_pos = game_manager.local_player.position.copy();
        let mut player_direction = game_manager.local_player.direction.copy();
        let mut scaled = player_direction.scale(280.0);
        let out = player_pos.add_vector(scaled);
        let world = w2s(&out, &game_manager.game.view_proj_matrix);
        let player_pos = w2s(&player_pos, &game_manager.game.view_proj_matrix);

        if !player_pos.is_none() && !world.is_none() {
            bg_draw_list.add_line(player_pos.unwrap().list(), world.unwrap().list(), [0.0, 0.1, 0.7, 0.7]).thickness(8.0).build();
        }





        //draw_circle_at(&bg_draw_list, &game_manager, &game_manager.local_player.position, game_manager.local_player.attack_range + 55.0, 3.0, true, rgba([120.0, 15.0, 51.0, 60.0]));
        //draw_circle_at(&bg_draw_list, &game_manager, &game_manager.local_player.position, game_manager.local_player.attack_range + 55.0, 3.0, false, rgba([220.0, 15.0, 102.0, 140.0]));

        game_manager.update();

        for minion in &game_manager.minion_list {
            if !(minion.health > 1.0) {continue}
            let minion_world_pos = w2s(&minion.position, &game_manager.game.view_proj_matrix);
            if minion_world_pos.is_none() {continue;}
            if !is_on_screen(&minion_world_pos.unwrap(), 0.0, 0.0, 1920.0, 1080.0) {continue;}

            draw_circle_at(&bg_draw_list, &game_manager, &minion.position, 55.0, 3.0, true, rgba([1.0, 122.0, 25.0, 60.0]));
            draw_circle_at(&bg_draw_list, &game_manager, &minion.position, 55.0, 3.0, false, rgba([2.0, 244.0, 50.0, 140.0]));
        }

        for hero in &game_manager.hero_list {
            if !(hero.health > 1.0) {continue}
            let minion_world_pos = w2s(&hero.position, &game_manager.game.view_proj_matrix);
            if minion_world_pos.is_none() {continue;}
            if !is_on_screen(&minion_world_pos.unwrap(), 0.0, 0.0, 1920.0, 1080.0) {continue;}

            draw_circle_at(&bg_draw_list, &game_manager, &hero.position, 55.0 + hero.attack_range, 3.0, true, rgba([120.0, 15.0, 51.0, 60.0]));
            draw_circle_at(&bg_draw_list, &game_manager, &hero.position, 55.0 + hero.attack_range, 3.0, false, rgba([220.0, 15.0, 102.0, 140.0]));
        }

        for turret in &game_manager.turret_list {
            if !(turret.health > 1.0) {continue}
            let minion_world_pos = w2s(&turret.position, &game_manager.game.view_proj_matrix);
            if minion_world_pos.is_none() {continue;}
            if !is_on_screen(&minion_world_pos.unwrap(), 0.0, 0.0, 1920.0, 1080.0) {continue;}

            draw_circle_at(&bg_draw_list, &game_manager, &turret.position, 850.0 + 20.0, 3.0, true, rgba([82.0, 5.0, 0.0, 60.0]));
            draw_circle_at(&bg_draw_list, &game_manager, &turret.position, 850.0 + 20.0, 3.0, false, rgba([82.0*2.0, 5.0*2.0, 0.0, 140.0]));
        }

    });





}
