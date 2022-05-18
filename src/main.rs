use bevy::prelude::*;
//mod blackjack;


// Scaling to support 16:9 resolutions such as 640x360, 1280x720, 1920x1080 and 2560x1440
// In theory is that this will work with minimal effort if all sprites and cordinates are multipied with the SCALE factor
pub const SCALE: f32 = 2.0;
pub const WINDOW_WIDTH: f32 = 640.0 * SCALE;
pub const WINDOW_HEIGHT: f32 = WINDOW_WIDTH / 16.0 * 9.0;

struct GameTextures {
    card_sheet: Handle<TextureAtlas>,
}


fn main() {
    //blackjack::main();    
    App::new()
            .insert_resource(WindowDescriptor {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            title: "Kasino".to_string(),
            present_mode: bevy::window::PresentMode::Fifo,
            resizable: true,
            ..Default::default()  
    })
        .add_plugins(DefaultPlugins)
        .add_startup_system_to_stage(StartupStage::PreStartup,setup)
        .add_startup_system(card_spawner)
        .run();
    
}

fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Initialize camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

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

fn card_spawner(
    mut commands: Commands,
    game_textures: Res<GameTextures>
) {
    commands.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(52),
        texture_atlas: game_textures.card_sheet.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, - (WINDOW_HEIGHT / 2.0) + (16.0 * SCALE), 1.0), // Near the bottom of the screen
            scale: Vec3::new(SCALE, SCALE, 1.0),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Name::new("Card"));
}