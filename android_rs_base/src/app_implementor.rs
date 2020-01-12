#![allow(unused_variables)]

use piston::input::event_id::EventId;
use std::sync::Arc;
use std::any::Any;
use piston::input::{TimeStamp, Input, AfterRenderArgs, RenderArgs, UpdateArgs};
use opengl_graphics::GlGraphics;
use graphics::Context;
use crate::{AppConfig, ShaderStorage};
use crate::storage::ShaderContext;

/// A trait describing an implementation of a basic android rust app
pub trait AppImpl: Sized {
    /// Data used to initialize
    type InitializationData;
    /// The transform-identifying tag used when rendering.
    /// Creates a new `Self` with graphics initialized.
    fn new(gl: &mut GlGraphics, data: Self::InitializationData, shaders: &mut ShaderStorage) -> Self;
    /// When focus is lost, this function is called to let app save states or do anything it needs to do to save
    #[inline]
    fn signal_pause(&mut self) {}

    /// Called just after `signal_pause_change` when focus is gained. Kind of meant to be the opposite to it, just more optional
    #[inline]
    fn refresh(&mut self) {}

    /// Called when rotated, or when split-screen is enabled (Unsure about that last point)
    #[inline]
    fn on_size_change(&mut self, new_size: &(usize, usize), old_size: &(usize, usize), shaders: &mut ShaderStorage) {}

    /// Called when asked to update. Pretty standard piston/glutin_window update
    fn update(&mut self, args: UpdateArgs, cfg: &mut AppConfig);

    /// Called when need to draw
    /// Used for drawing with custom shaders
    fn draw_shaded(&mut self, context: ShaderContext) {}

    /// Called when need to draw
    /// Initialization and stuff is taken care of behind the scenes
    fn draw_2d(&mut self, c: Context, gl: &mut GlGraphics, args: RenderArgs, cfg: &mut AppConfig);

    /// Called after drawing.
    fn after_draw(&mut self, args: AfterRenderArgs) {}

    /// Called when the app is closed.
    fn on_die(self) {}

    /// Called at the start of `AppContainer::run` if `config` requires a reset on start
    #[inline]
    fn reset_on_start(&mut self) {}

    /// Asks app if it wants to stop execution, considered even when running with a counted number of frames
    fn cancel_poll(&self) -> bool;

    /// Called with all other android events that `AppContainer` isn't ready to handle, usually can be ignored
    #[inline]
    fn handle_android_event(&mut self, event: android_glue::Event) {}

    /// Called when we get a custom window event
    #[inline]
    fn handle_custom_event(&mut self, event_id: EventId, event: Arc<dyn Any>, timestamp: Option<TimeStamp>) {}

    /// Called when we get an input event
    #[inline]
    fn input(&mut self, input: Input, timestamp: Option<TimeStamp>) {}
}
