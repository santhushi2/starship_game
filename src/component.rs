use bevy::prelude::*;

// Starship component
#[derive(Component)]
pub struct Ship;

// Box entity component
#[derive(Component)]
pub struct BoxEntity;

// Box movement direction component
#[derive(Component)]
pub struct BoxDirection(pub Vec3);

// Fireball component (for destruction animation)
#[derive(Component)]
pub struct Fireball;

// Start point component
#[derive(Component)]
pub struct StartPoint;

// End point component
#[derive(Component)]
pub struct EndPoint;

// Game timer resource
#[derive(Resource, Component)]
pub struct GameTimer(pub Option<f32>, pub bool);

// Starship lives resource
#[derive(Resource, Component)]
pub struct ShipLives(pub u32);

// Laser component
#[derive(Component)]
pub struct Laser;

// Fireball sprite atlas for explosion
#[derive(Resource)]
pub struct FireballAtlas(pub Handle<TextureAtlas>);

// Fireball animation timer
#[derive(Component)]
pub struct FireballAnimationTimer(pub Timer);
