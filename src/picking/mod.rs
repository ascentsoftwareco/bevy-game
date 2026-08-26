use bevy::{picking::mesh_picking::MeshPickingPlugin, prelude::*};

mod mesh_picking;

pub use mesh_picking::{rotate_on_drag, update_material_on};

pub fn plugin(app: &mut App) {
    app.add_plugins(MeshPickingPlugin)
        .add_plugins(mesh_picking::plugin);
}