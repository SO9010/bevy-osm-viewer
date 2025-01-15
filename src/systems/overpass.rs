use bevy::{
    prelude::*, window::PrimaryWindow,
};
use bevy_prototype_lyon::entity::Path;

use crate::map::{MapBundle, MapFeature};

use super::bbox_system;

pub fn spawn_starting_point(mut commands: Commands,
    map_bundle: Query<&mut MapBundle>,
    shapes_query: Query<(Entity, &Path, &GlobalTransform, &MapFeature)>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    query: Query<&mut OrthographicProjection, With<Camera>>,
) 
{
    bbox_system(commands, map_bundle, &camera_query, &primary_window_query, query, shapes_query);
}

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
