use heron::prelude::*;

#[derive(PhysicsLayer)]
pub enum CollisionLayer {
    Player,
    Enemy,
}

impl CollisionLayer {
    // defines which layers can interact with which other layers
    // an entity's layer groups determine which layers it is a part of
    // an entity's layer masks determine which layers it can collide/interact with
    // more docs here: https://docs.rs/heron/0.11.1/heron/struct.CollisionLayers.html
    pub fn layers(&self) -> CollisionLayers {
        match self {
            CollisionLayer::Player => CollisionLayers::none()
                .with_group(CollisionLayer::Player)
                .with_masks(vec![CollisionLayer::Enemy]),
            CollisionLayer::Enemy => CollisionLayers::none()
                .with_group(CollisionLayer::Enemy)
                .with_masks(vec![CollisionLayer::Player]),
        }
    }
}
