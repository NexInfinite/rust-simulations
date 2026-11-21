use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};

use crate::resources::global::{GlobalResource, change_sim_popup_in_focus};

pub struct UiChangeSimPopup;

#[derive(Component)]
pub struct ChangeSimPopup;

impl Plugin for UiChangeSimPopup {
    fn build(&self, app: &mut App) {
        app.add_observer(on_popup_spawned);
        app.add_observer(on_popup_despawned);
        app.add_systems(
            Update,
            (exit_button, controls).run_if(change_sim_popup_in_focus),
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
            ..default()
        },
        ChangeSimPopup,
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
        GlobalZIndex(100),
        children![(
            Node {
                width: Val::Percent(70.0),
                height: Val::Percent(80.0),
                left: Val::Percent(15.0),
                top: Val::Percent(10.0),
                border: UiRect::all(Val::Px(2.0)),
                display: Display::Flex,
                ..default()
            },
            BorderColor::all(Color::from(WHITE)),
            BackgroundColor(Color::from(BLACK)),
            children![(children![(create_popup_exit(), create_popup_options())]),],
        )],
    )
}

fn create_popup_options() -> impl Bundle + use<> {}

#[derive(Component)]
struct ExitButton;

fn create_popup_exit() -> impl Bundle + use<> {
    (
        Node {
            // top: Val::Percent(2.0),
            // left: Val::Percent(2.0),
            border: UiRect::all(Val::Px(2.0)),
            // padding: UiRect {
            //     left: Val::Px(5.0),
            //     right: Val::Px(5.0),
            //     top: Val::Px(2.5),
            //     bottom: Val::Px(2.5),
            // },
            height: Val::Px(100.0),
            width: Val::Px(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ExitButton,
        BorderColor::all(Color::from(WHITE)),
        BackgroundColor(Color::from(BLACK)),
        children![(
            Text::new("Exit"),
            TextFont {
                font_size: FontSize::Px(16.0),
                ..default()
            },
        )],
    )
}

fn exit_button(
    mut commands: Commands,
    mut exit_query: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
    popup_query: Query<Entity, With<ChangeSimPopup>>,
) {
    for interaction in &mut exit_query {
        if matches!(interaction, Interaction::Pressed)
            && let Ok(e) = popup_query.single()
        {
            commands.entity(e).despawn();
        }
    }
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
