use bevy_kira_audio::Audio;
use bevy::prelude::{*};
use crate::{AudioChannels, GameAssets};

#[derive(Bundle)]
pub struct LunarStartBundle {
    pub should_start: bool,
}

pub struct LanderStart;

pub struct LunarLanderProperties {
    pub velocity: f32,
    pub fuel: f32,
    pub touchdown: bool,
}

pub struct LunarLander;

impl Plugin for LunarLander {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(lander_start.system())
            .add_system(lander_input.system())
            .add_system(lander_run.system());
 
    }
}

fn spawn_lander(
    mut commands: Commands,
    game_assets: Res<GameAssets>
) 
{
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: game_assets.lunar_module.clone(),
        transform: Transform {
            translation: Vec3::new(0., 150., 5.),
            scale: Vec3::new(0.5, 0.5, 1.),
            ..Default::default()
        },

        ..Default::default()
    })
    .insert(LunarLander)
    .insert(LunarLanderProperties {
        velocity: 2.,
        fuel: 100.,
        touchdown: false,
    });
}

fn lander_start(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    query: Query<Entity, With<LanderStart>>
) 
{
    if let Ok(entity) = query.single() {
        commands.entity(entity).despawn();
        spawn_lander(commands, game_assets);
        println!("Spawned LunarLander");
    }

}

fn lander_run(
    audio: Res<Audio>,
    audiochannels: Res<AudioChannels>,
    game_assets: Res<GameAssets>,
    mut query: Query<(&mut LunarLanderProperties, &mut TextureAtlasSprite, &mut Transform), With<LunarLander> >    
) {
    if let Ok((mut properties, mut sprite, mut transform)) = query.single_mut() {
        if properties.touchdown {
            return;
        }

        if (transform.translation.y - properties.velocity) <= -200. {
            transform.translation.y;
            properties.touchdown = true;

            if properties.velocity > 1. {
                sprite.index = 2;
                println!("crashed");
                audio.play_in_channel(game_assets.sound_crashed.clone(), &audiochannels.radio);
            } else {
                sprite.index = 0;
                println!("Landed sucessfully");
                audio.play_in_channel(game_assets.sound_landed.clone(), &audiochannels.radio);
            }
            return;
        }

        transform.translation.y -= properties.velocity;

    }
}

fn lander_input(
    keyboard: Res<Input<KeyCode>>,
    audio: Res<Audio>,
    audiochannels: Res<AudioChannels>,
    game_assets: Res<GameAssets>,
    mut query: Query<(&mut LunarLanderProperties, &mut TextureAtlasSprite), With<LunarLander> >    
) 
{
    if let Ok((mut properties, mut sprite)) = query.single_mut() {
        if properties.touchdown {
            audio.stop_channel(&audiochannels.thruster);
            return;
        }
        
        if keyboard.pressed(KeyCode::Space) {
            sprite.index = 1;
            properties.velocity -= 0.1;
        } else {
            sprite.index = 0;
            properties.velocity += 0.03;
        }

        // just for audio
        if keyboard.just_pressed(KeyCode::Space) {
            audio.play_looped_in_channel(game_assets.sound_thruster.clone(), &audiochannels.thruster);
        }
        if keyboard.just_released(KeyCode::Space) {
        audio.stop_channel(&audiochannels.thruster);
        }

    }
}
