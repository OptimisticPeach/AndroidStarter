use piston::input::Input;
use piston::input::event_id::EventId;
use std::sync::Arc;
use std::any::Any;

pub enum InputEvent {
    Piston(Input),
    Custom(EventId, Arc<dyn Any + Send + Sync>)
}
