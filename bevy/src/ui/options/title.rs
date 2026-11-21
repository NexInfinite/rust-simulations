use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};

use crate::resources::global::*;

#[derive(Component)]
pub struct ExitButton;

pub fn options_create_title() -> impl Bundle + use<> {
    (
        Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            (
                Node {
                    justify_self: JustifySelf::Center,
                    ..default()
                },
                Text::new("Configurable Options"),
                TextFont {
                    font_size: FONT_LARGE,
                    ..default()
                },
            ),
            (
                Button,
                Node {
                    border: OUTLINE,
                    padding: PADDING_LARGE,
                    height: Val::Auto,
                    width: Val::Auto,
                    align_self: AlignSelf::Start,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ExitButton,
                BorderColor::all(Color::from(WHITE)),
                BackgroundColor(Color::from(BLACK)),
                Text::new("Exit"),
                TextFont {
                    font_size: FONT_LARGE,
                    ..default()
                },
            )
        ],
    )
}
