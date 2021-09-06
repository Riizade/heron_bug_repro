use bevy::prelude::*;

pub fn closest_position(source: &Vec2, targets: &Vec<Vec2>) -> Option<Vec2> {
    if targets.len() == 0 {
        None
    } else {
        let mut closest_target = targets.first().unwrap();
        let mut closest_distance = source.distance(*closest_target).abs();

        for target in targets {
            let distance = source.distance(*target).abs();
            if distance < closest_distance {
                closest_target = target;
                closest_distance = distance;
            }
        }

        Some(closest_target.to_owned())
    }
}