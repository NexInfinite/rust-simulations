use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};

use crate::resources::global::*;

#[derive(Component)]
pub struct SimDropdownButton;

#[derive(Component)]
struct SimDropdownLabel;

pub fn create_select_sim_dropdown() -> impl Bundle + use<> {
    (
        Node {
            display: Display::Grid,
            width: Val::Percent(100.0),
            grid_template_columns: vec![
                RepeatedGridTrack::flex(1, 1.0),
                RepeatedGridTrack::flex(1, 1.0),
            ],
            column_gap: Val::Px(10.0),
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            (
                Text::new("Select Sim:"),
                TextFont {
                    font_size: FONT_SMALL,
                    ..default()
                },
            ),
            (
                Button,
                SimDropdownButton,
                Node {
                    border: UiRect::all(Val::Px(2.0)),
                    padding: PADDING_SMALL,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor::all(Color::from(WHITE)),
                BackgroundColor(Color::from(BLACK)),
                children![
                    (
                        SimDropdownLabel,
                        Text::new("None selected..."),
                        TextFont {
                            font_size: FONT_SMALL,
                            ..default()
                        },
                    ),
                    Text::new("v"),
                ],
            ),
        ],
    )
}

pub fn select_sim_dropdown_click(
    q: Query<&Interaction, (Changed<Interaction>, With<SimDropdownButton>)>,
) {
    for interaction in &q {
        if *interaction == Interaction::Pressed {
            println!("CLICKED!!");
        }
    }
}
