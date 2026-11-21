use bevy::{
    color::palettes::css::{BLACK, RED, WHITE},
    prelude::*,
};

use crate::resources::global::*;

#[derive(Component)]
pub struct DropdownButton;

#[derive(Component)]
struct DropdownLabel;

#[derive(Component)]
pub struct DropdownList;

pub fn create_select_sim_dropdown() -> impl Bundle + use<> {
    (
        Node {
            display: Display::Grid,
            width: Val::Percent(100.0),
            grid_template_columns: vec![
                RepeatedGridTrack::flex(1, 1.0),
                RepeatedGridTrack::flex(1, 1.0),
            ],
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
                DropdownButton,
                Node {
                    border: OUTLINE,
                    padding: PADDING_SMALL,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor::all(Color::from(WHITE)),
                BackgroundColor(Color::from(BLACK)),
                children![
                    (
                        DropdownLabel,
                        Text::new("None selected..."),
                        TextFont {
                            font_size: FONT_SMALL,
                            ..default()
                        },
                    ),
                    Text::new("v"),
                ],
            ),
            (
                DropdownList,
                Node {
                    display: Display::None,
                    position_type: PositionType::Absolute,
                    padding: PADDING_SMALL,
                    top: Val::Percent(100.0),
                    width: Val::Percent(50.0),
                    left: Val::Percent(50.0),
                    height: Val::Px(50.0), // temp
                    flex_direction: FlexDirection::Column,
                    border: OUTLINE,
                    ..default()
                },
                BorderColor::all(Color::from(WHITE)),
                BackgroundColor(Color::from(RED)),
                ZIndex(10),
            )
        ],
    )
}

pub fn select_sim_dropdown_click(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<DropdownButton>)>,
    mut dropdown_list: Query<&mut Node, With<DropdownList>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            // Toggle dropdown visibility
            if let Ok(mut node) = dropdown_list.single_mut() {
                node.display = if node.display == Display::None {
                    Display::Flex
                } else {
                    Display::None
                }
            }
        }
    }
}
