use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};

use crate::resources::timing::SimulationTiming;

pub struct UiPlayPauseButton;
#[derive(Component)]
pub struct PlayPauseButton;
#[derive(Component)]
pub struct TimeLabel;

impl Plugin for UiPlayPauseButton {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, play_pause_button_system);
        app.add_systems(Update, (update_play_pause_label, update_time_label));
    }
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(play_pause_button(&assets));
}

fn play_pause_button(_asset_server: &AssetServer) -> impl Bundle + use<> {
    (
        // Container
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![
            // Play/Pause Button
            (
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(15.0),
                    left: Val::Px(15.0),
                    width: Val::Px(100.0),
                    height: Val::Px(45.0),
                    border: UiRect::all(Val::Px(2.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor(Color::from(WHITE)),
                BackgroundColor(Color::from(BLACK)),
                // Button text
                children![(
                    Text::new("Pause"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::from(WHITE)),
                )],
                PlayPauseButton
            ),
            // Time text
            (
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(75.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                children![(
                    Text::new("Time: 0.00s"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::from(WHITE)),
                )],
                TimeLabel
            )
        ],
    )
}

fn play_pause_button_system(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<PlayPauseButton>)>,
    mut simulation_timing: ResMut<SimulationTiming>,
) {
    for interaction in &mut interaction_query {
        // Check if we clicked it
        if matches!(interaction, Interaction::Pressed) {
            simulation_timing.paused = !simulation_timing.paused;
        }
    }
}

fn update_play_pause_label(
    mut text_query: Query<&mut Text>,
    mut button_query: Query<&Children, With<PlayPauseButton>>,
    simulation_timing: ResMut<SimulationTiming>,
) {
    for children in &mut button_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        **text = if simulation_timing.paused {
            "Play".to_string()
        } else {
            "Pause".to_string()
        }
    }
}

fn update_time_label(
    mut text_query: Query<&mut Text>,
    mut button_query: Query<&Children, With<TimeLabel>>,
    simulation_timing: ResMut<SimulationTiming>,
) {
    for children in &mut button_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        **text = format!("Time: {:.2}s", simulation_timing.time)
    }
}
