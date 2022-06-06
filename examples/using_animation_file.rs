use bevy::prelude::*;

use benimator::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AnimationPlugin::default()) // <-- Add the plugin
        .add_startup_system(spawn)
        .run();
}

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
) {
    // Don't forget the camera ;-)
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Create an animation
    // Here we use an index-range (from 0 to 4) where each frame has the same duration
    let animation_handle: Handle<SpriteSheetAnimation> = asset_server.load("coin.animation.yml");

    commands
        // Spawn a bevy sprite-sheet
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.add(TextureAtlas::from_grid(
                asset_server.load("coin.png"),
                Vec2::new(16.0, 16.0),
                5,
                1,
            )),
            transform: Transform::from_scale(Vec3::splat(10.0)),
            ..Default::default()
        })
        // Insert the asset handle of the animation
        .insert(animation_handle)
        // Start the animation immediately. Remove this component in order to pause the animation.
        .insert(Play);
}