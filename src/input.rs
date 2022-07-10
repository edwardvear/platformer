use bevy::prelude::*;
use bevy::app::AppExit;

use benimator::{
    Play,
};

#[derive(Component, Default)]
pub struct PlayerInput {
    pub move_right: bool,
    pub move_left: bool,
    pub jump: bool,
    pub crouch: bool,
}

impl PlayerInput {
    fn clear(&mut self) {
        *self = PlayerInput::default();
    }
}

pub fn input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut PlayerInput>,
    mut exit: EventWriter<AppExit>,
) {
    for mut player_input in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Escape) {
            exit.send(AppExit);
        }

        player_input.clear();

        if keyboard_input.pressed(KeyCode::W) {
            player_input.jump = true;
        }
        if keyboard_input.pressed(KeyCode::A) {
            player_input.move_left = true;
        }
        if keyboard_input.pressed(KeyCode::S) {
            player_input.crouch = true;
        }
        if keyboard_input.pressed(KeyCode::D) {
            player_input.move_right = true;
        } 

    }
}

/*
) {
    for (entity, mut player, mut velocity) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Escape) {
            exit.send(AppExit);
        }

        velocity.x = 0.0;

        match player.state {
            PlayerState::Jumping => {
                if keyboard_input.just_released(KeyCode::W) {
                    velocity.y = 0.0;
                }
            },
            _ => {},
        }

        match player.state {
            PlayerState::Jumping|PlayerState::Falling => {
                if keyboard_input.pressed(KeyCode::A) {
                    velocity.x += -1.0;
                }

                if keyboard_input.pressed(KeyCode::D) {
                    velocity.x += 1.0;
                } 
            },
            PlayerState::DrawingSword |
            PlayerState::SheathingSword |
            PlayerState::Slashing => {
                if !play_query.contains(entity) {
                    player.state = PlayerState::Idle;
                }
            },
            _ => {
                if keyboard_input.pressed(KeyCode::A) {
                    velocity.x += -1.0;
                }

                if keyboard_input.pressed(KeyCode::D) {
                    velocity.x += 1.0;
                } 

                if velocity.x.abs() > 0.0 {
                    player.state = PlayerState::Running;
                } else if keyboard_input.pressed(KeyCode::S) {
                    player.state = PlayerState::Crouched;
                } else {
                    player.state = PlayerState::Idle;
                }

                if keyboard_input.pressed(KeyCode::W) {
                    player.state = PlayerState::Jumping;
                    velocity.y = 5.0;
                }

                if keyboard_input.just_pressed(KeyCode::Space) && player.sword_drawn {
                    player.state = PlayerState::Slashing;

                    velocity.x = 0.0;
                    velocity.y = 0.0;
                }

                if keyboard_input.just_pressed(KeyCode::E) && !player.sword_drawn {
                    player.sword_drawn = true;
                    player.state = PlayerState::DrawingSword;

                    velocity.x = 0.0;
                    velocity.y = 0.0;
                } else if keyboard_input.just_pressed(KeyCode::E) {
                    player.sword_drawn = false;
                    player.state = PlayerState::SheathingSword;

                    velocity.x = 0.0;
                    velocity.y = 0.0;
                }
            }
        }

    }
}
*/
