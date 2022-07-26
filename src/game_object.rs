extern crate core;

use std::alloc::dealloc;
use std::ops::{BitXor, Not};
use imgui::sys::cty::c_longlong;
use winapi::shared::minwindef::DWORD;
use crate::{GameManager, Memory, w2s};
use crate::vector::{Vec2, Vec3};
use crate::offsets::{AI_MANAGER, AI_MANAGER_END_PATH, AI_MANAGER_IS_MOVING, AI_MANAGER_START_PATH, OBJECT_ARMOR, OBJECT_ATTACK_DAMAGE, OBJECT_ATTACK_RANGE, OBJECT_BONUS_ATTACK_DAMAGE, OBJECT_CHAMPION_NAME, OBJECT_DEAD, OBJECT_DIRECTION, OBJECT_HEALTH, OBJECT_HEALTH_BAR_POS, OBJECT_INDEX, OBJECT_INVULNERABLE, OBJECT_MAGIC_RESIST, OBJECT_MANA, OBJECT_MAX_HEALTH, OBJECT_MAX_MANA, OBJECT_MOVEMENT_SPEED, OBJECT_POSITION, OBJECT_TARGETABLE, OBJECT_TEAM, OBJECT_VISIBILITY};

pub fn get_ai_mgr(base: i32, mut memory: Memory) -> u32 {
    let v1= memory.read_i::<u8>(base + AI_MANAGER) as i32;
    let v2= base + AI_MANAGER - 8;
    let v3= memory.read_i::<i32>(v2 + 4);
    let mut v4= memory.read_t::<i32>((v2 + (4 * v1 + 12)) as DWORD);
    v4 = v4.bitxor(v3.not());
    return memory.read_i::<i32>((v4 + 8) as i32) as u32;
}



#[derive(Debug)]
pub struct AiManager {
    ai_address: u32,
    pub start_path: Vec3,
    pub end_path: Vec3,
    pub is_moving: bool
}

impl AiManager {
    pub fn new(ai_address: u32) -> AiManager {
        return AiManager {
            ai_address,
            start_path: Vec3::new(0.0, 0.0, 0.0),
            end_path: Vec3::new(0.0, 0.0, 0.0),
            is_moving: false
        }
    }
    pub fn update(&mut self, mut memory: Memory) {
        self.start_path = memory.read::<Vec3>(self.ai_address + AI_MANAGER_START_PATH);
        self.end_path = memory.read::<Vec3>(self.ai_address + AI_MANAGER_END_PATH);
        self.is_moving = memory.read::<bool>(self.ai_address + AI_MANAGER_IS_MOVING);

    }

}


#[derive(Debug)]
pub struct GameObject {
    pub base_address: u32,
    pub index: i32,
    pub team: u32,
    pub direction: Vec3,
    pub position: Vec3,
    pub dead: u32,
    pub visibility: bool,
    pub mana: f32,
    pub max_mana: f32,
    pub invulnearable: bool,
    pub targetable: bool,
    pub health: f32,
    pub max_health: f32,
    pub bonus_attack_damage: f32,
    pub attack_damage: f32,
    pub armor: f32,
    pub bonus_armor: f32,
    pub magic_resist: f32,
    pub movement_speed: f32,
    pub attack_range: f32,
    pub champion_name: String,
    pub health_bar_pos: Vec2,
    pub ai_manager: AiManager
}
impl GameObject {
    pub fn new(base_address: u32) -> GameObject {
        return GameObject {
            base_address,
            index: 0,
            team: 0,
            direction: Vec3::new(0.0, 0.0, 0.0),
            position: Vec3::new(0.0, 0.0, 0.0),
            health_bar_pos: Vec2::new(0.0, 0.0),
            dead: 0,
            visibility: false,
            mana: 0.0,
            max_mana: 0.0,
            invulnearable: false,
            targetable: false,
            health: 0.0,
            max_health: 0.0,
            bonus_attack_damage: 0.0,
            attack_damage: 0.0,
            armor: 0.0,
            bonus_armor: 0.0,
            magic_resist: 0.0,
            movement_speed: 0.0,
            attack_range: 0.0,
            champion_name: String::new(),
            ai_manager: AiManager::new(0)
        };
    }

