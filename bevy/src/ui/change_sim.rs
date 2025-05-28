use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};

use crate::simulations::selector::SimulationState;

pub struct UiChangeSim;

#[derive(Component)]
struct SimButton {
    sim: SimulationState,
}

impl Plugin for UiChangeSim {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (handle_button_interaction, update_button_background),
        );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(change_sim());
}

fn change_sim() -> impl Bundle + use<> {
    (
        // Container
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        // Buttons
        children![
            // Circular Button
            (
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
                BorderColor(Color::from(WHITE)),
                // Button text
                children![(
                    Text::new("Circular"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                )],
                SimButton {
                    sim: SimulationState::Circular
                }
            ),
            // Linear Button
            (
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(15.0),
                    left: Val::Px(150.0 + 15.0 + 15.0),
                    width: Val::Px(150.0),
                    height: Val::Px(45.0),
                    border: UiRect::all(Val::Px(2.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor(Color::from(WHITE)),
                // Button text
                children![(
                    Text::new("Linear"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                )],
                SimButton {
                    sim: SimulationState::Linear
                }
            )
        ],
    )
}

fn handle_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &SimButton),
        (Changed<Interaction>, With<SimButton>),
    >,
    mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
    for (interaction, sim_button) in &mut interaction_query {
        // Check if we clicked it
        if matches!(interaction, Interaction::Pressed) {
            next_sim_state.set(sim_button.sim.clone());
        }
    }
}

fn update_button_background(
    mut text_color_query: Query<&mut TextColor>,
    mut button_query: Query<(&mut BackgroundColor, &Children, &SimButton), With<SimButton>>,
    sim_state: Res<State<SimulationState>>,
) {
    for (mut background_color, children, sim_button) in &mut button_query {
        let mut text_color = text_color_query.get_mut(children[0]).unwrap();
        if sim_state.get().clone() == sim_button.sim {
            background_color.0 = Color::from(WHITE);
            text_color.0 = Color::from(BLACK);
        } else {
            background_color.0 = Color::from(BLACK);
            text_color.0 = Color::from(WHITE);
        }
    }
}
