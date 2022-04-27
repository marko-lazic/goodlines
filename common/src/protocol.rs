use naia_shared::Protocolize;

mod auth;
mod message;

pub use auth::Auth;
pub use message::Message;

#[derive(Protocolize)]
pub enum Protocol {
    Auth(Auth),
    Message(Message),
}
