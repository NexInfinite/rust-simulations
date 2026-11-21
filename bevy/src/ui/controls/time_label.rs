use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};

use crate::resources::{global::*, simulation_timing::*};

#[derive(Component)]
pub struct TimeLabel;

pub fn create_time_label() -> impl Bundle + use<> {
    (
        Node {
            padding: PADDING_LARGE,
            border: OUTLINE,
            ..default()
        },
        BorderColor::all(Color::from(WHITE)),
        BackgroundColor(Color::from(BLACK)),
        Text::new("Time: 0.00s"),
        TextFont {
            font_size: FONT_LARGE,
            ..default()
        },
        TextColor(Color::from(WHITE)),
        TimeLabel,
    )
}

pub fn sync_label_text(
    mut text_query: Query<&mut Text, With<TimeLabel>>,
    simulation_timing: Res<SimulationTiming>,
) {
    if !simulation_timing.is_changed() {
        return;
    }

    if let Ok(mut text) = text_query.single_mut() {
        text.0 = format!("Time: {:.2}s", simulation_timing.time)
    }
}
