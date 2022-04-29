pub(crate) const GRAVITY: f32 = 9.80665;

// Tile (タイル)

pub(crate) const TILE_LONG_X_COUNT: usize = 5;
pub(crate) const TILE_LONG_Y_COUNT: usize = 2;
pub(crate) const TILE_LONG_Z_COUNT: usize = 3;

pub(crate) const TILE_SHORT_X_COUNT: usize = TILE_LONG_X_COUNT + 1;
pub(crate) const TILE_SHORT_Y_COUNT: usize = TILE_LONG_Y_COUNT + 1;
pub(crate) const TILE_SHORT_Z_COUNT: usize = TILE_LONG_Z_COUNT + 1;

pub(crate) const TILE_ALL_X_COUNT: usize = TILE_LONG_X_COUNT + TILE_SHORT_X_COUNT;
pub(crate) const TILE_ALL_Y_COUNT: usize = TILE_LONG_Y_COUNT + TILE_SHORT_Y_COUNT;
pub(crate) const TILE_ALL_Z_COUNT: usize = TILE_LONG_Z_COUNT + TILE_SHORT_Z_COUNT;

pub(crate) const TILE_LONG_WIDTH: f32 = 1.0;
pub(crate) const TILE_SHORT_WIDTH: f32 = 0.2;

pub(crate) const TILE_WIDTH_AVERAGE: f32 = (TILE_LONG_WIDTH + TILE_SHORT_WIDTH) / 2.0;

pub(crate) const TILE_ALL_X_WIDTH: f32 =
    TILE_LONG_X_COUNT as f32 * TILE_LONG_WIDTH + TILE_SHORT_X_COUNT as f32 * TILE_SHORT_WIDTH;

// pub(crate) const TILE_ALL_Y_WIDTH: f32 =
//     TILE_LONG_Y_COUNT as f32 * TILE_LONG_WIDTH + TILE_SHORT_Y_COUNT as f32 * TILE_SHORT_WIDTH;

pub(crate) const TILE_ALL_Z_WIDTH: f32 =
    TILE_LONG_Z_COUNT as f32 * TILE_LONG_WIDTH + TILE_SHORT_Z_COUNT as f32 * TILE_SHORT_WIDTH;

// UI
pub(crate) const CONTROLLER_BASE_SIZE: f32 = 150.0;
pub(crate) const CONTROLLER_BASE_POSITION_X: f32 = 50.0;
pub(crate) const CONTROLLER_BASE_POSITION_Y: f32 = 50.0;

pub(crate) const CONTROLLER_MAIN_SIZE: f32 = 100.0;
pub(crate) const CONTROLLER_MAIN_MAIX_RADIUS: f32 = 45.0;
