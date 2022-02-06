use bevy::core::FixedTimestep;
use bevy::math::Quat;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_stage(
            "game_setup_actors",
            SystemStage::single(player_spawn.system()),
        )
        .add_system(player_movement.system())
        // .add_system(player_fire.system())
        // .add_system(laser_movement.system())
        .add_system_set(SystemSet::new().with_run_criteria(FixedTimestep::step(0.5)));
    }
}

#[derive(Component)]
struct Player;

fn player_spawn(mut commands: Commands, asset_server: Res<AssetServer>, time: Res<Time>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("ship2x.png"),
            ..Default::default()
        })
        .insert(Player);
    //.insert(PlayerReadyFire)
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_positions: Query<&mut Transform, With<Player>>,
) {
    for mut transform in player_positions.iter_mut() {
        // println!("Original: {:?}", transform.rotation);
        if keyboard_input.pressed(KeyCode::Left) {
            transform.rotate(Quat::from_rotation_z(0.01));
            transform.rotation = transform.rotation.normalize();
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.rotate(Quat::from_rotation_z(-0.01));
            transform.rotation = transform.rotation.normalize();
        }
        if keyboard_input.pressed(KeyCode::Down) {
            // transform.translation.y -= 2.;
            transform.rotation = Quat::IDENTITY;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            let (x, y) = rotation_to_vector(transform.rotation.clone());
            transform.translation.x += x;
            transform.translation.y += y;
        }
    }
}

fn rotation_to_vector(rotation: Quat) -> (f32, f32) {
    let (_r, angle) = rotation.to_axis_angle();

    let y = angle.cos();
    let x = angle.sin();

    // Need to convert to Euler to determine whether angle is positive or negative
    let (_x, _y, z) = rotation.to_euler(bevy::math::EulerRot::XYZ);
    // Then use that information to determine whether moving left or right
    let x = if z.is_sign_positive() {
        x.abs()
    } else {
        -(x.abs())
    };
    (x, -y)
}
