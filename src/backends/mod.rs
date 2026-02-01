use smithay::{backend::winit::WinitEventLoop, reexports::calloop::EventLoop};

use crate::{CalloopData, MiniWm, backends::winit::WinitBackend};

pub mod winit;

pub enum Backend {
    //Tty(Tty),
    Winit(WinitBackend),
    //Headless(Headless),
}

impl Backend {
    pub fn init(&mut self, winit_event_loop: WinitEventLoop, event_loop: &mut EventLoop<CalloopData>) -> Result<(), Box<dyn std::error::Error>>{
        match self {
            Backend::Winit(winit) => Ok(winit.init(winit_event_loop, event_loop)?),
        }
    }
}
