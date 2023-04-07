use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
pub struct CannonBall;

#[derive(Component)]
pub struct Destroyed;

#[derive(Component)]
pub struct FromPirate;

#[derive(Component)]
pub struct FromPlayer;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum AppState {
    MainMenu,
    GameOver,
    Playing,
}

#[derive(Component, Debug)]
pub struct Health {
    pub current: i32,
    pub damage: i32,
}

impl Health {
    pub fn new(current: i32) -> Health {
        Health { current, damage: 0 }
    }
    pub fn incur_damage(&mut self) {
        self.current = self.current - self.damage;
        self.damage = 0;
    }
}

#[derive(Component)]
pub struct Pirate;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerReadyFire(pub bool);

#[derive(Component)]
pub struct Range(pub u32);

#[derive(Resource)]
pub struct SpriteData {
    pub player_ship: usize,
    pub player_light_damage: usize,
    pub player_heavy_damage: usize,
    pub player_destroyed: usize,
    pub pirate_ship: usize,
    pub pirate_light_damage: usize,
    pub pirate_heavy_damage: usize,
    pub pirate_destroyed: usize,
    pub cannonball: usize,
}
impl SpriteData {
    pub fn build_from_map(sprite_hash: HashMap<&'static str, usize>) -> SpriteData {
        let get_index = |name: &'static str| *(sprite_hash.get(name).unwrap());
        SpriteData {
            player_ship: get_index("ship-plain"),
            player_light_damage: get_index("ship-plain-light-damage"),
            player_heavy_damage: get_index("ship-plain-heavy-damage"),
            player_destroyed: get_index("ship-plain-destroyed"),
            pirate_ship: get_index("ship-pirate"),
            pirate_light_damage: get_index("ship-pirate-light-damage"),
            pirate_heavy_damage: get_index("ship-pirate-heavy-damage"),
            pirate_destroyed: get_index("ship-pirate-destroyed"),
            cannonball: get_index("cannonball"),
        }
    }
    pub fn get_player_sprite(&self, health: i32) -> usize {
        match health {
            0 => self.player_destroyed,
            1..=3 => self.player_heavy_damage,
            4..=5 => self.player_light_damage,
            _ => self.player_ship,
        }
    }
    pub fn get_pirate_sprite(&self, health: i32) -> usize {
        match health {
            0 => self.pirate_destroyed,
            1 => self.pirate_heavy_damage,
            2 => self.pirate_light_damage,
            _ => self.pirate_ship,
        }
    }
}

#[derive(Resource)]
pub struct MyAtlases {
    pub ships_atlas: Handle<TextureAtlas>,
}

#[derive(Component)]
pub struct Origin {
    pub location: Vec3,
}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
