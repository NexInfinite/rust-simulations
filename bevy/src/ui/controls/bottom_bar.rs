use bevy::prelude::*;

use crate::{
    resources::global::*,
    ui::controls::{options_button, play_pause_button, time_label},
};

pub struct UiBottomBar;

impl Plugin for UiBottomBar {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);

        // Options systems
        app.add_systems(Update, options_button::click);
        app.add_systems(Update, options_button::keybinds.run_if(main_in_focus));

        // Play/Pause systems
        app.add_systems(
            Update,
            (
                play_pause_button::click,
                play_pause_button::sync_button_label,
            ),
        );

        // Time label
        app.add_systems(Update, time_label::sync_label_text);
    }
}

fn startup(mut commands: Commands) {
    commands.spawn(create_bottom_bar());
}

fn create_bottom_bar() -> impl Bundle + use<> {
    (
        Node {
            justify_content: JustifyContent::SpaceBetween,
            width: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(15.0)),
            position_type: PositionType::Absolute,
            bottom: Val::Px(0.0),
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            (
                Node {
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(15.0),
                    ..default()
                },
                children![
                    play_pause_button::create_play_pause_button(),
                    time_label::create_time_label(),
                ],
            ),
            options_button::create_options_button(),
        ],
    )
}
