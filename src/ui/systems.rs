use crate::*;
use bevy::prelude::*;

pub(crate) fn setup_ui(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // base
    let base_size = UISquareSize {
        size: CONTROLLER_BASE_SIZE,
    };
    let base_position = UIPosition::new(CONTROLLER_BASE_POSITION_X, CONTROLLER_BASE_POSITION_Y);

    // main
    let main_size = UISquareSize {
        size: CONTROLLER_MAIN_SIZE,
    };
    let main_position = UIPosition::default();
    let main_max_radius = UIMaxSize::new(CONTROLLER_MAIN_MAIX_RADIUS);

    // mouse controller
    commands
        // base
        .spawn()
        .insert(base_size.clone())
        .insert(base_position.clone())
        .insert(UIView::from_ui_position(base_position.clone()))
        .insert_bundle(ImageBundle {
            style: Style {
                size: base_size.into(),
                position_type: PositionType::Absolute,
                position: base_position.into(),
                ..default()
            },
            image: asset_server.load("image/mouse_controller_base.png").into(),
            ..default()
        })
        .insert(MouseControllerBase)
        .with_children(|parent| {
            // main
            parent
                .spawn()
                .insert(main_size.clone())
                .insert(main_position.clone())
                .insert(main_max_radius)
                .insert(UIView::from_ui_position(main_position.clone()))
                .insert_bundle(ImageBundle {
                    style: Style {
                        size: main_size.into(),
                        position_type: PositionType::Absolute,
                        position: main_position.into(),
                        ..default()
                    },
                    image: asset_server.load("image/mouse_controller_main.png").into(),
                    ..default()
                })
                .insert(MouseControllerMain);
        });
}

pub(crate) fn update_mouse_coltroller_main_position(
    mut cursor_moved_events: EventReader<CursorMoved>,
    windows: Res<Windows>,
    query_base: Query<
        (&UIPosition, &UISquareSize),
        (With<MouseControllerBase>, Without<MouseControllerMain>),
    >,
    mut query_main: Query<
        (&mut UIPosition, &UISquareSize, &UIMaxSize),
        (With<MouseControllerMain>, Without<MouseControllerBase>),
    >,
) {
    // カーソルの位置にあった main_position に更新する

    // let max_controller_radius = 45.0;

    for event in cursor_moved_events.iter() {
        // カーソルの位置を取得
        let cursor_position = event.position;

        // ウィンドウ幅を取得するために window を取得
        if let Some(window) = windows.iter().last() {
            let iter = query_base.iter().zip(query_main.iter_mut());

            if let Some((
                (base_position, base_size),
                (mut main_position, _, max_controller_radius),
            )) = iter.last()
            {
                let base_center = Vec2::splat(base_size.size) / 2.0 + base_position.vec2;

                let cursor_position_relative =
                    Vec2::new(window.width() - cursor_position.x, cursor_position.y) - base_center;

                main_position.vec2 =
                    if cursor_position_relative.length() > max_controller_radius.size {
                        max_controller_radius.size * cursor_position_relative.normalize()
                    } else {
                        cursor_position_relative
                    };
            }
        }
    }
}

pub(crate) fn update_ui_view_by_mouse_coltroller_main_position(
    query_base: Query<&UISquareSize, (With<MouseControllerBase>, Without<MouseControllerMain>)>,
    query_main: Query<&UISquareSize, (With<MouseControllerMain>, Without<MouseControllerBase>)>,
    mut query: Query<(&mut UIView, &UIPosition), With<MouseControllerMain>>,
) {
    for (mut ui_view, ui_position) in query.iter_mut() {
        let iter = query_base.iter().zip(query_main.iter());
        if let Some((base_size, main_size)) = iter.last() {
            ui_view.position =
                ui_position.vec2 + Vec2::splat(base_size.size - main_size.size) / 2.0;
        }
    }
}

pub(crate) fn update_ui_style_by_ui_view(
    mut query: Query<(&mut Style, &UIView), With<MouseControllerMain>>,
) {
    for (mut style, ui_view) in query.iter_mut() {
        style.position = ui_view.clone().into();
    }
}
