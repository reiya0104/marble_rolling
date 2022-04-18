use bevy::prelude::Component;

// オブジェクト描画用コンポーネント
#[derive(Debug, Component)]
struct ObjectView;

// 位置情報などのコンポーネント
#[derive(Debug, Component)]
struct Acceleration;

#[derive(Debug, Component)]
struct Velocity;

#[derive(Debug, Component)]
struct Position;

// 衝突用のコンポーネント
#[derive(Debug, Component)]
struct Collision;

// 天板用のコンポーネント
#[derive(Debug, Component)]
struct NormalVector;

// カメラ用のコンポーネン
#[derive(Debug, Component)]
struct LookAt;

#[derive(Debug, Component)]
struct Audio;

// Entity 用コンポーネント
#[derive(Debug, Component)]
struct Marble;

#[derive(Debug, Component)]
struct Actor;

#[derive(Debug, Component)]
struct Board;

#[derive(Debug, Component)]
struct Tile;

#[derive(Debug, Component)]
struct Camera;

#[derive(Debug, Component)]
struct Start;

#[derive(Debug, Component)]
struct Goal;

#[derive(Debug, Component)]
struct Item;

#[derive(Debug, Component)]
struct Enemy;
