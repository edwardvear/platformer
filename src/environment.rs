use bevy::prelude::*;

use bevy_rapier2d::{
    geometry::{
        Collider,
    },
};

#[derive(Component, Default)]
pub struct Tilemap {
    atlas: Handle<TextureAtlas>,
}

pub fn load_textures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let _: Handle<Image> = asset_server.load("../assets/sky.png");
    let _: Handle<Image> = asset_server.load("../assets/clouds.png");
    let _: Handle<Image> = asset_server.load("../assets/tileset.png");
}

pub fn check_textures(
    mut state: ResMut<State<crate::AppState>>,
    asset_server: Res<AssetServer>,
    textures: Query<&Handle<Image>>,
) {
    let mut loaded = true;
    for texture in textures.iter() {
        if let bevy::asset::LoadState::Loaded =
            asset_server.get_load_state(texture)
        {
        }
        else { loaded = false }
    }

    if loaded { state.set(crate::AppState::Finished).unwrap() }
}

pub fn environment_setup(
    mut commands: Commands,
    mut tilemap: ResMut<Tilemap>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.get_handle("../assets/sky.png"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        });

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.get_handle("../assets/clouds.png"),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        });

    let mut atlas = TextureAtlas::new_empty(
        asset_server.get_handle("../assets/tileset.png"),
        Vec2::new(320., 928.),
    );

    let platform_idx = atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(20.0, 160.0),
        max: Vec2::new(140.0, 300.0),
    });

    tilemap.atlas = atlases.add(atlas);

    for i in 0..5 {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: tilemap.atlas.clone(),
                sprite: TextureAtlasSprite::new(platform_idx),
                transform: Transform::from_xyz(i as f32 * 100.0 , -65.0, 4.0),
                ..Default::default()
            })
            .insert(Collider::cuboid(50.0, 50.0));
    }
}

pub fn environment_scaling_system(
    asset_server: Res<AssetServer>,
    images: Res<Assets<Image>>,
    windows: Res<Windows>,
    mut query: Query<(&mut Transform, &Handle<Image>), With<Sprite>>,
) {
    let window = windows.get_primary().unwrap();
    let window_width = window.width();
    let window_height = window.height();

    let sky_handle: Handle<Image> = asset_server.get_handle("../assets/sky.png");
    let cloud_handle: Handle<Image> = asset_server.get_handle("../assets/clouds.png");

    for (mut tf, image_handle) in query.iter_mut() {
        if image_handle == &sky_handle
        {
            if let Some(texture) = images.get(image_handle) {
                let texture_size = texture.size();
                tf.scale.x = window_width / texture_size[0];
                tf.scale.y = window_height / texture_size[1];
            }
        }

        if image_handle == &cloud_handle {
            if let Some(texture) = images.get(image_handle) {
                let texture_size = texture.size();
                tf.scale.x = window_width / texture_size[0];
            }
        }
    }
}
