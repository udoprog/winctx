use self::sender::InputEvent;
pub use self::sender::Sender;
mod sender;

pub use self::event_loop::EventLoop;
mod event_loop;

pub use self::event::{ClipboardEvent, Event};
mod event;
