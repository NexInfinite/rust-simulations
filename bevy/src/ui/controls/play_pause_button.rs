use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};

use crate::resources::{global::*, simulation_timing::SimulationTiming};

#[derive(Component)]
pub struct PlayPauseButton;

pub fn create_play_pause_button() -> impl Bundle + use<> {
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
        children![(
            Text::new("Pause"),
            TextFont {
                font_size: FONT_LARGE,
                ..default()
            },
            TextColor(Color::from(WHITE)),
        )],
        PlayPauseButton,
    )
}

pub fn click(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<PlayPauseButton>)>,
    mut simulation_timing: ResMut<SimulationTiming>,
) {
    for interaction in &mut interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            simulation_timing.paused = !simulation_timing.paused;
        }
    }
}

pub fn sync_button_label(
    mut text_query: Query<&mut Text>,
    mut button_query: Query<&Children, With<PlayPauseButton>>,
    simulation_timing: Res<SimulationTiming>,
) {
    if !simulation_timing.is_changed() {
        return;
    }

    for children in &mut button_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        **text = if simulation_timing.paused {
            "Play".to_string()
        } else {
            "Pause".to_string()
        }
    }
}
