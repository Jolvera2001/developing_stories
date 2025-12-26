use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

mod assets_definition;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .run();
}
