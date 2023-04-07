use crate::prelude::*;
use bevy::time::FixedTimestep;
use bevy::prelude::*;

pub struct PiratePlugin;

impl Plugin for PiratePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_stage("game_setup_pirates", SystemStage::single(pirate_spawn))
            .add_system(pirate_movement)
            .add_system(pirate_fire)
            .add_system(health)
            .add_system_set(SystemSet::new().with_run_criteria(FixedTimestep::step(0.5)));
    }
}

fn pirate_spawn(mut commands: Commands, sprite_data: Res<SpriteData>, my_atlases: Res<MyAtlases>) {
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: my_atlases.ships_atlas.clone(),
            transform: Transform {
                scale: Vec3::splat(SPRITE_SCALE),
                translation: Vec3::new(100., 100., 0.0),
                ..Default::default()
            },
            sprite: TextureAtlasSprite {
                index: sprite_data.get_pirate_sprite(PIRATE_STARTING_HEALTH),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Pirate)
        .insert(Range(DEFAULT_RANGE))
        .insert(Health::new(PIRATE_STARTING_HEALTH));
}

fn pirate_movement(mut _pirates_positions: Query<&mut Transform, With<Pirate>>) {
    ()
}

fn pirate_fire(
    mut commands: Commands,
    my_atlases: Res<MyAtlases>,
    sprite_data: Res<SpriteData>,
    mut pirates: Query<(&Transform, &Range), With<Pirate>>,
    query: Query<&Transform, With<Player>>,
) {
    if let Ok(player) = query.get_single() {
        for (transform, range) in pirates.iter_mut() {
            let player_in_range = player
                .translation
                .distance_squared(transform.translation.clone())
                < range.0.pow(2) as f32;

            if player_in_range {
                let pos_x = transform.translation.x;
                let pos_y = transform.translation.y;

                let mut spawn_cannonballs = |x: f32, y: f32| {
                    commands
                        .spawn(SpriteSheetBundle {
                            texture_atlas: my_atlases.ships_atlas.clone(),
                            transform: Transform {
                                translation: Vec3::new(pos_x, pos_y, 0.),
                                scale: Vec3::splat(SPRITE_SCALE),
                                ..Default::default()
                            },
                            sprite: TextureAtlasSprite {
                                index: sprite_data.cannonball,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(CannonBall)
                        .insert(FromPirate)
                        .insert(Velocity::new(x * 5., y * 5.));
                };

                spawn_cannonballs(pos_x, pos_y);
            }
        }
    }
}

fn health(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TextureAtlasSprite, &mut Health), With<Pirate>>,
    sprite_data: Res<SpriteData>,
) {
    for (entity, mut sprite, mut health) in query.iter_mut() {
        if health.damage > 0 && health.current > 0 {
            health.incur_damage();
            sprite.index = sprite_data.get_pirate_sprite(health.current);
            if health.current == 0 {
                commands.entity(entity).insert(Destroyed);
            }
        }
    }
}
