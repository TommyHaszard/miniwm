use smithay::{
    desktop::Window,
    output::Output,
    utils::{Logical, Rectangle},
};

use crate::layout::{
    WindowTarget, compute_working_area,
    tile::{Tile, TileId},
    workspace::{Workspace, WorkspaceId},
};

pub struct Layout {
    workspaces: Vec<Workspace>,
    current_workspace_id: Option<WorkspaceId>,
    output: Option<Output>,
    working_zone: Rectangle<f64, Logical>,
}

impl Layout {
    pub fn default() -> Self {
        let mut layout = Layout {
            workspaces: Vec::new(),
            current_workspace_id: None,
            output: None,
            working_zone: Default::default(),
        };

        layout.add_workspace(None);
        layout
    }

    pub fn new(output: Output) -> Self {
        let working_zone = compute_working_area(&output);
        let mut layout = Layout {
            workspaces: Vec::new(),
            current_workspace_id: None,
            output: Some(output),
            working_zone,
        };

        layout.add_workspace(None);
        layout
    }

    pub fn add_workspace(&mut self, workspace: Option<Workspace>) {
        if self.workspaces.is_empty() {
            if workspace.is_none() {
                let workspace = Workspace::default();
                self.current_workspace_id = Some(workspace.workspace_id.clone());
                self.workspaces.push(workspace);
            }
        }
    }

    pub fn add_window(&mut self, window: Window, target: WindowTarget) -> Vec<(Window, Rectangle<i32, Logical>)> {
        let tile = Tile::new(window);
        let tile_id = tile.tile_id.clone();
        if self.workspaces.is_empty() {
            let mut workspace = Workspace::default();
            self.current_workspace_id = Some(workspace.workspace_id.clone())
        }

        match target {
            WindowTarget::Auto => {
                let workspace_id = &self.current_workspace_id.clone();
                tracing::info!("Added Tile {:?} to workspace {:?}", tile_id, workspace_id);
                tile.set_geometry(&self.working_zone);
                self.add_window_to_workspace(tile, workspace_id);
            }
            WindowTarget::Workspace(workspace_id) => {
                self.add_window_to_workspace(tile, &Some(workspace_id))
            }
            WindowTarget::NextTo(tile_id) => todo!(),
        };

        self.retile(target)
    }

    fn add_window_to_workspace(&mut self, tile: Tile, workspace_id: &Option<WorkspaceId>) {
        if let Some(id) = workspace_id {
            let workspace = self.workspaces.iter_mut().find(|ws| ws.workspace_id == *id);

            if let Some(ws) = workspace {
                ws.tiling_space.tiles.push(tile);
            }
        }
    }

    pub fn retile(&mut self, target: WindowTarget) -> Vec<(Window, Rectangle<i32, Logical>)>{
        // based on the window target we know where we need to retile
        match target {
            WindowTarget::Auto => {
                // get the windows in the workstation in the future and recalculate
                let workspace_id = &self.current_workspace_id.clone();
                if let Some(id) = workspace_id {
                    let workspace = self
                        .workspaces
                        .iter_mut()
                        .find(|ws| ws.workspace_id == *id);

                    if let Some(ws) = workspace {
                        return ws.tiling_space.retile(self.working_zone.to_i32_down());
                    }
                }
            }
            WindowTarget::Workspace(workspace_id) => {}
            WindowTarget::NextTo(tile_id) => todo!(),
        }

        Vec::new()
    }

    pub fn set_output(&mut self, output: Output) {
        let working_zone = compute_working_area(&output);
        self.output = Some(output);
        self.working_zone = working_zone;
    }
}
