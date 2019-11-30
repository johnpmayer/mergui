///Contains various structs and traits to comunicate with the widgets
pub mod channels;
mod context;
///Contains structs and traits of the most simple of widgets.
pub mod core;
mod responses;
///Contains the most used widgets and traits.
///Often they are composed of multiple core or normal widgets
pub mod widgets;

pub use crate::context::{Assets, Context};
pub(crate) use responses::{
    LayerChannelReceiver, LayerChannelSender, LayerInstructions, LayerNummerId,
    WidgetChannelReceiver, WidgetChannelSender, WidgetInstruction, WidgetNummerId,
};
pub use responses::{LayerId, Response, WidgetId};