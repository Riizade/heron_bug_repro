extern crate bevy_template;
use bevy::prelude::*;
use bevy_template::core::plugin::GamePlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