    pub fn update(&mut self, mut memory: Memory) {
        self.index = memory.read::<i32>(self.base_address + OBJECT_INDEX);
        self.team = memory.read::<u32>(self.base_address + OBJECT_TEAM);
        self.direction = memory.read::<Vec3>(self.base_address + OBJECT_DIRECTION); //0x1364
        self.position = memory.read::<Vec3>(self.base_address + OBJECT_POSITION);
        self.dead = memory.read::<u32>(self.base_address + OBJECT_DEAD);
        self.visibility = memory.read::<bool>(self.base_address + OBJECT_VISIBILITY);
        self.mana = memory.read::<f32>(self.base_address + OBJECT_MANA);
        self.max_mana = memory.read::<f32>(self.base_address + OBJECT_MAX_MANA);
        self.invulnearable = memory.read::<bool>(self.base_address + OBJECT_INVULNERABLE);
        self.targetable = memory.read::<bool>(self.base_address + OBJECT_TARGETABLE);
        self.health = memory.read::<f32>(self.base_address + OBJECT_HEALTH);
        self.max_health = memory.read::<f32>(self.base_address + OBJECT_MAX_HEALTH);
        self.bonus_attack_damage = memory.read::<f32>(self.base_address + OBJECT_BONUS_ATTACK_DAMAGE);
        self.attack_damage = memory.read::<f32>(self.base_address + OBJECT_ATTACK_DAMAGE);
        self.armor = memory.read::<f32>(self.base_address + OBJECT_ARMOR);
        self.bonus_armor = memory.read::<f32>(self.base_address + OBJECT_BONUS_ATTACK_DAMAGE);
        self.magic_resist = memory.read::<f32>(self.base_address + OBJECT_MAGIC_RESIST);
        self.movement_speed = memory.read::<f32>(self.base_address + OBJECT_MOVEMENT_SPEED);
        self.attack_range = memory.read::<f32>(self.base_address + OBJECT_ATTACK_RANGE);
        if self.champion_name.len() == 0 {
            self.champion_name.clear();
            self.champion_name.push_str(memory.read_string(self.base_address + OBJECT_CHAMPION_NAME).as_str());
        }

        if self.ai_manager.ai_address == 0 {
            let manager = get_ai_mgr(self.base_address as i32, memory);
            if manager != 0 {
                self.ai_manager = AiManager::new(manager)
            }
        } else {
            self.ai_manager.update(memory);
        }

    }

}

#[derive(Debug)]
pub struct MinionObject {
    pub base_address: u32,
    pub index: i32,
    pub team: u32,
    pub position: Vec3,
    pub visibility: bool,
    pub targetable: bool,
    pub health: f32,
    pub max_health: f32,
    pub movement_speed: f32,
}
impl MinionObject {
    pub fn new(base_address: u32) -> MinionObject {
        return MinionObject {
            base_address,
            index: 0,
            team: 0,
            position: Vec3::new(0.0, 0.0, 0.0),
            visibility: false,
            targetable: false,
            health: 0.0,
            max_health: 0.0,
            movement_speed: 0.0,
        };
    }

    pub fn update(&mut self, mut memory: Memory) {
        self.index = memory.read::<i32>(self.base_address + OBJECT_INDEX);
        self.team = memory.read::<u32>(self.base_address + OBJECT_TEAM);
        self.position = memory.read::<Vec3>(self.base_address + OBJECT_POSITION);
        self.visibility = memory.read::<bool>(self.base_address + OBJECT_VISIBILITY);
        self.targetable = memory.read::<bool>(self.base_address + OBJECT_TARGETABLE);
        self.health = memory.read::<f32>(self.base_address + OBJECT_HEALTH);
        self.max_health = memory.read::<f32>(self.base_address + OBJECT_MAX_HEALTH);
        self.movement_speed = memory.read::<f32>(self.base_address + OBJECT_MOVEMENT_SPEED);

    }
}

#[derive(Debug)]
pub struct TurretObject {
    pub base_address: u32,
    pub index: i32,
    pub team: u32,
    pub position: Vec3,
    pub invulnearable: bool,
    pub targetable: bool,
    pub health: f32,
    pub max_health: f32,
    pub attack_range: f32,
}
impl TurretObject {
    pub fn new(base_address: u32) -> TurretObject {
        return TurretObject {
            base_address,
            index: 0,
            team: 0,
            position: Vec3::new(0.0, 0.0, 0.0),
            invulnearable: false,
            targetable: false,
            health: 0.0,
            max_health: 0.0,
            attack_range: 0.0,
        };
    }

    pub fn update(&mut self, mut memory: Memory) {
        self.index = memory.read::<i32>(self.base_address + OBJECT_INDEX);
        self.team = memory.read::<u32>(self.base_address + OBJECT_TEAM);
        self.position = memory.read::<Vec3>(self.base_address + OBJECT_POSITION);
        self.invulnearable = memory.read::<bool>(self.base_address + OBJECT_INVULNERABLE);
        self.targetable = memory.read::<bool>(self.base_address + OBJECT_TARGETABLE);
        self.health = memory.read::<f32>(self.base_address + OBJECT_HEALTH);
        self.max_health = memory.read::<f32>(self.base_address + OBJECT_MAX_HEALTH);
        self.attack_range = memory.read::<f32>(self.base_address + OBJECT_ATTACK_RANGE);

    }
}

