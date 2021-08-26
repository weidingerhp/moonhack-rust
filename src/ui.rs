use bevy::prelude::*;

use crate::{GameAssets, lunar_lander::spawn_lander};

pub struct StartButton;
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage("UI-Start", SystemStage::single(start_ui.system()))
           .add_system(button_interaction.system()); 
    }
}

pub fn start_ui(
    mut commands: Commands,
    game_assets: Res<GameAssets>
) {
    commands.spawn_bundle( SpriteBundle {
        material: game_assets.button.clone(),
        transform: Transform {
            translation: Vec3::new(0., 0., 10.),
            scale: Vec3::new(1., 1., 1.),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(StartButton);
}

fn button_interaction(
    mut commands: Commands,
    mouse_button_input_events: Res<Input<MouseButton>>,
    game_assets: Res<GameAssets>,
    mut interaction_query: Query<Entity, With<StartButton>>
) {
    if let Ok(entity) = interaction_query.single_mut() {
        if mouse_button_input_events.just_pressed(MouseButton::Left) {
            commands.entity(entity).despawn();
            spawn_lander(commands, game_assets);
        }
    }
}