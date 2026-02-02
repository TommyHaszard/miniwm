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
        FloatingSpace {
            tiles: Vec::new()
        }
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
            layout_mode: TilingLayoutMode::Fibonacci
        }
    }
}

enum TilingLayoutMode {
    MasterStack { master_idx: usize, master_ratio: f32 },
    Grid { columns: usize },
    Fibonacci,
}

