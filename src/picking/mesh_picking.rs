//! Reusable mesh-picking helpers, adapted from
//! [bevy's mesh_picking example](https://github.com/bevyengine/bevy/blob/7c153070c018db5f606f59eb6b04963df7b56ed6/examples/picking/mesh_picking.rs).
//!
//! [`MeshPickingPlugin`](bevy::picking::mesh_picking::MeshPickingPlugin) is the backend that
//! actually raycasts against meshes; it's added in [`super::plugin`]. This module only provides
//! the debug gizmo and observer helpers for wiring picking behavior onto your own entities.

use bevy::{color::palettes::tailwind::*, picking::pointer::PointerInteraction, prelude::*};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, draw_mesh_intersections);
}

/// Returns an observer that updates the entity's material to the one specified.
///
/// Attach with `.observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))` etc. when
/// spawning a pickable mesh.
pub fn update_material_on<E: EntityEvent>(
    new_material: Handle<StandardMaterial>,
) -> impl Fn(On<E>, Query<&mut MeshMaterial3d<StandardMaterial>>) {
    move |event, mut query| {
        if let Ok(mut material) = query.get_mut(event.event_target()) {
            material.0 = new_material.clone();
        }
    }
}

/// A system that draws hit indicators for every pointer.
fn draw_mesh_intersections(pointers: Query<&PointerInteraction>, mut gizmos: Gizmos) {
    for (point, normal) in pointers
        .iter()
        .filter_map(|interaction| interaction.get_nearest_hit())
        .filter_map(|(_entity, hit)| hit.position.zip(hit.normal))
    {
        gizmos.sphere(point, 0.05, RED_500);
        gizmos.arrow(point, point + normal.normalize() * 0.5, PINK_100);
    }
}

/// An observer to rotate an entity when it is dragged.
///
/// Attach with `.observe(rotate_on_drag)` when spawning a draggable mesh.
pub fn rotate_on_drag(drag: On<Pointer<Drag>>, mut transforms: Query<&mut Transform>) {
    let mut transform = transforms.get_mut(drag.entity).unwrap();
    transform.rotate_y(drag.delta.x * 0.02);
    transform.rotate_x(drag.delta.y * 0.02);
}
