use smithay::{desktop::Window, utils::{Logical, Rectangle}};

use crate::utils_wm::id::IdCounter;

pub struct Tile {
    pub tile_id: TileId,
    pub window: Window,
    pub geometry: Rectangle<i32, Logical>
}

static TILE_ID_COUNTER: IdCounter = IdCounter::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileId(u64);

impl TileId {
    pub fn next() -> TileId {
        TileId(TILE_ID_COUNTER.next())
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

impl Tile {
    pub fn new(window: Window) -> Self {
        Tile {
            tile_id: TileId::next(),
            geometry: window.geometry().clone(),
            window: window
        }
    }

    pub fn get_geometry(&self) -> Rectangle<i32, Logical> {
        self.window.geometry()
    }

    pub fn set_geometry(&self, geometry: &Rectangle<f64, Logical>) {
        if let Some(toplevel) = self.window.toplevel() {
            toplevel.with_pending_state(|state| {
                state.size = Some(geometry.to_i32_down().size);
            });
            toplevel.send_configure();
            tracing::info!("Set geometry: {:?}", geometry.size)
        }
    }
}
