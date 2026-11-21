use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};

use crate::{resources::global::*, simulations::plugin::SimulationSceneEnum};

#[derive(Component)]
pub struct DropdownButton;

#[derive(Component)]
pub struct DropdownLabel;

#[derive(Component)]
pub struct DropdownList;

#[derive(Component)]
pub struct DropdownOption(SimulationSceneEnum);

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
                        Text::new("Nothing selected..."),
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
                    top: Val::Percent(100.0),
                    width: Val::Percent(50.0),
                    left: Val::Percent(50.0),
                    flex_direction: FlexDirection::Column,
                    border: OUTLINE,
                    ..default()
                },
                BorderColor::all(Color::from(WHITE)),
                BackgroundColor(Color::from(BLACK)),
                ZIndex(10),
                Children::spawn(SpawnIter(SimulationSceneEnum::ALL.into_iter().map(row)))
            )
        ],
    )
}

pub fn row(option: SimulationSceneEnum) -> impl Bundle + use<> {
    (
        Button,
        DropdownOption(option),
        Node {
            padding: PADDING_SMALL,
            ..default()
        },
        BackgroundColor(Color::from(BLACK)),
        children![(
            Text::new(option.label()),
            TextColor(Color::from(WHITE)),
            TextFont {
                font_size: FONT_SMALL,
                ..default()
            }
        )],
    )
}

pub fn toggle_dropdown(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<DropdownButton>)>,
    mut dropdown_list: Query<&mut Node, With<DropdownList>>,
) {
    let Ok(interaction) = interaction_query.single() else {
        return;
    };
    if *interaction == Interaction::Pressed
        && let Ok(mut node) = dropdown_list.single_mut()
    {
        node.display = if node.display == Display::None {
            Display::Flex
        } else {
            Display::None
        }
    }
}

pub fn select_option(
    option_query: Query<(&Interaction, &DropdownOption), Changed<Interaction>>,
    mut list_query: Query<&mut Node, With<DropdownList>>,
    mut selected: ResMut<NextState<SimulationSceneEnum>>,
) {
    for (interaction, option) in &option_query {
        if *interaction == Interaction::Pressed {
            selected.set(option.0);
            if let Ok(mut node) = list_query.single_mut() {
                node.display = Display::None
            }
        }
    }
}

pub fn sync_dropdown_label(
    selected: Res<State<SimulationSceneEnum>>,
    mut label_query: Query<&mut Text, With<DropdownLabel>>,
) {
    for mut text in &mut label_query {
        let current_sim = selected.label();
        if text.0 != current_sim {
            text.0 = current_sim.to_string();
        }
    }
}

pub fn sync_dropdown_option_styling(
    selected: Res<State<SimulationSceneEnum>>,
    mut option_query: Query<(&DropdownOption, &mut BackgroundColor, &Children)>,
    mut text_query: Query<&mut TextColor>,
) {
    for (option, mut bg, children) in &mut option_query {
        let (bg_color, text_color) = if option.0 == *selected.get() {
            (WHITE, BLACK)
        } else {
            (BLACK, WHITE)
        };

        bg.0 = Color::from(bg_color);

        let mut texts = text_query.iter_many_mut(children.iter());
        while let Some(mut tc) = texts.fetch_next() {
            tc.0 = Color::from(text_color);
        }
    }
}

pub fn close_dropdown_click_outside(
    mouse: Res<ButtonInput<MouseButton>>,
    dropdown_interactions: Query<&Interaction, Or<(With<DropdownOption>, With<DropdownButton>)>>,
    mut list_query: Query<&mut Node, With<DropdownList>>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    if dropdown_interactions
        .iter()
        .any(|i| *i != Interaction::None)
    {
        return;
    }

    for mut node in &mut list_query {
        if node.display != Display::None {
            node.display = Display::None;
        }
    }
}
