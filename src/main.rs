use bevy::{prelude::*, render::camera::ScalingMode, window::WindowMode::*};
mod blackjack;
use blackjack::BlackjackPlugin;

// Scaling to support 16:9 resolutions such as 640x360, 1280x720, 1920x1080 and 2560x1440 corresponding to const SCALE values 1.0, 2.0, 3.0, 4.0 respectively
pub const SCALE: f32 = 2.0;
pub const WINDOW_WIDTH: f32 = 640.0 * SCALE;
pub const WINDOW_HEIGHT: f32 = WINDOW_WIDTH / 16.0 * 9.0;

pub const SPRITE_SCALE: f32 = 5.0;


struct GameTextures {
    card_sheet: Handle<TextureAtlas>,
}

struct SFXPlayCard(Handle<AudioSource>);

fn main() {
    App::new()
            .insert_resource(WindowDescriptor {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            title: "Kasino".to_string(),
            present_mode: bevy::window::PresentMode::Fifo,
            resizable: false,
            mode: Windowed,
            ..Default::default()  
    })
        .add_plugins(DefaultPlugins)
        .add_startup_system_to_stage(StartupStage::PreStartup,setup)
        .add_plugin(BlackjackPlugin)
        .run();
    
}

fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {

    // load audio
    let play_card_sfx = asset_server.load("play_card01.ogg");
    commands.insert_resource(SFXPlayCard(play_card_sfx));

    // Initialize camera
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scaling_mode = ScalingMode::FixedVertical;
    camera.orthographic_projection.scale = 640.0;
    commands.spawn_bundle(camera);

	// load card_sheet.png as texture atlas
	let texture_handle = asset_server.load("card_sheet.png");
	let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32., 32.), 13, 5);
	let card_sheet = texture_atlases.add(texture_atlas);

	// add sprites and atlases to GameTextures resource
	let game_textures = GameTextures {
		card_sheet,
	};
	commands.insert_resource(game_textures);
}