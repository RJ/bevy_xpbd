//! Renders physics objects and events like [AABBs](ColliderAabb) and [contacts](Collision) for debugging purposes.
//!
//! See [`PhysicsDebugPlugin`].

use crate::prelude::*;
use bevy::prelude::*;

/// Renders physics objects and events like [AABBs](ColliderAabb) and [contacts](Collision) for debugging purposes.
///
/// You can configure what is rendered using the [`PhysicsDebugConfig`] resource.
pub struct PhysicsDebugPlugin;

impl Plugin for PhysicsDebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PhysicsDebugConfig>()
            .insert_resource(GizmoConfig {
                line_width: 1.0,
                ..default()
            })
            .register_type::<PhysicsDebugConfig>()
            .add_systems(
                PostUpdate,
                debug_render_aabbs
                    .run_if(|config: Res<PhysicsDebugConfig>| config.render_aabbs)
                    .after(PhysicsSet::Sync),
            )
            .add_systems(
                PostUpdate,
                debug_render_contacts
                    .run_if(|config: Res<PhysicsDebugConfig>| config.render_contacts)
                    .after(PhysicsSet::Sync),
            );
    }
}

/// Controls the [`PhysicsDebugPlugin`] configuration.
#[derive(Reflect, Resource)]
#[reflect(Resource)]
pub struct PhysicsDebugConfig {
    /// Renders the Axis-Aligned Bounding Boxes of [colliders](`Collider`).
    pub render_aabbs: bool,
    /// Renders contact points.
    pub render_contacts: bool,
}

impl Default for PhysicsDebugConfig {
    fn default() -> Self {
        Self {
            render_aabbs: true,
            render_contacts: true,
        }
    }
}

fn debug_render_aabbs(aabbs: Query<&ColliderAabb>, mut gizmos: Gizmos) {
    #[cfg(feature = "2d")]
    for aabb in aabbs.iter() {
        gizmos.cuboid(
            Transform::from_scale(Vector::from(aabb.extents()).extend(0.0).as_f32())
                .with_translation(Vector::from(aabb.center()).extend(0.0).as_f32()),
            Color::WHITE,
        );
    }

    #[cfg(feature = "3d")]
    for aabb in aabbs.iter() {
        gizmos.cuboid(
            Transform::from_scale(Vector::from(aabb.extents()).as_f32())
                .with_translation(Vector::from(aabb.center()).as_f32()),
            Color::WHITE,
        );
    }
}

#[allow(clippy::unnecessary_cast)]
fn debug_render_contacts(mut collisions: EventReader<Collision>, mut gizmos: Gizmos) {
    #[cfg(feature = "2d")]
    for Collision(contact) in collisions.iter() {
        let p1 = contact.point1.as_f32();
        let p2 = contact.point2.as_f32();

        gizmos.line_2d(p1 - Vec2::X * 0.3, p1 + Vec2::X * 0.3, Color::CYAN);
        gizmos.line_2d(p1 - Vec2::Y * 0.3, p1 + Vec2::Y * 0.3, Color::CYAN);

        gizmos.line_2d(p2 - Vec2::X * 0.3, p2 + Vec2::X * 0.3, Color::CYAN);
        gizmos.line_2d(p2 - Vec2::Y * 0.3, p2 + Vec2::Y * 0.3, Color::CYAN);
    }
    #[cfg(feature = "3d")]
    for Collision(contact) in collisions.iter() {
        let p1 = contact.point1.as_f32();
        let p2 = contact.point2.as_f32();

        gizmos.line(p1 - Vec3::X * 0.3, p1 + Vec3::X * 0.3, Color::CYAN);
        gizmos.line(p1 - Vec3::Y * 0.3, p1 + Vec3::Y * 0.3, Color::CYAN);
        gizmos.line(p1 - Vec3::Z * 0.3, p1 + Vec3::Z * 0.3, Color::CYAN);

        gizmos.line(p2 - Vec3::X * 0.3, p2 + Vec3::X * 0.3, Color::CYAN);
        gizmos.line(p2 - Vec3::Y * 0.3, p2 + Vec3::Y * 0.3, Color::CYAN);
        gizmos.line(p2 - Vec3::Z * 0.3, p2 + Vec3::Z * 0.3, Color::CYAN);
    }
}
