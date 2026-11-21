use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};

use crate::{resources::global::*, ui::options::popup::ChangeSimPopup};

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
                    padding: UiRect::left(Val::Px(5.0)),
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
                Node {
                    border: UiRect::all(Val::Px(2.0)),
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

pub fn options_exit_button(
    mut commands: Commands,
    mut exit_query: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
    popup_query: Query<Entity, With<ChangeSimPopup>>,
) {
    for interaction in &mut exit_query {
        if matches!(interaction, Interaction::Pressed)
            && let Ok(e) = popup_query.single()
        {
            commands.entity(e).despawn();
        }
    }
}
