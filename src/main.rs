use bevy::prelude::*;

use quartz::QuartzPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    //transparent: true,
                    title: String::from("awawawa"),
                    ..default()
                }),
                ..default()
            }), QuartzPlugin));
    app.run();
}
