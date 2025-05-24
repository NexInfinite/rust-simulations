use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};

use crate::timing::SimulationTiming;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, button_system);
    }
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(button(&assets));
}

fn button(_asset_server: &AssetServer) -> impl Bundle + use<> {
    (
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

fn button_system(
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut simulation_timing: ResMut<SimulationTiming>,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        if matches!(interaction, Interaction::Pressed) {
            simulation_timing.paused = !simulation_timing.paused;
            **text = if simulation_timing.paused {
                "Play".to_string()
            } else {
                "Pause".to_string()
            }
        }
    }
}
