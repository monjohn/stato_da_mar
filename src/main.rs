mod components;
mod player;
mod sprites;
mod prelude {
    pub use crate::components::*;
    pub const WIN_HEIGHT: f32 = 500.;
    pub const WIN_WIDTH: f32 = 700.;
}

use bevy::prelude::*;
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
        .add_system(cannonball_movement)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let mut ship_texture_atlas = sprites::build_ship_atlas(asset_server);
    let sprite_data = sprites::load_ship_atlas(&mut ship_texture_atlas);
    commands.insert_resource(sprite_data);

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
