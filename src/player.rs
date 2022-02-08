use crate::prelude::*;
use bevy::core::FixedTimestep;
use bevy::math::Quat;
use bevy::prelude::*;
use std::f32::consts::PI;

pub struct PlayerPlugin;
const SCALE: f32 = 0.6;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_stage("game_setup_actors", SystemStage::single(player_spawn))
            .add_system(player_movement)
            .add_system(player_fire)
            .add_system_set(SystemSet::new().with_run_criteria(FixedTimestep::step(0.5)));
    }
}

fn player_spawn(mut commands: Commands, sprite_data: Res<SpriteData>, my_atlases: Res<MyAtlases>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: my_atlases.ships_atlas.clone(),
            transform: Transform::from_scale(Vec3::splat(SCALE)),
            sprite: TextureAtlasSprite {
                index: sprite_data.player,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(PlayerReadyFire(true));
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_positions: Query<&mut Transform, With<Player>>,
) {
    for mut transform in player_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.rotate(Quat::from_rotation_z(0.01));
            transform.rotation = transform.rotation.normalize();
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.rotate(Quat::from_rotation_z(-0.01));
            transform.rotation = transform.rotation.normalize();
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.rotation = Quat::IDENTITY;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            let (x, y) = rotation_to_vector(&transform.rotation);
            transform.translation.x += x;
            transform.translation.y += y;
        }
    }
}

fn player_fire(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    my_atlases: Res<MyAtlases>,
    sprite_data: Res<SpriteData>,
    mut query: Query<(&Transform, &mut PlayerReadyFire), With<Player>>,
) {
    if let Ok((transform, mut ready_fire)) = query.get_single_mut() {
        if ready_fire.0 && kb.pressed(KeyCode::Space) {
            let pos_x = transform.translation.x;
            let pos_y = transform.translation.y;

            let mut spawn_cannonballs = |x: f32, y: f32, x_offset: f32| {
                commands
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: my_atlases.ships_atlas.clone(),
                        transform: Transform {
                            translation: Vec3::new(pos_x + x_offset, pos_y, 0.),
                            scale: Vec3::splat(SCALE),
                            ..Default::default()
                        },
                        sprite: TextureAtlasSprite {
                            index: sprite_data.cannonball,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(CannonBall)
                    .insert(FromPlayer)
                    .insert(Velocity::new(x * 5., y * 5.));
            };

            let x_offset = 144.0 / 4.0 - 5.0;
            let rot1 = transform.rotation.mul_quat(Quat::from_rotation_z(PI / 2.));
            let (x1, y1) = rotation_to_vector(&rot1);
            let rot2 = transform.rotation.mul_quat(Quat::from_rotation_z(PI / -2.));
            let (x2, y2) = rotation_to_vector(&rot2);
            spawn_cannonballs(x1, y1, x_offset);
            spawn_cannonballs(x2, y2, -x_offset);

            ready_fire.0 = false;
        }

        if kb.just_released(KeyCode::Space) {
            ready_fire.0 = true;
        }
    }
}

fn rotation_to_vector(rotation: &Quat) -> (f32, f32) {
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
