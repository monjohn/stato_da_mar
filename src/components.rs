use bevy::prelude::*;

#[derive(Component)]
pub struct CannonBall;

#[derive(Component)]
pub struct FromPlayer;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerReadyFire(pub bool);

#[derive(Component)]
// pub string SpriteData(pub HashMap<&'static str, usize>)
pub struct SpriteData {
    pub player: usize,
    pub player_light_damage: usize,
    pub cannonball: usize,
}

#[derive(Component)]
pub struct MyAtlases {
    pub ships_atlas: Handle<TextureAtlas>,
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
