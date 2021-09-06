use bevy::prelude::*;
use crate::core::player::ecs::Player;
use crate::core::utils::position::*;
use bevy::math::swizzles::Vec3Swizzles;
use heron::prelude::*;
use crate::core::collision_layers::CollisionLayer;
use crate::core::constants::*;

#[derive(Default, Debug, Clone)]
pub struct Enemy;

#[derive(Default, Clone, Bundle)]
pub struct EnemyBundle {
    #[bundle]
    pub sprite: SpriteBundle,
    pub enemy: Enemy,
    pub rigid_body: RigidBody,
    pub collider: CollisionShape,
    pub collision_layers: CollisionLayers,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub physics_material: PhysicMaterial,
}


pub fn spawn_enemies_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    // spawn enemies
    for i in 0..NUM_ENEMIES {
        let enemy_texture_handle = asset_server.load("player_red.png");
        commands.spawn_bundle(EnemyBundle {
            sprite: SpriteBundle {
                material: materials.add(enemy_texture_handle.into()),
                sprite: Sprite {
                    size: Vec2::new(32.0, 32.0),
                    ..Default::default()
                },
                transform: Transform::from_xyz(i as f32 * 100.0, 0.0, 1.0),
                ..Default::default()
            },
            enemy: Enemy,
            rigid_body: RigidBody::Dynamic,
            collider: CollisionShape::Sphere { radius: 16.0 },
            velocity: Velocity::default(),
            acceleration: Acceleration::default(),
            collision_layers: CollisionLayer::Enemy.layers(),
            physics_material: PhysicMaterial { friction: 1.0, density: 1.0, ..Default::default() },
        });
    }
}

// performs enemy movement using current pathfinding setup
pub fn enemy_movement_system(mut query_set: QuerySet<(Query<(&Player, &Transform)>, Query<(&Enemy, &mut Transform, &mut Velocity)>)>) {
    let player_positions = query_set.q0().iter().map(|(_, transform)| transform.translation.xy()).collect();
    for (_,  mut transform, mut velocity) in query_set.q1_mut().iter_mut() {
        let self_position: Vec2 = transform.translation.xy();

        let closest_target = closest_position(&transform.translation.xy(), &player_positions);

        let destination: Vec2 = match closest_target {
            Some(target) => {
                let distance_to_target = self_position.distance(target).abs();
                if distance_to_target < ENEMY_DESIRED_MAX_DISTANCE && distance_to_target > ENEMY_DESIRED_MIN_DISTANCE {
                    transform.translation.xy() // current position
                } else if distance_to_target > ENEMY_DESIRED_MAX_DISTANCE {
                    // point from target to self
                    let direction = (self_position - target).normalize();
                    let destination = target + direction * (ENEMY_DESIRED_MAX_DISTANCE);
                    destination
                } else { // distance_to_target < MIN_DISTANCE
                    // point from target to self
                    let direction = (self_position - target).normalize();
                    let destination = target + direction * (ENEMY_DESIRED_MIN_DISTANCE);
                    destination
                }
            },
            None => transform.translation.xy(),
        };

        let current_position = transform.translation.xy();
        let difference_vector = destination - current_position;
        let desired_distance = difference_vector.length();
        let direction = difference_vector.normalize();

        // if we're not there yet
        if desired_distance > 0.0001 {
            // update velocity vector
            let velocity_vector = direction * ENEMY_SPEED;
            velocity.linear = velocity_vector.extend(0.0);
        }

        // update rotation
        let rotation = Quat::from_rotation_z(Vec2::Y.angle_between(direction));
        transform.rotation = rotation;
    }
}