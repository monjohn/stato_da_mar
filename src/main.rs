mod components;
mod player;

mod prelude {
    pub use crate::components::*;
    pub const TIME_STEP: f32 = 1. / 60.;
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
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::AQUAMARINE)) // Set background color
        .add_startup_system(setup)
        .add_plugin(PlayerPlugin)
        .add_system(cannonball_movement)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn cannonball_movement(
    mut query: Query<(Entity, &Velocity, &mut Transform), (With<CannonBall>, With<FromPlayer>)>,
) {
    for (_cannonball_entity, velocity, mut cannonball_tf) in query.iter_mut() {
        let translation = &mut cannonball_tf.translation;
        translation.x += velocity.x; //* TIME_STEP;
        translation.y += velocity.y; // * TIME_STEP;
                                     // if translation.y > win_size.h {
                                     //     commands.entity(cannonball_entity).despawn();
                                     // }
    }
}
