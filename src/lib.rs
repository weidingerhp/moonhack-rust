use bevy_kira_audio::{AudioChannel, AudioPlugin, AudioSource};
use bevy;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{*};
use bevy::sprite::{ColorMaterial, TextureAtlas};
use lunar_lander::{LanderStart, LunarLander, LunarStartBundle};
use wasm_bindgen::prelude::*;
pub mod lunar_lander;

struct GameAssets {
    background: Handle<ColorMaterial>,
    lunar_module: Handle<TextureAtlas>,
    sound_thruster: Handle<AudioSource>,
    sound_landed: Handle<AudioSource>,
    sound_crashed: Handle<AudioSource>,
}

impl GameAssets {
    fn new(mut materials: ResMut<Assets<ColorMaterial>>, mut texture_atlases: ResMut<Assets<TextureAtlas>>, asset_server: Res<AssetServer>) -> Self { 

        let lunar_lander_handle = asset_server.load("lunar_module_map.png");
        
        Self { 
            background: materials.add(asset_server.load("background.png").into()), 
            lunar_module: texture_atlases.add(TextureAtlas::from_grid(lunar_lander_handle, Vec2::new(128., 96.), 2, 2)), 
            sound_thruster: asset_server.load("thrusters.ogg").into(), 
            sound_landed: asset_server.load("landed.ogg").into(), 
            sound_crashed: asset_server.load("problem.ogg").into(), 
        } 
    }
}

pub struct AudioChannels {
    thruster: AudioChannel,
    radio: AudioChannel,   
}

impl AudioChannels {
    fn new() -> Self {
        Self {
            thruster: AudioChannel::new("thruster".to_owned()),
            radio: AudioChannel::new("radio".to_owned()),
        }
    }
}

#[wasm_bindgen]
pub fn run() {
    let mut app = App::build();
    app.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "CoderDojo MoonHack".to_string(),
            width: 640.,
            height: 480.,
            scale_factor_override: Some(1.5),
            ..Default::default()
        })
        .insert_resource(AudioChannels::new())
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(LunarLander)
        .add_startup_system(setup.system())
        .add_startup_stage("background", SystemStage::single(spawn_background.system()));

        #[cfg(target_arch = "wasm32")]
        app.add_plugin(bevy_webgl2::WebGL2Plugin);
        
        app.run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(GameAssets::new(materials, texture_atlases, asset_server));
}

fn spawn_background(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    commands.spawn_bundle(SpriteBundle {
        material: game_assets.background.clone(), 
        transform: Transform {
            translation: Vec3::new(0., 0., 1.),
            scale: Vec3::new(0.7, 0.7, 1.), // durch Probieren rausgefunden 
            ..Default::default()
        },
        ..Default::default()
    });
    
    commands.spawn_bundle(LunarStartBundle {
        should_start: true,
    }).insert(LanderStart);
}