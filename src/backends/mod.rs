use smithay::{backend::winit::WinitEventLoop, output::Output, reexports::calloop::EventLoop};

use crate::{CalloopData, backends::winit::WinitBackend};

pub mod winit;

pub enum Backend {
    //Tty(Tty),
    Winit(WinitBackend),
    //Headless(Headless),
}

impl Backend {
    pub fn init(
        &mut self,
        winit_event_loop: WinitEventLoop,
        event_loop: &mut EventLoop<CalloopData>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Backend::Winit(winit) => Ok(winit.init(winit_event_loop, event_loop)?),
        }
    }

    pub fn get_output(&self) -> Output {
        match self {
            Backend::Winit(winit) => winit.output.clone(),
        }
    }
}
