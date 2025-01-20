use bevy::prelude::*;
use bevy_egui::{egui::{self, color_picker::{color_edit_button_rgb, color_edit_button_rgba, color_edit_button_srgba, color_picker_color32}, Checkbox, Color32, RichText}, EguiContexts};
use bevy_prototype_lyon::entity::Path;
use crate::systems::settings::egui::color_picker::Alpha::Opaque;

use crate::map::MapFeature;

use super::{overpass_types::SettingsOverlay, CameraSettings};


pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ui_example_system)
            .init_resource::<OccupiedScreenSpace>()
            .insert_resource(SettingsOverlay::new())
            .insert_resource(CameraSettings { scale: 1.0 });
    }
}


#[derive(Default, Resource)]
pub struct OccupiedScreenSpace {
    pub left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

fn ui_example_system(
    mut contexts: EguiContexts,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
    mut overpass_settings: ResMut<SettingsOverlay>,
    shapes_query: Query<(Entity, &Path, &GlobalTransform, &MapFeature)>,
    mut commands: Commands
) {
    let ctx = contexts.ctx_mut();

    occupied_screen_space.left = egui::SidePanel::left("Layers")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Layers");

            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut color = Color32::from_rgb(221, 221, 221);
                
                for (category_name, category) in &mut overpass_settings.categories {
                    if category.disabled {
                        color = Color32::from_rgb(135, 135, 135);
                    } else {
                        color = Color32::from_rgb(221, 221, 221);
                    }
                    ui.collapsing(RichText::new(category_name).color(color), |ui| {
                        ui.horizontal(|ui| {
                            if ui.checkbox(&mut category.all.clone(), RichText::new("All").color(color)).clicked() {
                                if category.all {
                                    category.all = false;
                                } else {
                                    category.all = true;
                                    category.set_children(true);
                                }
                                if category.none {
                                    category.none = false;
                                }
                            }
                            if ui.checkbox(&mut category.none.clone(), RichText::new("None").color(color)).clicked() {
                                if category.none {
                                    category.none = false;
                                } else {
                                    category.none = true;
                                    category.set_children(false);
                                }
                                if category.all {
                                    category.all = false;
                                }
                            }
                        });
        
                        // Individual toggles
                        for (item_name, (state, clr)) in &mut category.items {
                            ui.horizontal(|ui| {
                                if ui.checkbox(state , RichText::new(item_name).color(color)).clicked() {
                                    category.all = false;
                                    category.none = false;
                                }
                                color_edit_button_srgba(ui, clr, Opaque)
                            });
                        }
                    });
                }
                if ui.button("Clear Map").on_hover_text("Despawns the data which makes up this map").clicked() {
                    for (entity, _, _, _) in shapes_query.iter() {
                        commands.entity(entity).despawn_recursive(); // Use despawn_recursive instead of despawn
                    } 
                }
            });
            
    
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
}