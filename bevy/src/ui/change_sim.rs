use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};

use crate::simulations::selector::SimulationState;

pub struct UiChangeSim;

#[derive(Component)]
struct ChangeSimButton;

impl Plugin for UiChangeSim {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, handle_change_sim_click);
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
                width: Val::Px(175.0),
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
                Text::new("Change Sim"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
            )],
            ChangeSimButton
        ),],
    )
}

fn handle_change_sim_click(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ChangeSimButton>)>,
) {
    for interaction in &mut interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            println!("Clicked")
        }
    }
}

// fn handle_change_sim_click(
//     mut interaction_query: Query<
//         (&Interaction, &SimButton),
//         (Changed<Interaction>, With<ChangeSimButton>),
//     >,
//     mut next_sim_state: ResMut<NextState<SimulationState>>,
// ) {
//     for (interaction, sim_button) in &mut interaction_query {
//         // Check if we clicked it
//         if matches!(interaction, Interaction::Pressed) {
//             next_sim_state.set(sim_button.sim.clone());
//         }
//     }
// }
