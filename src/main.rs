use bevy::{prelude::*, render::camera::ScalingMode, window::WindowMode::*};
mod blackjack;
use blackjack::BlackjackPlugin;

// debug
mod debug;
use debug::DebugPlugin;

// Scaling to support 16:9 resolutions such as 640x360, 1280x720, 1920x1080 and 2560x1440 corresponding to const SCALE values 1.0, 2.0, 3.0, 4.0 respectively - Needs rework after bevy 0.8
pub const SCALE: f32 = 2.0;
pub const WINDOW_WIDTH: f32 = 640.0 * SCALE;
pub const WINDOW_HEIGHT: f32 = WINDOW_WIDTH / 16.0 * 9.0;

pub const SPRITE_SCALE: f32 = 5.0;

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
        .add_startup_system_to_stage(StartupStage::PreStartup,setup_system)
        .add_plugin(BlackjackPlugin)
        .add_plugin(DebugPlugin) // debug
        .add_system(update_ui_system)
        .run();
    
}


struct GameTextures {
    card_sheet: Handle<TextureAtlas>,
}

struct SFXPlayCard(Handle<AudioSource>);

#[derive(Component)]
pub struct UiPlayerMoney;

pub struct Player {
    money: f32,
    bet: f32,
}

fn setup_system(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {

    // load audio
    let play_card_sfx = asset_server.load("play_card01.ogg");
    commands.insert_resource(SFXPlayCard(play_card_sfx));

    // Initialize camera
    let mut camera = Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(WINDOW_WIDTH),
            //scaling_mode: ScalingMode::WindowSize
            //scale: 640.0,
            ..default()
        },
        ..default()
    };
    //camera.orthographic_projection.scaling_mode = ScalingMode::FixedVertical;
    //camera.orthographic_projection.scale = 640.0;
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

    // setup Player values
    let player = Player {
        money: 100.0,
        bet: 1.0,
    };
    commands.insert_resource(player); 

    commands
    .spawn_bundle(Text2dBundle {
        transform: Transform {
            translation: Vec3::new(-1100.0, 600.0, 100.0),
            ..default()
        },
        text: Text::from_section("Money: ???", TextStyle {
                font: asset_server.load("retro_gaming.ttf"),
                font_size: 50.0,
                color: Color::WHITE,
            }),
        ..default()
    }).insert(UiPlayerMoney);   

}

pub fn update_ui_system(
    player: ResMut<Player>,
    mut query: Query<&mut Text, With<UiPlayerMoney>>,
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Money: {} Bet {}", player.money, player.bet);
    }
}
