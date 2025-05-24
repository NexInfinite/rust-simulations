use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};

use crate::resources::timing::SimulationTiming;

pub struct UiPlayPauseButton;

impl Plugin for UiPlayPauseButton {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (play_pause_button_system, update_play_pause_label));
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
        // Button
        children![(
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
            )]
        )],
    )
}

fn play_pause_button_system(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
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
    mut button_query: Query<&Children, With<Button>>,
    simulation_timing: ResMut<SimulationTiming>,
) {
    for children in &mut button_query {
        // Update text: this needs to happen constantly as timing can be changed from other means
        let mut text = text_query.get_mut(children[0]).unwrap();
        **text = if simulation_timing.paused {
            "Play".to_string()
        } else {
            "Pause".to_string()
        }
    }
}
