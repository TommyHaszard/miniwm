use smithay::{desktop::Window, utils::{Logical, Point, Rectangle, Size}};

use crate::{layout::tile::Tile, utils_wm::id::IdCounter};

pub struct Workspace {
    pub workspace_id: WorkspaceId,
    floating_space: FloatingSpace,
    pub tiling_space: TilingSpace,
}

impl Workspace {
    pub fn default() -> Self {
        Workspace {
            workspace_id: WorkspaceId::next(),
            floating_space: FloatingSpace::default(),
            tiling_space: TilingSpace::default(),
        }
    }
}

static WORKSPACE_ID_COUNTER: IdCounter = IdCounter::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorkspaceId(u64);

impl WorkspaceId {
    pub fn next() -> WorkspaceId {
        WorkspaceId(WORKSPACE_ID_COUNTER.next())
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

struct FloatingSpace {
    tiles: Vec<Tile>,
}

impl FloatingSpace {
    pub fn default() -> Self {
        FloatingSpace { tiles: Vec::new() }
    }
}

pub struct TilingSpace {
    pub tiles: Vec<Tile>,
    pub layout_mode: TilingLayoutMode,
}

impl TilingSpace {
    pub fn default() -> Self {
        TilingSpace {
            tiles: Vec::new(),
            layout_mode: TilingLayoutMode::Grid { columns: 2 },
        }
    }

    pub fn retile(&mut self, output_view: Rectangle<i32, Logical>) -> Vec<(Window, Rectangle<i32, Logical>)> {
        match self.layout_mode {
            TilingLayoutMode::MasterStack {
                master_idx,
                master_ratio,
            } => todo!(),
            TilingLayoutMode::Grid { columns } => self.retile_grid(columns, output_view),
            TilingLayoutMode::Fibonacci => self.retile_fibonnaci(),
        }
    }

    fn retile_fibonnaci(&self) -> Vec<(Window, Rectangle<i32, Logical>)> {
        Vec::new()
    }

    fn retile_grid(&mut self, columns: usize, output_view: Rectangle<i32, Logical>) -> Vec<(Window, Rectangle<i32, Logical>)>{
        let tile_count = self.tiles.len();
        if tile_count == 0 {
            return Vec::new()
        }

        let rows = (tile_count + columns - 1) / columns;

        let tile_width = (output_view.size.w) / columns as i32;
        let tile_height = (output_view.size.h) / rows as i32;
            
        let mut vec = Vec::new();
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            let col = (i % columns) as i32;
            let row = (i / columns) as i32;

            tile.geometry = Rectangle {
                loc: Point::new(output_view.loc.x + (col * tile_width), output_view.loc.y + (row * tile_height)),
                size: Size::new(tile_width, tile_height),

            };

            if let Some(toplevel) = tile.window.toplevel() {
                toplevel.with_pending_state(|state| {
                    state.size = Some(tile.geometry.size);
                });
                toplevel.send_configure();
            }
            vec.push((tile.window.clone(), tile.geometry.clone()));
        }
        vec

    }
}

pub enum TilingLayoutMode {
    MasterStack {
        master_idx: usize,
        master_ratio: f32,
    },
    Grid {
        columns: usize,
    },
    Fibonacci,
}
