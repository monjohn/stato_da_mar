mod components;
mod pirates;
mod player;
mod sprites;
mod prelude {
    pub use crate::components::*;
    pub const WIN_HEIGHT: f32 = 700.;
    pub const WIN_WIDTH: f32 = 900.;
    pub const PLAYER_STARTING_HEALTH: i32 = 10;
    pub const PIRATE_STARTING_HEALTH: i32 = 3;
    pub const SPRITE_SCALE: f32 = 0.6;
}

use bevy::{prelude::*, sprite::collide_aabb::collide};
use pirates::PiratePlugin;
use player::PlayerPlugin;
use prelude::*;

#[derive(Component)]
struct PlayerReadyFire(bool);

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Stato Da Mar".to_string(),
            width: WIN_WIDTH,
            height: WIN_HEIGHT,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::AQUAMARINE)) // Set background color
        .add_startup_system(setup)
        .add_plugin(PlayerPlugin)
        .add_plugin(PiratePlugin)
        .add_system(cannonball_movement)
        .add_system(collide_with_player_cannonballs)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let mut ship_texture_atlas = sprites::build_ship_atlas(asset_server);
    let sprite_hash = sprites::load_ship_atlas(&mut ship_texture_atlas);
    commands.insert_resource(SpriteData::build_from_map(sprite_hash));

    commands.insert_resource(MyAtlases {
        ships_atlas: texture_atlases.add(ship_texture_atlas),
    });
}

fn cannonball_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &Velocity, &mut Transform), (With<CannonBall>, With<FromPlayer>)>,
) {
    for (cannonball_entity, velocity, mut cannonball_tf) in query.iter_mut() {
        let translation = &mut cannonball_tf.translation;
        translation.x += velocity.x;
        translation.y += velocity.y;

        if translation.y > WIN_HEIGHT
            || translation.y < -WIN_HEIGHT
            || translation.x < -WIN_WIDTH
            || translation.x > WIN_WIDTH
        {
            commands.entity(cannonball_entity).despawn();
        }
    }
}

fn collide_with_player_cannonballs(
    mut commands: Commands,
    mut pirates: Query<(&mut Health, &Transform), With<Pirate>>,
    cannonballs: Query<(Entity, &Transform), (With<CannonBall>, With<FromPlayer>)>,
) {
    for (mut health, enemy_tf) in pirates.iter_mut() {
        for (cannonball_entity, cannonball_tf) in cannonballs.iter() {
            let _cannonball_size = 10;
            // let cannonball_scale = Vec2::from(cannonball_tf.scale.abs().xy());
            // let enemy_scale = Vec2::from(enemy_tf.scale.xy());
            let collision = collide(
                cannonball_tf.translation,
                Vec2::new(10., 10.), // player_laser_size * player_laser_scale,
                enemy_tf.translation,
                Vec2::new(66. * SPRITE_SCALE, 13. * SPRITE_SCALE), //enemy_size * enemy_scale,
            );
            if let Some(_) = collision {
                health.damage = 1;
                commands.entity(cannonball_entity).despawn();
                println!("Pirate health: {}", health.damage)
                // spawn the ExplosionToSpawn entity
                // commands
                // 	.spawn()
                // 	.insert(ExplosionToSpawn(player_tf.translation.clone()));
            }
        }
    }
}
