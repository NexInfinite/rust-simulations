use bevy::prelude::*;

#[derive(Component)]
pub struct TimeLabel;

pub fn create_time_label() -> impl Bundle + use<> {
    (
        Node { ..default() },
        children![(
            Text::new("Time: 0.00s"),
            TextFont {
                font_size: FONT_SMALL,
                ..default()
            },
            TextColor(Color::from(WHITE)),
        )],
        TimeLabel,
    )
}

pub fn sync_label_text(
    mut text_query: Query<&mut Text>,
    mut button_query: Query<&Children, With<TimeLabel>>,
    simulation_timing: Res<SimulationTiming>,
) {
    if !simulation_timing.is_changed() {
        return;
    }

    for children in &mut button_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        **text = format!("Time: {:.2}s", simulation_timing.time)
    }
}
