mod connection;
mod global;
mod settings;
mod ui;
mod util;

use crate::connection::ConnectionPlugin;
use crate::global::Global;
use crate::ui::UiPlugin;
use bevy::app::App;
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;
use common::channels::Channels;
use common::config::shared_config;
use common::protocol::Protocol;
use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin};

fn main() {
    App::new()
        .init_resource::<Global>()
        .insert_resource(settings::window_descriptor())
        .add_plugins(DefaultPlugins)
        .add_plugin(ClientPlugin::<Protocol, Channels>::new(
            ClientConfig::default(),
            shared_config(),
        ))
        .add_plugin(EguiPlugin)
        .add_plugin(ConnectionPlugin)
        .add_plugin(UiPlugin)
        .run();
}
