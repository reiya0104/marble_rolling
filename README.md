# Marble Rolling (ビー玉転がし)

ビー玉転がしの 3D ゲーム

## 完成像

天板の上が迷路のようになっていて，その上にあるビー玉を天板を傾けることによって動かし，ゴールへと動かすようなゲームにする予定です．

## 目標

## 設計

### Entity

- Marble (ビー玉)
- Actor (メインビー玉)
- Board (天板)
- Tile (壁・地面のタイル)
- Light (光)
- Camera (カメラ)
- Start (スタート)
- Goal (ゴール)
- MouseController (コントローラ)
    - MouseControllerBase
    - MouseControllerMain
- Item (アイテム) ? (できたら)
- Enemy (敵) ? (できたら)

### Compponent (コンポーネント)

オブジェクト描画用コンポーネント

- ObjectView コンポーネント  
    対象 Entity: Marble, Actor, Board, Tile, Light, Camera, MouseController, (Item, Enemy)

位置情報などのコンポーネント

- Acceleration (加速度) コンポーネント  
    対象 Entity: Marble, Actor, Board, Camera, (Item, Enemy)
- Velocity (速度) コンポーネント  
    対象 Entity: Marble, Actor, Board, Camera, (Item, Enemy)
- Position (位置) コンポーネント  
    対象 Entity: Marble, Actor, Board, Tile, Light, Camera, MouseController(Base, Main), (Item, Enemy)

衝突用のコンポーネント

衝突や接触を意味する

- Collision (衝突) コンポーネント
    対象 Entity: Marble, Actor, Board, Tile, (Item, Enemy)


天板用のコンポーネント

- Normal Vector (法線ベクトル) コンポーネント  
    対象 Entity: Board, Tile

カメラ用のコンポーネント

- LookAt (目線) コンポーネント  
    対象 Entity: Camera

Entity 用のコンポーネント

- Marble コンポーネント  
    対象 Entity: Marble
- Actor コンポーネント  
    対象 Entity: Actor
- Board コンポーネント  
    対象 Entity: Board
- Tile コンポーネント  
    対象 Entity: Tile 
- Light コンポーネント
    対象 Entity: Light
- Camera コンポーネント  
    対象 Entity: Camera
- Start コンポーネント  
    対象 Entity: Start
- Goal コンポーネント  
    対象 Entity: Goal
- MouseControllerBase コンポーネント  
    対象 Entity: MouseControllerBase
- MouseControllerMain コンポーネント  
    対象 Entity: MouseControllerMain
- Item コンポーネント  
    対象 Entity: Item
- Enemy コンポーネント  
    対象 Entity: Enemy

音用 コンポーネント

- Audio コンポーネント
    対象 Entity: Todo!()

### リソース

- Input リソース  
    対象 Entity: Camera, MouseController

### システム

Input に関するシステム

- Input (リソース) の情報をもとに，対象 Entity のコンポーネント (Camera, MouseController) がもっている Velocity・Position (コンポーネント) を変更するシステム

Velocity・Position を変更するシステム

- Acceleration から Velocity を変更するシステム
- Velocity から Position を変更するシステム

Position が決まったときに起こるシステム
- Position から Collision を変更するシステム
- Position から ObjectView を変更するシステム

Collision が決まったときに起こるシステム

- Collision から，Acceleration・Velocity を変更するシステム
- Collision から，Audio を変更するシステム
