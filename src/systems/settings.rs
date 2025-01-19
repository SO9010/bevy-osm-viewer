use bevy::prelude::*;
use bevy_egui::{egui::{self}, EguiContexts};

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
) {
    let ctx = contexts.ctx_mut();

    occupied_screen_space.left = egui::SidePanel::left("Layers")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Layers");


            egui::ScrollArea::vertical().show(ui, |ui| {
                for (category_name, category) in &mut overpass_settings.categories {
                    ui.collapsing(category_name, |ui| {
                        // All/None toggles
                        ui.horizontal(|ui| {
                            if ui.checkbox(&mut category.all.clone(), "All").clicked() {
                                category.all = !category.all;
                                if category.disabled {
                                    category.disabled = false;
                                }
                            }
                            if ui.checkbox(&mut category.disabled.clone(), "None").clicked() {
                                // Handle the "None" toggle logic
                                category.disabled = !category.disabled;
                                if category.all {
                                    category.all = false;
                                }
                            }
                        });
        
                        // Individual toggles
                        for (item_name, state) in &mut category.items {
                            if ui.checkbox(state, item_name).clicked() {
                                category.all = false;
                                category.disabled = false;
                            }
                        }
                    });
                }
            });
            
    
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
}
