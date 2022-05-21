use std::{fmt};
use bevy::prelude::*;
use rand::prelude::SliceRandom;
use crate::{GameTextures, SFXPlayCard, SPRITE_SCALE};

pub struct BlackjackPlugin;

impl Plugin for BlackjackPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(setup)
        .add_system(card_spawner)
        .add_system(card_despawner);
    }
}

#[derive(Copy, Clone)]
pub struct PlayingCard {
    suit: CardSuit,
    value: u8,
}

#[derive(Component)]
struct Card;
struct CardPiles {
    deck: Vec<PlayingCard>,
    player_hand: Vec<PlayingCard>,
}

struct Coordinates {
    card_deal_pos_x: f32,
    card_deal_pos_z: f32,
}

#[derive(Copy, Clone)]
enum CardSuit {
    Heart,
    Diamond,
    Spade,
    Club,
}



//to_string for CardSuit
impl fmt::Display for CardSuit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CardSuit::Heart => write!(f, "Heart"),
            CardSuit::Diamond => write!(f, "Diamond"),
            CardSuit::Spade => write!(f, "Spade"),
            CardSuit::Club => write!(f, "Club"),
        }
    }
}


fn setup(
	mut commands: Commands,
) {
    let deck = init_deck();
    //let mut dealer_hand: Vec<PlayingCard> = Vec::new();
    let player_hand: Vec<PlayingCard> = Vec::new();

    let card_piles = CardPiles {
        deck: deck,
        player_hand: player_hand,
    };

    commands.insert_resource(card_piles);

    // Set player hand deal location
    let cords = Coordinates {
        card_deal_pos_x: 0.0,
        card_deal_pos_z: 0.0,
    };
    commands.insert_resource(cords);    

    //Initial card draw
    //player_hand.push(draw_card(&mut deck));
    //dealer_hand.push(draw_card(&mut deck));
    //player_hand.push(draw_card(&mut deck));

    /*loop { //Player's Turn
        println!("Your hand ({})", hand_value(&player_hand));
        describe_hand(&player_hand);
        println!("Dealer's hand ({}) ", hand_value(&dealer_hand));
        describe_hand(&dealer_hand);

        if player_hand.len() == 2 && hand_value(&player_hand) == 21   {
            println!("Player Blackjack!");
            break;
        }

        if hand_value(&player_hand) > 21 {
            println!("Player bust!");
            break;
        }

        println!("hit or stand?");
        
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "hit" {
            player_hand.push(draw_card(&mut deck));

        } else if input.trim() == "stand" {
            println!("You stand");
            break;
        } else {
            println!("Invalid input!");
        }
        println!("---------------------------------");
    }

    println!("---------------------------------");

    loop { //Dealer's Turn
        if hand_value(&player_hand) > 21 { break }; //Skip Dealer's Turn if Player busted
        println!("Your hand ({})", hand_value(&player_hand));
        describe_hand(&player_hand);
        println!("Dealer's hand ({}) ", hand_value(&dealer_hand));
        describe_hand(&dealer_hand);

        if player_hand.len() == 2 && hand_value(&player_hand) == 21   {
            println!("Dealer Blackjack!");
            break;
        }

        if hand_value(&dealer_hand) > 21 {
            println!("Dealer bust!");
            break;
        } else if hand_value(&dealer_hand) >= 17 {
            println!("Dealer stays");
            break;
        } else {
            println!("Dealer draws a card");
            dealer_hand.push(draw_card(&mut deck));
        }
        println!("---------------------------------");
    }

    //Winner
    if hand_value(&player_hand) > 21 {
        println!("You lose!"); //Player bust
    } else if hand_value(&dealer_hand) > 21 {
        println!("You win!"); //Dealer bust
    } else if hand_value(&player_hand) == 21 && player_hand.len() == 2 {
        if hand_value(&dealer_hand) == 21 && dealer_hand.len() == 2 {
            println!("Draw!"); //Dealer & Player blacjack
        } else {
            println!("You win!"); //Player blacjack, x1.5 payout
        }
    } else if hand_value(&dealer_hand) == 21 && dealer_hand.len() == 2 {
        println!("You lose!"); //Dealer blackjack
    } else if hand_value(&player_hand) == hand_value(&dealer_hand) {
        println!("Draw!"); //Dealer and Player have same hand value
    } else if hand_value(&player_hand) > hand_value(&dealer_hand) {
        println!("You win"); //Player hand is better than Dealer's
    } else {
        println!("You lose!") //Player hand is worse than Dealer's
    } */

}

pub fn init_deck() -> Vec<PlayingCard> {
    let mut deck = Vec::new();
    let mut current_suit = CardSuit::Heart;
    for i in 1..53 {

        if i % 13 == 0 {
            deck.push(PlayingCard { suit: current_suit, value: 13});    
            match current_suit {
                CardSuit::Heart => current_suit = CardSuit::Diamond,
                CardSuit::Diamond => current_suit = CardSuit::Spade,
                CardSuit::Spade => current_suit = CardSuit::Club,
                CardSuit::Club => current_suit = CardSuit::Heart,
            }
        }
        else {
            deck.push(PlayingCard { suit: current_suit, value: i % 13});        
        }
    }

    deck.shuffle(&mut rand::thread_rng());

    return deck;
}

pub fn draw_card(deck: &mut Vec<PlayingCard>) -> PlayingCard {
    let i = (rand::random::<f32>() * deck.len() as f32).floor() as usize;
    let card = deck.remove(i);
    return card;
}

/*fn describe_hand(hand: &Vec<PlayingCard>) {
    for i in 0..hand.len() {
        println!("There is a {} of {}s", hand[i].value, hand[i].suit);
    }
} */

// translate PlayingCard struct to the corresponding index in card_sheet.png
pub fn card_to_asset_index(card: &PlayingCard) -> usize {
    match card.suit {
        CardSuit::Heart => return (card.value - 1) as usize,
        CardSuit::Diamond => return (card.value + 12 ) as usize,
        CardSuit::Spade => return (card.value + 25) as usize,
        CardSuit::Club => return (card.value + 38) as usize,
    }
}

/*fn hand_value(hand: &Vec<PlayingCard>) -> u8 {
    let mut hand_value: u8 = 0;
    let mut aces: u8 = 0;
    for i in 0..hand.len() {
        hand_value += cmp::min(hand[i].value, 10);
        if hand[i].value == 1 {
             aces += 1;
        }
    }
    while aces > 0 && hand_value != 21 {
        if hand_value + 10 <= 21 {
            hand_value += 10;
        }
        aces -= 1;
    }
    return hand_value;
} */

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