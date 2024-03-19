use bevy::prelude::*;
use bevy2d_template::game;
fn main() {
    App::new().add_plugins((DefaultPlugins, game)).run();
}
