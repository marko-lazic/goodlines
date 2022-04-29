use naia_shared::Protocolize;

mod auth;
mod entity_assignment;
mod message;

pub use auth::Auth;
pub use entity_assignment::EntityAssignment;
pub use message::Message;

#[derive(Protocolize)]
pub enum Protocol {
    Auth(Auth),
    Message(Message),
    EntityAssignment(EntityAssignment),
}
