use core::ops::Deref;
use std::time::Duration;

use bevy::prelude::*;
use bevy::sprite::collide_aabb::Collision;

use benimator::{
    Play,
    SpriteSheetAnimation,
    SpriteSheetAnimationState,
};

use bevy_rapier2d::{
    dynamics::{
        LockedAxes,
        RigidBody,
        Velocity,
    },
    geometry::{
        Collider,
    },
};

use crate::input::PlayerInput;

#[derive(Component)]
pub struct Player {
    pub state: PlayerState,
    pub max_speed: f32,
    pub sword_drawn: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            state: PlayerState::Idle,
            max_speed: 100.0,
            sword_drawn: false,
        }
    }
}

#[derive(Debug)]
pub enum PlayerState {
    Idle,
    Crouched,
    Running,
    Jumping,
    Falling,
    DrawingSword,
    SheathingSword,
    Slashing
}

#[derive(Component)]
pub struct PlayerAnimations {
    idle: Handle<SpriteSheetAnimation>,
    idle_sword_drawn: Handle<SpriteSheetAnimation>,
    crouched: Handle<SpriteSheetAnimation>,
    running: Handle<SpriteSheetAnimation>,
    jumping: Handle<SpriteSheetAnimation>,
    falling: Handle<SpriteSheetAnimation>,
    drawing_sword: Handle<SpriteSheetAnimation>,
    sheathing_sword: Handle<SpriteSheetAnimation>,
    slashing: Handle<SpriteSheetAnimation>,
}

impl PlayerAnimations {
    fn new(
        mut assets: ResMut<Assets<SpriteSheetAnimation>>,
    ) -> Self {
        let idle = assets.add(SpriteSheetAnimation::from_range(
                0..=3,
                Duration::from_secs_f64(1.0 / 7.0),
        ));
        let idle_sword_drawn = assets.add(SpriteSheetAnimation::from_range(
                38..=41,
                Duration::from_secs_f64(1.0 / 7.0),
        ));
        let crouched = assets.add(SpriteSheetAnimation::from_range(
                4..=7,
                Duration::from_secs_f64(1.0 / 7.0),
        ));
        let running = assets.add(SpriteSheetAnimation::from_range(
                8..=13,
                Duration::from_secs_f64(1.0 / 7.0),
        ));
        let jumping = assets.add(SpriteSheetAnimation::from_range(
                15..=17,
                Duration::from_secs_f64(1.0 / 7.0),
        ).once());
        let falling = assets.add(SpriteSheetAnimation::from_range(
                22..=23,
                Duration::from_secs_f64(1.0 / 7.0),
        ));
        let drawing_sword = assets.add(SpriteSheetAnimation::from_range(
                69..=72,
                Duration::from_secs_f64(1.0 / 7.0),
        ).once());
        let sheathing_sword = assets.add(SpriteSheetAnimation::from_range(
                73..=76,
                Duration::from_secs_f64(1.0 / 7.0),
        ).once());
        let slashing = assets.add(SpriteSheetAnimation::from_range(
                47..=52,
                Duration::from_secs_f64(1.0 / 7.0),
        ).once());

        Self {
            idle,
            idle_sword_drawn,
            crouched,
            running,
            jumping,
            falling,
            drawing_sword,
            sheathing_sword,
            slashing,
        }
    }
}

pub fn player_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    animations: ResMut<Assets<SpriteSheetAnimation>>,
) {
    let texture_handle = asset_server.load("adventurer.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(50.0, 37.0), 7, 11);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let player_animations = PlayerAnimations::new(animations);
    let player_animation_handle = player_animations.idle.clone();

    let mut player_tf = Transform::from_scale(Vec3::splat(1.5));
    player_tf.translation.y = 100.0;
    player_tf.translation.z = 10.0;

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: player_tf,
            ..default()
        })
        .insert(Player::new())
        .insert(PlayerInput::default())
        .insert(RigidBody::KinematicVelocityBased)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Collider::cuboid(10.0, 16.0))
        .insert(Velocity::default())
        .insert(player_animation_handle)
        .insert(Play);

    commands.insert_resource(player_animations);
}

pub fn movement_system(
    time: Res<Time>,
    mut query: Query<(&PlayerInput, &mut Player, &mut Velocity, &mut Transform)>,
) {
    for (input, mut player, mut velocity, mut tf) in query.iter_mut() {
        let dt = time.delta_seconds();

        if input.move_right {
            velocity.linvel.x = player.max_speed;
        }
        if input.move_left {
            velocity.linvel.x = -player.max_speed;
        }

        // If no inputs, stop.
        if !(input.move_right | input.move_left) {
            player.state = PlayerState::Idle;
            velocity.linvel.x = 0.0;
        }

        if (input.move_right | input.move_left) && velocity.linvel.x.abs() > 0.0 {
            player.state = PlayerState::Running;
        }

        velocity.linvel.x = velocity.linvel.x.clamp(-player.max_speed, player.max_speed);

        /*
        let dx = velocity.x * dt;
        let dy = velocity.y * dt;

        tf.translation += Vec3::new(dx, dy, 0.0);
        */
    }
}

pub fn player_animation_system(
    mut query: Query<(
        &Player,
        &mut Handle<SpriteSheetAnimation>,
    )>,
    animations: Res<PlayerAnimations>,
) {
    for (player, mut animation_handle) in query.iter_mut() {
        let desired_animation_handle = match player.state {
            PlayerState::Idle => {
                if player.sword_drawn {
                    animations.idle_sword_drawn.clone() 
                } else {
                    animations.idle.clone()
                }
            },
            PlayerState::Crouched => animations.crouched.clone(),
            PlayerState::Running => animations.running.clone(),
            PlayerState::Jumping => animations.jumping.clone(),
            PlayerState::Falling => animations.falling.clone(),
            PlayerState::DrawingSword => animations.drawing_sword.clone(),
            PlayerState::SheathingSword => animations.sheathing_sword.clone(),
            PlayerState::Slashing => animations.slashing.clone(),
        };

        if desired_animation_handle != *animation_handle {
            *animation_handle = desired_animation_handle;
        }
    }
}

pub fn animation_reset_system(
    mut query: Query<
        (Entity, &mut SpriteSheetAnimationState),
        Changed<Handle<SpriteSheetAnimation>>,
    >,
    play_query: Query<&Play>,
    mut commands: Commands,
) {
    for (entity, mut animation_state) in query.iter_mut() {
        animation_state.reset();
        if !play_query.contains(entity) {
            commands.entity(entity)
                .insert(Play);
        }
    }
}

pub fn sprite_flipping_system(
    mut query: Query<(&PlayerInput, &mut TextureAtlasSprite)>,
) {
    for (input, mut sprite) in query.iter_mut() {
        if input.move_right {
            sprite.flip_x = false;
        } else if input.move_left {
            sprite.flip_x = true;
        }
    }
}
