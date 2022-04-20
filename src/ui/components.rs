use bevy::prelude::*;

// UIオブジェクト描画用コンポーネント
#[derive(Debug, Default, Clone, Component)]
pub(crate) struct UIView {
    pub(crate) position: Vec2,
}

impl UIView {
    // pub(crate) fn new(position_vec2: Vec2) -> Self {
    //     Self {
    //         position: position_vec2,
    //     }
    // }

    pub(crate) fn from_ui_position(ui_position: UIPosition) -> Self {
        Self {
            position: ui_position.vec2,
        }
    }
}

impl From<UIView> for Rect<Val> {
    fn from(ui_view: UIView) -> Self {
        Self {
            right: Val::Px(ui_view.position.x),
            bottom: Val::Px(ui_view.position.y),
            ..default()
        }
    }
}

#[derive(Debug, Default, Clone, Component)]
pub(crate) struct UIPosition {
    pub(crate) vec2: Vec2,
}

impl UIPosition {
    pub(crate) fn new(x: f32, y: f32) -> Self {
        Self {
            vec2: Vec2::new(x, y),
        }
    }
}

impl From<UIPosition> for Rect<Val> {
    fn from(position: UIPosition) -> Self {
        Self {
            right: Val::Px(position.vec2.x),
            bottom: Val::Px(position.vec2.y),
            ..default()
        }
    }
}

#[derive(Debug, Default, Clone, Component)]
pub(crate) struct UISquareSize {
    pub(crate) size: f32,
}

impl From<UISquareSize> for Size<Val> {
    fn from(ui_square_size: UISquareSize) -> Self {
        Self::new(Val::Px(ui_square_size.size), Val::Px(ui_square_size.size))
    }
}

#[derive(Component)]
pub(crate) struct MouseControllerBase;

#[derive(Component)]
pub(crate) struct MouseControllerMain;
