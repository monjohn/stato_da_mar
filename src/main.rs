mod components;
mod main_menu;
mod pirates;
mod player;
mod sprites;
mod prelude {
    pub use crate::components::*;
    pub const DEFAULT_RANGE: u32 = 150;
    pub const WIN_HEIGHT: f32 = 700.;
    pub const WIN_WIDTH: f32 = 900.;
    pub const PLAYER_STARTING_HEALTH: i32 = 10;
    pub const PIRATE_STARTING_HEALTH: i32 = 3;
    pub const SPRITE_SCALE: f32 = 0.6;
}

use bevy::{prelude::*, sprite::collide_aabb::collide};
use main_menu::MainMenuPlugin;
use pirates::PiratePlugin;
use player::PlayerPlugin;
use prelude::*;

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
        .add_state(AppState::MainMenu)
        .add_startup_system(setup)
        .add_plugin(MainMenuPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(PiratePlugin)
        .add_system(cannonball_movement)
        .add_system(collide_with_player_cannonballs)
        .add_system(within_range)
        .add_system(game_over)
        .add_system_set(
            SystemSet::on_update(AppState::Playing)
                .with_system(back_to_main_menu_controls),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
commands.spawn_bundle(Camera2dBundle::default());

    let mut ship_texture_atlas = sprites::build_ship_atlas(asset_server);
    let sprite_hash = sprites::load_ship_atlas(&mut ship_texture_atlas);
    commands.insert_resource(SpriteData::build_from_map(sprite_hash));

    commands.insert_resource(MyAtlases {
        ships_atlas: texture_atlases.add(ship_texture_atlas),
    });
}

fn within_range(
    mut _commands: Commands,
    mut pirates: Query<(&Transform, &Range), With<Pirate>>,
    query: Query<&Transform, With<Player>>,
) {
    if let Ok(player) = query.get_single() {
        for (enemy_tf, range) in pirates.iter_mut() {
            // let delta = enemy_tf.translation - player.translation;
            // let angle_to_enemy = delta.y.atan2(delta.x);
            // let angle_between = player
            //     .rotation
            //     .angle_between(Quat::from_rotation_z(angle_to_enemy));

            // if angle_between > 1.0 && angle_between < 2.0 {
            //     println!("Player not broadside: {}", angle_between);
            //     continue;
            // }

            let distance = player
                .translation
                .distance_squared(enemy_tf.translation.clone());
            let in_range = distance < range.0.pow(2) as f32;
            println!("is in range: {}", in_range);
        }
    }
}

fn cannonball_movement(
    mut commands: Commands,
    mut query: Query<
        (Entity, &Velocity, &Origin, &Range, &mut Transform),
        (With<CannonBall>, With<FromPlayer>),
    >,
) {
    for (cannonball_entity, velocity, origin, range, mut cannonball_tf) in query.iter_mut() {
        let translation = &mut cannonball_tf.translation;
        translation.x += velocity.x;
        translation.y += velocity.y;

        if is_outside_window(translation.x, translation.y)
            || is_beyond_range(*translation, origin.location, range.0)
        {
            commands.entity(cannonball_entity).despawn();
        }
    }
}

fn is_outside_window(x: f32, y: f32) -> bool {
    y > WIN_HEIGHT || y < -WIN_HEIGHT || x < -WIN_WIDTH || x > WIN_WIDTH
}

fn is_beyond_range(pos1: Vec3, pos2: Vec3, range: u32) -> bool {
    pos1.distance_squared(pos2) > range.pow(2) as f32
}

fn collide_with_player_cannonballs(
    mut commands: Commands,
    mut pirates: Query<(&mut Health, &Transform), With<Pirate>>,
    cannonballs: Query<(Entity, &Transform), (With<CannonBall>, With<FromPlayer>)>,
) {
    for (mut health, enemy_tf) in pirates.iter_mut() {
        for (cannonball_entity, cannonball_tf) in cannonballs.iter() {
            let cannonball_size = 10.;
            let collision = collide(
                cannonball_tf.translation,
                Vec2::new(cannonball_size, cannonball_size), // size * scale,
                enemy_tf.translation,
                Vec2::new(66. * SPRITE_SCALE, 13. * SPRITE_SCALE), //enemy_size * enemy_scale,
            );
            if let Some(_) = collision {
                health.damage = 1;
                commands.entity(cannonball_entity).despawn();
            }
        }
    }
}

fn game_over(
    mut _state: ResMut<State<AppState>>,
    pirates: Query<&Health, (With<Pirate>, Without<Destroyed>)>,
) {
    if pirates.is_empty() {
        // TODO: do something with this
        // state.set(AppState::GameOver).unwrap();

        println!("GAME OVER!")
    }
}

// System implementation
fn back_to_main_menu_controls(
    mut keys: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
) {
    if *app_state.current() == AppState::Playing {
        if keys.just_pressed(KeyCode::Escape) {
            app_state.set(AppState::MainMenu).unwrap();
            keys.reset(KeyCode::Escape);
        }
    }
}
