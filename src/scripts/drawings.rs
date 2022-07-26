use std::borrow::{Borrow, BorrowMut};
use std::fs::copy;
use imgui::{CollapsingHeader, Condition, DrawListMut, Ui};
use crate::{draw_circle_at, GameManager, GameObject, is_on_screen, rgba, w2s};

pub struct DrawingScript {
    draw_enemy_ranges: bool,
    draw_ally_ranges: bool,
    draw_enemy_turret_ranges: bool,
    draw_ally_turret_ranges: bool,
    draw_ally_path: bool,
    draw_enemy_path: bool,
}


impl DrawingScript { // TODO::Rewrite whole module.

    pub fn new() -> DrawingScript {
        return DrawingScript{
            draw_enemy_ranges:true,
            draw_ally_ranges: true,
            draw_enemy_turret_ranges: true,
            draw_ally_turret_ranges: true,
            draw_ally_path: true,
            draw_enemy_path: true
        };
    }

    pub unsafe fn update(&mut self, game_manager: &mut GameManager, draw_list: &DrawListMut) {
        if self.draw_enemy_ranges || self.draw_ally_ranges || self.draw_ally_path || self.draw_enemy_path {
            for mut hero in &game_manager.hero_list {
                if !(hero.health > 1.0 && hero.visibility) { continue } //TODO: check if visible etc
                let hero_world_pos = w2s(&hero.position, &game_manager.game.view_proj_matrix);
                if hero_world_pos.is_none() { continue; }
                let mut hero_unwrap = hero_world_pos.unwrap();
                if hero.team == game_manager.local_player.team && self.draw_ally_path == true {
                    if hero.ai_manager.is_moving {
                        let s_path = w2s(&hero.ai_manager.start_path, &game_manager.game.view_proj_matrix);
                        let e_path = w2s(&hero.ai_manager.end_path, &game_manager.game.view_proj_matrix);


                        if !s_path.is_none() && !e_path.is_none() {
                            let mut e_path_unwrap = e_path.unwrap();
                            draw_list.add_line(hero_unwrap.list(), e_path_unwrap.list(), rgba([255.0, 220.0, 220.0, 100.0])).thickness(3.0).build();
                            draw_list.add_text(e_path_unwrap.list(), rgba([50.0, 220.0, 50.0, 100.0]), &hero.champion_name);
                        }
                    }
                }
                if hero.team != game_manager.local_player.team && self.draw_enemy_path == true {
                    if hero.ai_manager.is_moving {
                        let s_path = w2s(&hero.ai_manager.start_path, &game_manager.game.view_proj_matrix);
                        let e_path = w2s(&hero.ai_manager.end_path, &game_manager.game.view_proj_matrix);

                        if !s_path.is_none() && !e_path.is_none() {
                            let mut e_path_unwrap = e_path.unwrap();
                            draw_list.add_line(hero_unwrap.list(), e_path_unwrap.list(), rgba([255.0, 220.0, 220.0, 100.0])).thickness(3.0).build();
                            draw_list.add_text(e_path_unwrap.list(), rgba([50.0, 220.0, 50.0, 100.0]), &hero.champion_name);
                        }
                    }
                }
                if hero.team == game_manager.local_player.team && self.draw_ally_ranges == false { continue; }
                if hero.team != game_manager.local_player.team && self.draw_enemy_ranges == false { continue; }
                if !is_on_screen(&hero_unwrap, 0.0, 0.0, 1920, 1080) { continue; }

                let gameplay_radius = game_manager.unit_data.get_champion(hero.champion_name.to_lowercase().as_str()).gameplay_radius;

                draw_circle_at(&draw_list, &game_manager, &hero.position, gameplay_radius + hero.attack_range, 3.0, true, rgba([120.0, 15.0, 51.0, 60.0]));
                draw_circle_at(&draw_list, &game_manager, &hero.position, gameplay_radius + hero.attack_range, 3.0, false, rgba([220.0, 15.0, 102.0, 140.0]));
            }
        }
        if self.draw_enemy_turret_ranges || self.draw_ally_turret_ranges {
            for turret in &game_manager.turret_list {
                if !(turret.health > 1.0) { continue } //TODO: check if visible etc
                if turret.team == game_manager.local_player.team && self.draw_ally_turret_ranges == false { continue; }
                if turret.team != game_manager.local_player.team && self.draw_enemy_turret_ranges == false { continue; }
                let turret_world_pos = w2s(&turret.position, &game_manager.game.view_proj_matrix);
                if turret_world_pos.is_none() { continue; }
                if !is_on_screen(&turret_world_pos.unwrap(), 0.0, 0.0, 1920, 1080) { continue; }

                draw_circle_at(&draw_list, &game_manager, &turret.position, 850.0 + 20.0, 3.0, true, rgba([82.0, 5.0, 0.0, 60.0]));
                draw_circle_at(&draw_list, &game_manager, &turret.position, 850.0 + 20.0, 3.0, false, rgba([82.0 * 2.0, 5.0 * 2.0, 0.0, 140.0]));
            }
        }
    }


    pub fn gui(&mut self, ui: &Ui) {
        ui.window("Drawings")
            .size([400.0, 600.0], Condition::FirstUseEver)
            .position([0.0, 0.0], Condition::Appearing)
            .resizable(false)
            .build(|| {

                if CollapsingHeader::new("Ranges").build(ui) {
                    if ui.radio_button_bool("Draw enemy turret ranges", *&mut self.draw_enemy_turret_ranges) {
                        self.draw_enemy_turret_ranges = !self.draw_enemy_turret_ranges;
                    }
                    ui.same_line();
                    if ui.radio_button_bool("Draw ally turret ranges", *&mut self.draw_ally_turret_ranges) {
                        self.draw_ally_turret_ranges = !self.draw_ally_turret_ranges;
                    }
                    ui.separator();
                    if ui.radio_button_bool("Draw enemy ranges", *&mut self.draw_enemy_ranges) {
                        self.draw_enemy_ranges = !self.draw_enemy_ranges;
                    }
                    ui.same_line();
                    if ui.radio_button_bool("Draw ally ranges", *&mut self.draw_ally_ranges) {
                        self.draw_ally_ranges = !self.draw_ally_ranges;
                    }
                    ui.separator();
                }
                ui.separator();
                if CollapsingHeader::new("Paths").build(ui) {
                    if ui.radio_button_bool("Draw ally path", *&mut self.draw_ally_path) {
                        self.draw_ally_path = !self.draw_ally_path;
                    }
                    ui.same_line();
                    if ui.radio_button_bool("Draw enemy path", *&mut self.draw_enemy_path) {
                        self.draw_enemy_path = !self.draw_enemy_path;
                    }
                }
            });
    }
}