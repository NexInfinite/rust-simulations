use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};

use crate::{
    resources::global::main_in_focus,
    ui::change_sim::popup::{ChangeSimPopup, create_sim_popup},
};

// use crate::simulations::selector::SimulationState;

pub struct UiChangeSimButton;

#[derive(Component)]
struct ChangeSimButton;

impl Plugin for UiChangeSimButton {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, handle_change_sim_click);
        app.add_systems(Update, (controls).run_if(main_in_focus));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(select_sim());
}

fn select_sim() -> impl Bundle + use<> {
    (
        // Container
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Button,
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(15.0),
                left: Val::Px(15.0),
                width: Val::Px(150.0),
                height: Val::Px(45.0),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor::all(Color::from(WHITE)),
            BackgroundColor(Color::from(BLACK)),
            // Button text
            children![(
                Text::new("Options asdasd"),
                TextFont {
                    font_size: FontSize::Px(24.0),
                    ..default()
                },
            )],
            ChangeSimButton
        ),],
    )
}

fn handle_change_sim_click(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ChangeSimButton>)>,
    popup_plugin_query: Query<Entity, With<ChangeSimPopup>>,
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

fn controls(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    popup_plugin_query: Query<Entity, With<ChangeSimPopup>>,
) {
    if input.pressed(KeyCode::KeyO) {
        if popup_plugin_query.single().is_ok() {
            return;
        }

        commands.spawn(create_sim_popup());
    }
}
