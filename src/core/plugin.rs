
use crate::core::player::ecs::*;
use bevy::prelude::*;
use heron::prelude::*;
use crate::core::enemy::enemy_movement::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(PhysicsPlugin::default())
            .add_startup_system(setup_camera_system.system())
            .add_startup_system(spawn_player_system.system())
            .add_startup_system(spawn_enemies_system.system())
            .add_system_to_stage(CoreStage::Update, enemy_movement_system.system())
            .add_system(player_movement_system.system())
            .add_system(camera_follow_system.system())
            .add_system(debug_system.system());
    }
}

pub fn debug_system(query: QuerySet<(Query<(&Player, &Transform, &Velocity, &Acceleration)>, Query<(&Enemy, &Transform, &Velocity, &Acceleration)>)>) {
    for (_, transform, velocity, acceleration) in query.q0().iter() {
        println!("player - t: {:?} // v: {:?} // a: {:?}", transform.translation, velocity.linear, acceleration.linear);
    }

    for (_, transform, velocity, acceleration) in query.q1().iter() {
        println!("enemy - t: {:?} // v: {:?} // a: {:?}", transform.translation, velocity.linear, acceleration.linear);
    }
}
