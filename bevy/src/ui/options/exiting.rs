use bevy::prelude::*;

use crate::ui::options::{
    popup::{OptionsPopup, create_sim_popup},
    title::ExitButton,
};

pub fn exit_from_button(
    mut commands: Commands,
    mut exit_query: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
    popup_query: Query<Entity, With<OptionsPopup>>,
) {
    for interaction in &mut exit_query {
        if matches!(interaction, Interaction::Pressed)
            && let Ok(e) = popup_query.single()
        {
            commands.entity(e).despawn();
        }
    }
}

pub fn exit_from_escape(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    popup_query: Query<Entity, With<OptionsPopup>>,
) {
    if input.pressed(KeyCode::Escape) {
        if let Ok(e) = popup_query.single() {
            commands.entity(e).despawn();
        } else {
            commands.spawn(create_sim_popup());
        }
    }
}
