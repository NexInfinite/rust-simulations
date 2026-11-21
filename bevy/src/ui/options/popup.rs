use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};

use crate::{
    resources::global::*,
    ui::options::{
        exiting::{exit_from_button, exit_from_escape},
        select_sim::{
            close_dropdown_click_outside, create_select_sim_dropdown, select_option,
            sync_dropdown_label, sync_dropdown_option_styling, toggle_dropdown,
        },
        title::options_create_title,
    },
};

pub struct UiOptionsPopup;

#[derive(Component)]
pub struct OptionsPopup;

#[derive(Component)]
pub struct OptionsPopupPannel;

impl Plugin for UiOptionsPopup {
    fn build(&self, app: &mut App) {
        app.add_observer(on_popup_spawned);
        app.add_observer(on_panel_spawned);
        app.add_observer(on_popup_despawned);

        app.add_systems(
            Update,
            (
                close_dropdown_click_outside,
                sync_dropdown_option_styling,
                sync_dropdown_label,
                toggle_dropdown,
                select_option,
            )
                .run_if(change_sim_popup_in_focus),
        );
        app.add_systems(
            Update,
            (exit_from_escape, exit_from_button).run_if(change_sim_popup_in_focus),
        );
    }
}

fn on_popup_spawned(
    add: On<Add, OptionsPopup>,
    mut global_resource: ResMut<GlobalResource>,
    mut commands: Commands,
) {
    global_resource.focus = crate::resources::global::FocusEnum::ChangeSim;

    // Allow clicking off the popup to close
    let root = add.entity;
    commands
        .entity(root)
        .observe(move |_: On<Pointer<Click>>, mut commands: Commands| {
            commands.entity(root).despawn();
        });
}

fn on_panel_spawned(add: On<Add, OptionsPopupPannel>, mut commands: Commands) {
    // Clicks inside the panel, which is smaller than the popup
    // Popup = entire screen
    // Pannel = a part of the popup which is what the user thinks of as a "popup", ik bad naming
    commands
        .entity(add.entity)
        .observe(|mut click: On<Pointer<Click>>| {
            click.propagate(false);
        });
}

fn on_popup_despawned(_: On<Remove, OptionsPopup>, mut global_resource: ResMut<GlobalResource>) {
    global_resource.focus = crate::resources::global::FocusEnum::Main;
}

pub fn create_sim_popup() -> impl Bundle + use<> {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        OptionsPopup,
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
        GlobalZIndex(100),
        children![(
            OptionsPopupPannel,
            Interaction::default(),
            Node {
                width: Val::Percent(70.0),
                border: OUTLINE,
                padding: UiRect {
                    left: Val::Px(15.0),
                    right: Val::Px(10.0),
                    top: Val::Px(10.0),
                    bottom: Val::Px(10.0)
                },
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                ..default()
            },
            BorderColor::all(Color::from(WHITE)),
            BackgroundColor(Color::from(BLACK)),
            children![options_create_title(), create_select_sim_dropdown()],
        )],
    )
}
