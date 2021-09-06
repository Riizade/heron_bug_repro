use crate::core::collision_layers::*;
use crate::core::constants::*;
use bevy::prelude::*;
use bevy::render::camera::Camera;
use heron::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct Player;

#[derive(Debug, Clone, Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub rigid_body: RigidBody,
    pub collider: CollisionShape,
    pub collision_layers: CollisionLayers,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub physics_material: PhysicMaterial,
}

// moves the player with the keyboard keys
pub fn player_movement_system(
    keys: Res<Input<KeyCode>>,
    mut player: Query<(&Player, &mut Velocity)>,
) {
    // TODO: after implementing axes, modulate speed by axis distance from origin
    let mut vector: Vec2 = Vec2::ZERO;

    if keys.pressed(KeyCode::W) {
        vector += Vec2::Y;
    }
    if keys.pressed(KeyCode::S) {
        vector -= Vec2::Y;
    }
    if keys.pressed(KeyCode::A) {
        vector -= Vec2::X;
    }
    if keys.pressed(KeyCode::D) {
        vector += Vec2::X;
    }

    if let Some((_, mut velocity)) = player.single_mut().ok() {
        if vector != Vec2::ZERO {
            let new_velocity = vector.normalize() * PLAYER_SPEED as f32;
            *velocity = Velocity::from_linear(Vec3::new(new_velocity.x, new_velocity.y, 0.0));
        } else {
            *velocity = Velocity::from_linear(Vec3::ZERO);
        }
    }
}

pub fn camera_follow_system(
    mut query: QuerySet<(
        Query<(&Player, &Transform)>,
        Query<(&Camera, &mut Transform)>,
    )>,
) {
    let player_transform = query.q0().single().unwrap().1.clone();
    let mut camera_transform = query.q1_mut().single_mut().unwrap().1;
    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

pub fn setup_camera_system(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub fn spawn_player_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player_texture_handle = asset_server.load("player_blue.png");
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            material: materials.add(player_texture_handle.into()),
            sprite: Sprite {
                size: *TILE_SIZE,
                ..Default::default()
            },
            transform: Transform::from_xyz(100.0, 100.0, 1.0),
            ..Default::default()
        })
        .insert_bundle(PlayerBundle::default());
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            rigid_body: RigidBody::Dynamic,
            collider: CollisionShape::Sphere { radius: 16.0 },
            velocity: Velocity::default(),
            acceleration: Acceleration::default(),
            collision_layers: CollisionLayer::Player.layers(),
            physics_material: PhysicMaterial { friction: 1.0, density: 1.0, ..Default::default() },
        }
    }
}
