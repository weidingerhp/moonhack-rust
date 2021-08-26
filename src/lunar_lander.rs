use bevy_kira_audio::Audio;
use bevy::prelude::{*};
use crate::{AudioChannels, GameAssets, ui::start_ui};

pub struct LanderStart;

pub struct Explosion;

pub struct LunarLanderProperties {
    pub velocity: f32,
    pub fuel: f32,
    pub touchdown: bool,
    pub thrusting: bool,
}

pub struct LunarLander;

impl Plugin for LunarLander {
    fn build(&self, app: &mut AppBuilder) {
        app
            //.add_startup_stage("spawn_lander", SystemStage::single(spawn_lander.system()))
            .add_system(lander_input.system())
            .add_system(lander_run.system())
            .add_system(explode_lander.system());
    }
}

pub fn spawn_lander(
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
        thrusting: false,
    });
}

fn remove_old_landers_if_needed(
    commands: &mut Commands,
    query: &mut Query<(&mut LunarLanderProperties, &mut TextureAtlasSprite, &mut Transform, Entity), With<LunarLander> >    
) -> bool {
    let mut landers = 0;
    // check if there are two landers and remove all the landed ones ....
    query.iter_mut().for_each(|(_, _, _, _)| {
        landers += 1;
    });

    if landers > 1 {
        query.iter_mut().for_each(|(properties, _, _, entity)| {
            if properties.touchdown {
                commands.entity(entity).despawn();
            }
        });
        return true;
    }

    return false;
}

fn lander_run(
    mut commands: Commands,
    audio: Res<Audio>,
    audiochannels: Res<AudioChannels>,
    game_assets: Res<GameAssets>,
    mut query: Query<(&mut LunarLanderProperties, &mut TextureAtlasSprite, &mut Transform, Entity), With<LunarLander> >    
) {
    if remove_old_landers_if_needed(&mut commands, &mut query) {
        return;
    }

    if let Ok((mut properties, mut sprite, mut transform, _)) = query.single_mut() {
        if properties.touchdown {
            return;
        }

        if (transform.translation.y - properties.velocity) <= -200. {
            transform.translation.y;
            properties.touchdown = true;

            if properties.velocity > 1. {
                sprite.index = 2;

                let mut explosion_translation = transform.translation.clone();
                explosion_translation.z += 1.;

                audio.play_in_channel(game_assets.sound_crashed.clone(), &audiochannels.radio);
                commands.spawn_bundle( SpriteSheetBundle {
                    texture_atlas: game_assets.explosion.clone(),
                    transform: Transform {
                        translation: explosion_translation,
                        scale: Vec3::new(1., 1., 1.),
                        ..Default::default()
                    },
                ..Default::default()
                })
                .insert(Explosion)
                .insert(Timer::from_seconds(0.1, true));

            } else {
                sprite.index = 0;
                audio.play_in_channel(game_assets.sound_landed.clone(), &audiochannels.radio);
            }

            start_ui(commands, game_assets);
            return;
        }

        transform.translation.y -= properties.velocity;

    }
}

fn lander_input(
    keyboard: Res<Input<KeyCode>>,
    mouse_button_input_events: Res<Input<MouseButton>>,
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
        
        if (keyboard.pressed(KeyCode::Space) || mouse_button_input_events.pressed(MouseButton::Left)) && (properties.fuel > 0.) {
            if !properties.thrusting {
                sprite.index = 1;
                audio.play_looped_in_channel(game_assets.sound_thruster.clone(), &audiochannels.thruster);
            }
            properties.velocity -= 0.1;
            properties.thrusting = true;
        } else {
            if properties.thrusting {
                sprite.index = 0;
                audio.stop_channel(&audiochannels.thruster);
            }
            properties.velocity += 0.03;
            properties.thrusting = false;
        }
    }
}

fn explode_lander(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TextureAtlasSprite, &mut Timer), With<Explosion>>
) {

    for (entity, mut sprite, mut timer) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            if sprite.index >= 9 {
                commands.entity(entity).despawn();
            } else {
                sprite.index += 1;
            }
        }
    }
}


