use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};

use crate::resources::global::GlobalResource;

pub struct UiChangeSimPopup;

#[derive(Component)]
pub struct ChangeSimPopup;

impl Plugin for UiChangeSimPopup {
    fn build(&self, app: &mut App) {
        app.add_observer(on_popup_spawned);
        app.add_observer(on_popup_despawned);
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
            ..default()
        },
        ChangeSimPopup,
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
        GlobalZIndex(100),
        children![(
            Node {
                width: Val::Percent(70.0),
                height: Val::Percent(80.0),
                left: Val::Percent(15.0),
                top: Val::Percent(10.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            children![(
                Node {
                    bottom: Val::Px(0.0),
                    left: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    border: UiRect::all(Val::Px(2.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor::all(Color::from(WHITE)),
                BackgroundColor(Color::from(BLACK)),
            ),],
        )],
    )
}
