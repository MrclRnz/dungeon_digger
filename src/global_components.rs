use bevy::ecs::component::Component;
use bevy::ecs::bundle::Bundle;
use bevy::ecs::system::Res;
use bevy::asset::AssetServer;
use bevy::ecs::system::Commands;
use bevy::sprite::SpriteBundle;
use bevy::utils::default;

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Weapon {
    damage: u8,
    weapon_type: WeaponType
}

enum WeaponType {
    RANGED(u8),
    MELEE
}

#[derive(Component)]
struct Health(f32);

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Player;

enum FloorType {
    CONCRETE,
    WATER,
    LAVA
}

#[derive(Component)]
enum Terrain {
    WALL,
    OBSTACLE,
    FLOOR(FloorType)
}

#[derive(Bundle)]
struct PlayerBundle {
    position: Position,
    health: Health,
    _p: Player,
}
