use bevy::{prelude::*, render::camera::ScalingMode, window::WindowMode::*};
mod blackjack;
use crate::blackjack::PlayingCard;
use crate::blackjack::draw_card;
use crate::blackjack::card_to_asset_index;
use crate::blackjack::init_deck;

// Scaling to support 16:9 resolutions such as 640x360, 1280x720, 1920x1080 and 2560x1440 corresponding to const SCALE values 1.0, 2.0, 3.0, 4.0 respectively
pub const SCALE: f32 = 2.0;
pub const WINDOW_WIDTH: f32 = 640.0 * SCALE;
pub const WINDOW_HEIGHT: f32 = WINDOW_WIDTH / 16.0 * 9.0;

pub const SPRITE_SCALE: f32 = 5.0;

struct Coordinates {
    card_deal_pos_x: f32,
    card_deal_pos_z: f32,
}

#[derive(Component)]
struct Card;

struct GameTextures {
    card_sheet: Handle<TextureAtlas>,
}

struct CardPiles {
    deck: Vec<PlayingCard>,
    player_hand: Vec<PlayingCard>,
}

struct SFXPlayCard(Handle<AudioSource>);



fn main() {
    //blackjack::main();    
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
        .add_startup_system(blackjack::main)
        .add_system(card_spawner)
        .add_system(card_despawner)
        .run();
    
}

fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {

    // Set player hand deal location
    let cords = Coordinates {
        card_deal_pos_x: 0.0,
        card_deal_pos_z: 0.0,
    };
    commands.insert_resource(cords);


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

fn card_spawner(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    keyboard: Res<Input<KeyCode>>,
    mut cords: ResMut<Coordinates>,
    mut card_piles: ResMut<CardPiles>,
    audio: Res<Audio>,
    sound: Res<SFXPlayCard>,
) {
    if keyboard.just_pressed(KeyCode::Z) {
        audio.play(sound.0.clone());
        let card: PlayingCard = draw_card(&mut card_piles.deck);
        println!("Card asset index is {}", card_to_asset_index(&card));
        commands.spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(card_to_asset_index(&card)),
            texture_atlas: game_textures.card_sheet.clone(),
            transform: Transform {
            translation: Vec3::new(cords.card_deal_pos_x, -570.0, cords.card_deal_pos_z), // Near the bottom of the screen
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
            ..Default::default()
            },
            ..Default::default()
        }).insert(Card);

        cords.card_deal_pos_x += 50.0;
        cords.card_deal_pos_z += 1.0;
        card_piles.player_hand.push(card);

    }
}

fn card_despawner(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mut cords: ResMut<Coordinates>,
    mut cards: Query<(Entity, With<Card>)>,
    mut card_piles: ResMut<CardPiles>,
) {
    if keyboard.just_pressed(KeyCode::X) {
        for entity in cards.iter_mut() {
            commands.entity(entity.0).despawn();
        }
        cords.card_deal_pos_x = 0.0;
        cords.card_deal_pos_z = 1.0;
        card_piles.deck = init_deck();
        card_piles.player_hand = Vec::new();
    }
}

