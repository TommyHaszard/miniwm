use smithay::{
    desktop::layer_map_for_output,
    output::Output,
    utils::{Logical, Rectangle},
};

use crate::layout::{tile::TileId, workspace::WorkspaceId};

pub mod layout;
pub mod tile;
pub mod workspace;

pub enum WindowTarget {
    Auto,
    Workspace(WorkspaceId),
    NextTo(TileId),
}

fn compute_working_area(output: &Output) -> Rectangle<f64, Logical> {
    layer_map_for_output(output).non_exclusive_zone().to_f64()
}
