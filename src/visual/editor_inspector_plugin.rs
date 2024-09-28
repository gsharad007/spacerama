use bevy::prelude::*;
#[cfg(feature = "inspector")]
use bevy_debug_grid::DebugGridPlugin;
#[cfg(feature = "inspector")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Debug)]
pub struct EditorInspectorPlugin;

impl Plugin for EditorInspectorPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        {
            #[cfg(feature = "inspector")]
            {
                _ = app
                    .add_plugins(DebugGridPlugin::with_floor_grid())
                    .add_plugins(WorldInspectorPlugin::new());
                // .add_plugins(EguiPlugin)
                // .add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin) // adds default options and `InspectorEguiImpl`s
                // .add_systems(FixedUpdate, inspector_ui)
            }
        }
    }
}

// fn inspector_ui(world: &mut World) {
//     let Ok(egui_context) = world
//         .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
//         .get_single(world)
//     else {
//         return;
//     };
//     let mut egui_context = egui_context.clone();

//     _ = egui::Window::new("UI").show(egui_context.get_mut(), |ui| {
//         _ = egui::ScrollArea::vertical().show(ui, |ui| {
//             // equivalent to `WorldInspectorPlugin`
//             bevy_inspector::ui_for_world(world, ui);

//             _ = egui::CollapsingHeader::new("Materials").show(ui, |ui| {
//                 bevy_inspector::ui_for_assets::<StandardMaterial>(world, ui);
//             });

//             _ = ui.heading("Entities");
//             bevy_inspector::ui_for_world_entities(world, ui);
//         });
//     });
// }
