#![allow(irrefutable_let_patterns)]

mod handlers;

mod backends;
mod grabs;
mod input;
mod state;

use smithay::{reexports::{
        calloop::EventLoop,
        wayland_server::{Display, DisplayHandle},
    }
};
pub use state::MiniWm;

use crate::backends::{Backend, winit::WinitBackend};

pub struct CalloopData {
    state: MiniWm,
    display_handle: DisplayHandle,
    backend: Backend
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(env_filter) = tracing_subscriber::EnvFilter::try_from_default_env() {
        tracing_subscriber::fmt().with_env_filter(env_filter).init();
    } else {
        tracing_subscriber::fmt().init();
    }

    let mut event_loop: EventLoop<CalloopData> = EventLoop::try_new()?;

    let display: Display<MiniWm> = Display::new()?;
    let display_handle = display.handle();
    let mut state = MiniWm::new(&mut event_loop, display);
    let (winit_backend, winit_event_loop) = WinitBackend::new(&mut state)?;
    let mut backend = Backend::Winit(winit_backend);
    backend.init(winit_event_loop, &mut event_loop)?;

    let mut data = CalloopData {
        state,
        display_handle,
        backend,
    };


    event_loop.run(None, &mut data, move |_| {})?;

    Ok(())
}
