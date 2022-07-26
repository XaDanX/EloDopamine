extern crate core;
use crate::{Game, GameObject, HERO_INTERFACE, Memory, UnitData};
use crate::game_object::{MinionObject, TurretObject};
use crate::offsets::{LOCAL_PLAYER, MINION_INTERFACE, TURRET_INTERFACE};

pub struct GameManager {
    pub memory: Memory,
    pub local_player: GameObject,
    pub hero_list: Vec<GameObject>,
    pub minion_list: Vec<MinionObject>,
    pub turret_list: Vec<TurretObject>,
    pub game: Game,
    pub enemy_minion_list: Vec<MinionObject>,
    pub ally_minion_list: Vec<MinionObject>,
    pub unit_data: UnitData
}

impl GameManager {
    pub fn new(memory: Memory) -> GameManager {

        let mut new_structure: GameManager = GameManager {
            memory,
            local_player: GameObject::new(0),
            hero_list: Vec::<GameObject>::new(),
            minion_list: Vec::<MinionObject>::new(),
            enemy_minion_list: Vec::<MinionObject>::new(),
            ally_minion_list: Vec::<MinionObject>::new(),
            turret_list: Vec::<TurretObject>::new(),
            game: Game::new(),
            unit_data: UnitData::new()
        };
        new_structure.reread();
        new_structure.update();

        return new_structure;
    }

    pub fn update(&mut self) {
        self.local_player.update(self.memory);
        self.game.update(self.memory);

        for hero in &mut self.hero_list {
            hero.update(self.memory);
        }

        for turret in &mut self.turret_list {
            turret.update(self.memory);
        }

        /*
            Read and update all minions
        */
        let minion_pointer_list = self.memory.read_template(MINION_INTERFACE);
        self.minion_list.clear();
        self.enemy_minion_list.clear();
        self.ally_minion_list.clear();
        for minion in minion_pointer_list {
            let mut current_minion = MinionObject::new(minion);
            current_minion.update(self.memory);
            if current_minion.team == 100 || current_minion.team == 200 && current_minion.movement_speed > 150.0 {
                let mut tmp_obj = MinionObject::new(minion);
                tmp_obj.update(self.memory);
                if current_minion.team == self.local_player.team {
                    self.ally_minion_list.push(tmp_obj);
                } else {
                    self.enemy_minion_list.push(tmp_obj)
                }
                self.minion_list.push(current_minion);
            }
        }

    }

    pub fn reread(&mut self) {
        let local_player_ptr: u32 = self.memory.read::<u32>(self.memory.base_address + LOCAL_PLAYER);
        self.local_player = GameObject::new(local_player_ptr);

        let hero_pointer_list = self.memory.read_template(HERO_INTERFACE);
        let turret_pointer_list = self.memory.read_template(TURRET_INTERFACE);

        self.hero_list.clear();
        self.turret_list.clear();
        for hero in hero_pointer_list {
            self.hero_list.push(GameObject::new(hero));
        }

        for turret in turret_pointer_list {
            self.turret_list.push(TurretObject::new(turret));
        }

    }

}