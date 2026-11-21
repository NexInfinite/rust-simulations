use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};

use crate::{
    resources::global::{FONT_LARGE, OUTLINE, PADDING_LARGE},
    ui::options::popup::{OptionsPopup, create_sim_popup},
};

#[derive(Component)]
pub struct ChangeSimButton;

pub fn create_options_button() -> impl Bundle + use<> {
    (
        Button,
        Node {
            padding: PADDING_LARGE,
            border: OUTLINE,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor::all(Color::from(WHITE)),
        BackgroundColor(Color::from(BLACK)),
        // Button text
        children![(
            Text::new("Options"),
            TextFont {
                font_size: FONT_LARGE,
                ..default()
            },
        )],
        ChangeSimButton,
    )
}

pub fn click(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ChangeSimButton>)>,
    popup_plugin_query: Query<Entity, With<OptionsPopup>>,
) {
    for interaction in &mut interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            if let Ok(e) = popup_plugin_query.single() {
                commands.entity(e).despawn();
            } else {
                commands.spawn(create_sim_popup());
            }
        }
    }
}

pub fn keybinds(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    popup_plugin_query: Query<Entity, With<OptionsPopup>>,
) {
    if input.pressed(KeyCode::KeyO) {
        if popup_plugin_query.single().is_ok() {
            return;
        }

        commands.spawn(create_sim_popup());
    }
}
