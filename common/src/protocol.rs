use naia_shared::Protocolize;

mod message;

pub use message::Message;

#[derive(Protocolize)]
pub enum Protocol {
    Message(Message),
}
