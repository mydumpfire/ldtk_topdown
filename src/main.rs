use bevy::{prelude::*, window};
use bevy_ecs_ldtk::prelude::*;

#[derive(Clone, Copy, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelSelection::Index(0))
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: false,
                },
                set_clear_color: SetClearColor::FromLevelBackground,
                ..default()
            })
            .register_ldtk_int_cell::<WallBundle>(1);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("topdown.ldtk"),
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy".to_string(),
                fit_canvas_to_parent: true,
                resolution: (800.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(LdtkPlugin)
        .add_plugin(GamePlugin)
        .add_startup_system(setup)
        .add_system(window::close_on_esc)
        .run();
}
