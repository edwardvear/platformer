use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod input;
mod physics;
mod player;
mod environment;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Setup,
    Finished,
}

fn main() {
    App::new()
        .init_resource::<environment::Tilemap>()
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: 500.,
            height: 300.,
            mode: bevy::window::WindowMode::Fullscreen,
            ..default()
        })
        .insert_resource(Msaa { samples: 1 })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(18.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(benimator::AnimationPlugin::default())
        .add_startup_system(setup)
        .add_startup_system(player::player_setup)
        .add_state(AppState::Setup)
        .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(environment::load_textures))
        .add_system_set(SystemSet::on_update(AppState::Setup).with_system(environment::check_textures))
        .add_system_set(SystemSet::on_enter(AppState::Finished).with_system(environment::environment_setup))
        .add_system(environment::environment_scaling_system)
        .add_system(
            input::input_system
                .before(player::movement_system)
        )
        .add_system(
            player::movement_system
                .before(player::player_animation_system)
        )
        .add_system(player::sprite_flipping_system)
        .add_system(
            player::player_animation_system
                .before(player::animation_reset_system)
        )
        //.add_system(player::animation_reset_system)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
