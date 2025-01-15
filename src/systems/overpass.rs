use bevy::{
    prelude::*,
};


// Should be stored in a resource
pub fn spawn_settings(mut commands: Commands) {
    commands
    .spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::SpaceBetween,
        ..default()
    }).with_children(|parent| {
        // left vertical fill (border)
        parent
            .spawn((
                Node {
                    width: Val::Px(200.),
                    border: UiRect::all(Val::Px(2.)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.65, 0.65, 0.65)),
            ))
            .with_children(|parent| {
                // left vertical fill (content)
                parent
                    .spawn((
                    //    SettingsOverlay,
                        Node {
                            width: Val::Percent(100.),
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(5.)),
                            row_gap: Val::Px(5.),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    ));
            });
        });
}
