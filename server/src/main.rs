mod global;
mod systems;
mod user;

use bevy_app::{App, ScheduleRunnerPlugin};
use bevy_core::CorePlugin;
use bevy_log::{info, LogPlugin};
use common::channels::Channels;
use common::protocol::Protocol;

use common::config::shared_config;
use naia_bevy_server::{Plugin as ServerPlugin, ServerConfig, Stage};

use crate::systems::events;
use crate::user::UserState;
use systems::init;

fn main() {
    info!("Goodlines starting up");
    // Build App
    App::default()
        // Resources
        .init_resource::<UserState>()
        // Plugins
        .add_plugin(CorePlugin::default())
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_plugin(LogPlugin::default())
        .add_plugin(ServerPlugin::<Protocol, Channels>::new(
            ServerConfig::default(),
            shared_config(),
        ))
        // Startup System
        .add_startup_system(init)
        // Systems
        .add_system_to_stage(Stage::ReceiveEvents, events::authorization_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::connection_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::disconnection_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::receive_message_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::broadcast_message_event)
        .add_system_to_stage(Stage::Tick, events::tick)
        .run();
}
