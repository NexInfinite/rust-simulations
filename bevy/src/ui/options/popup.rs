use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};

use crate::{
    resources::global::*,
    ui::options::{
        select_sim::{create_select_sim_dropdown, select_sim_dropdown_click},
        title::{options_create_title, options_exit_button},
    },
};

pub struct UiChangeSimPopup;

#[derive(Component)]
pub struct ChangeSimPopup;

impl Plugin for UiChangeSimPopup {
    fn build(&self, app: &mut App) {
        app.add_observer(on_popup_spawned);
        app.add_observer(on_popup_despawned);
        app.add_systems(
            Update,
            (options_exit_button, select_sim_dropdown_click, controls)
                .run_if(change_sim_popup_in_focus),
        );
    }
}

fn on_popup_spawned(_: On<Add, ChangeSimPopup>, mut global_resource: ResMut<GlobalResource>) {
    global_resource.focus = crate::resources::global::FocusEnum::ChangeSim;
}

fn on_popup_despawned(_: On<Remove, ChangeSimPopup>, mut global_resource: ResMut<GlobalResource>) {
    global_resource.focus = crate::resources::global::FocusEnum::Main;
}

pub fn create_sim_popup() -> impl Bundle + use<> {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ChangeSimPopup,
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
        GlobalZIndex(100),
        children![(
            Node {
                width: Val::Percent(70.0),
                border: UiRect::all(Val::Px(2.0)),
                padding: UiRect {
                    left: Val::Px(15.0),
                    right: Val::Px(10.0),
                    top: Val::Px(10.0),
                    bottom: Val::Px(10.0)
                },
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                ..default()
            },
            BorderColor::all(Color::from(WHITE)),
            BackgroundColor(Color::from(BLACK)),
            children![options_create_title(), create_select_sim_dropdown()],
        )],
    )
}

fn controls(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    popup_query: Query<Entity, With<ChangeSimPopup>>,
) {
    if input.pressed(KeyCode::Escape) {
        if let Ok(e) = popup_query.single() {
            commands.entity(e).despawn();
        } else {
            commands.spawn(create_sim_popup());
        }
    }
}
