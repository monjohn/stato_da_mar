use crate::prelude::*;
use bevy::{app::AppExit, prelude::*};

pub struct MainMenuPlugin;

#[derive(Resource)]
struct MainMenuData {
    camera_entity: Entity,
    ui_root: Entity,
}
#[derive(Resource)]
struct MenuMaterials {
    button: Handle<ColorMaterial>,
    button_hovered: Handle<ColorMaterial>,
    button_pressed: Handle<ColorMaterial>,
}

impl FromWorld for MenuMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        MenuMaterials {
            button: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            button_hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            button_pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}

#[derive(Component)]
enum MenuButton {
    Play,
    Quit,
}

fn button_system(
    materials: Res<MenuMaterials>,
    mut buttons: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut material) in buttons.iter_mut() {
        match *interaction {
            Interaction::Clicked => *material = materials.button_pressed.clone(),
            Interaction::Hovered => *material = materials.button_hovered.clone(),
            Interaction::None => *material = materials.button.clone(),
        }
    }
}

fn button_press_system(
    buttons: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<State<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, button) in buttons.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                MenuButton::Play => state
                    .set(AppState::Playing)
                    .expect("Couldn't switch state to Playing"),
                MenuButton::Quit => exit.send(AppExit),
            };
        }
    }
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuMaterials>()
            .add_system(button_system)
            .add_system(button_press_system)
            .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(cleanup));
    }
}

fn root() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },

        background_color: BackgroundColor(Color::NONE),
        ..Default::default()
    }
}

fn border() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(400.0), Val::Auto),
            border: UiRect::all(Val::Px(8.0)),
            ..Default::default()
        },
        background_color: BackgroundColor(Color::rgb(0.65, 0.65, 0.65)),
        ..Default::default()
    }
}

fn menu_background() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::ColumnReverse,
            padding: UiRect::all(Val::Px(5.0)),
            ..Default::default()
        },
        background_color: BackgroundColor(Color::rgb(0.15, 0.15, 0.15)),
        ..Default::default()
    }
}

fn button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        background_color: BackgroundColor(Color::rgb(0.15, 0.15, 0.15)),
        ..Default::default()
    }
}

fn button_text(asset_server: &Res<AssetServer>, label: &str) -> TextBundle {
    return TextBundle {
        style: Style {
            margin: UiRect::all(Val::Px(10.0)),
            ..Default::default()
        },
        text: Text::from_section(
            label,
            TextStyle {
                font: asset_server.load("fonts/Auxerre Bold.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
        ),
        ..Default::default()
    };
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera_entity = commands.spawn(Camera2dBundle::default()).id();

    let ui_root = commands
        .spawn(root())
        .with_children(|parent| {
            // left vertical fill (border)
            parent.spawn(border()).with_children(|parent| {
                // left vertical fill (content)
                parent.spawn(menu_background()).with_children(|parent| {
                    parent
                        .spawn(button())
                        .with_children(|parent| {
                            parent.spawn(button_text(&asset_server, "New Game"));
                        })
                        .insert(MenuButton::Play);
                    parent
                        .spawn(button())
                        .with_children(|parent| {
                            parent.spawn(button_text(&asset_server, "Quit"));
                        })
                        .insert(MenuButton::Quit);
                });
            });
        })
        .id();

    commands.insert_resource(MainMenuData {
        camera_entity,
        ui_root,
    });
}

fn cleanup(mut commands: Commands, menu_data: Res<MainMenuData>) {
    commands.entity(menu_data.ui_root).despawn_recursive();
    commands.entity(menu_data.camera_entity).despawn_recursive();
}
