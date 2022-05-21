use std::fmt;
use bevy::prelude::*;
use rand::prelude::SliceRandom;
use crate::{GameTextures, SFXPlayCard, SPRITE_SCALE};

pub const CARD_SHIFT: f32 = 50.0;

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
    dealer_hand: Vec<PlayingCard>,
}

struct Coordinates {
    player_deal_pos_x: f32,
    player_deal_pos_z: f32,
    dealer_deal_pos_x: f32,
    dealer_deal_pos_z: f32,
}

#[derive(Copy, Clone)]
enum CardSuit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
enum BlackjackState {
    Betting,
    InitialDraw,
    PlayerTurn,
    PlayerDraw,
    DealerTurn,
    DealerDraw,
    CleanUp,
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
pub struct BlackjackPlugin;

impl Plugin for BlackjackPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(setup)
        .add_state(BlackjackState::Betting)
        .add_system(card_spawner)
        .add_system(state_changer)
        .add_system_set(
            SystemSet::on_enter(BlackjackState::InitialDraw)
                .with_system(initial_draw))
        .add_system_set(
            SystemSet::on_enter(BlackjackState::PlayerDraw)
                .with_system(draw))
        .add_system_set(
            SystemSet::on_enter(BlackjackState::DealerDraw)
                .with_system(draw))
        .add_system_set(
            SystemSet::on_enter(BlackjackState::CleanUp)
                .with_system(clean_up));
    }
}


fn setup(
	mut commands: Commands,
) {
    let deck = init_deck();
    let  dealer_hand: Vec<PlayingCard> = Vec::new();
    let player_hand: Vec<PlayingCard> = Vec::new();

    let card_piles = CardPiles {
        deck: deck,
        player_hand: player_hand,
        dealer_hand: dealer_hand,
    };

    commands.insert_resource(card_piles);

    // Set player hand deal location
    let cords = Coordinates {
        player_deal_pos_x: 0.0,
        player_deal_pos_z: 1.0,
        dealer_deal_pos_x: 50.0,
        dealer_deal_pos_z: 1.0,
    };
    commands.insert_resource(cords);    

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
            player_hand.push(get_card(&mut deck));

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
            dealer_hand.push(get_card(&mut deck));
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

fn get_card(deck: &mut Vec<PlayingCard>) -> PlayingCard {
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
    keyboard: Res<Input<KeyCode>>,
    mut blackjack_state: ResMut<State<BlackjackState>>,
) {
    if blackjack_state.current() == &BlackjackState::PlayerTurn && keyboard.just_pressed(KeyCode::Z) {
        blackjack_state.set(BlackjackState::PlayerDraw).unwrap();
    } else if blackjack_state.current() == &BlackjackState::DealerTurn && keyboard.just_pressed(KeyCode::Z) {
        blackjack_state.set(BlackjackState::DealerDraw).unwrap();
    }
}

fn clean_up(
    mut commands: Commands,
    mut cords: ResMut<Coordinates>,
    mut cards: Query<(Entity, With<Card>)>,
    mut card_piles: ResMut<CardPiles>,
) {
    for entity in cards.iter_mut() {
        commands.entity(entity.0).despawn();
    }
    cords.player_deal_pos_x = 0.0;
    cords.player_deal_pos_z = 1.0;
    cords.dealer_deal_pos_x = 50.0;
    cords.dealer_deal_pos_z = 1.0;
    card_piles.deck = init_deck();
    card_piles.player_hand = Vec::new();
}

fn state_changer( // debugging fundtion. Press x to progress through game state
    keyboard: ResMut<Input<KeyCode>>,
    mut blackjack_state: ResMut<State<BlackjackState>>,
) {
    if keyboard.just_pressed(KeyCode::X) {
        match blackjack_state.current() {
            BlackjackState::Betting => blackjack_state.set(BlackjackState::InitialDraw,).unwrap(),
            BlackjackState::InitialDraw => blackjack_state.set(BlackjackState::PlayerTurn).unwrap(),
            BlackjackState::PlayerTurn => blackjack_state.set(BlackjackState::DealerTurn).unwrap(),
            BlackjackState::DealerTurn => blackjack_state.set(BlackjackState::CleanUp).unwrap(),
            BlackjackState::PlayerDraw => blackjack_state.set(BlackjackState::CleanUp).unwrap(),
            BlackjackState::DealerDraw => blackjack_state.set(BlackjackState::CleanUp).unwrap(),
            BlackjackState::CleanUp => blackjack_state.set(BlackjackState::CleanUp).unwrap(),
        }
    }
}

fn draw(
    mut commands: Commands,
    mut cords: ResMut<Coordinates>,
    mut card_piles: ResMut<CardPiles>,
    audio: Res<Audio>,
    sound: Res<SFXPlayCard>,
    game_textures: Res<GameTextures>,
    mut blackjack_state: ResMut<State<BlackjackState>>,
) {
    audio.play(sound.0.clone());
    let card: PlayingCard = get_card(&mut card_piles.deck);
    match blackjack_state.current() {
        BlackjackState::Betting => panic!("Should not call this function in this state!"),
        BlackjackState::InitialDraw => panic!("Should not call this function in this state!"),
        BlackjackState::PlayerTurn => panic!("Should not call this function in this state!"),
        BlackjackState::DealerTurn => panic!("Should not call this function in this state!"),
        BlackjackState::CleanUp => panic!("Should not call this function in this state!"),
        BlackjackState::PlayerDraw => {

            commands.spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(card_to_asset_index(&card)),
                texture_atlas: game_textures.card_sheet.clone(),
                transform: Transform {
                translation: Vec3::new(cords.player_deal_pos_x, -570.0, cords.player_deal_pos_z),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
                ..Default::default()
                },
                ..Default::default()
            }).insert(Card);
            cords.player_deal_pos_x += CARD_SHIFT;
            cords.player_deal_pos_z += 1.0;
            card_piles.player_hand.push(card);
            if card_piles.player_hand.len() + card_piles.dealer_hand.len() < 5 { blackjack_state.set(BlackjackState::InitialDraw).unwrap(); } else { blackjack_state.set(BlackjackState::PlayerTurn).unwrap(); }
        },
        BlackjackState::DealerDraw => {
            let asset_index: usize = if card_piles.dealer_hand.len() == 1 {
                52
            } else {
                card_to_asset_index(&card)
            };
            let card: PlayingCard = if asset_index != 1 {
                get_card(&mut card_piles.deck)
            } else {
                PlayingCard{ suit: CardSuit::Heart, value: 0 } // dummy card
            };


            commands.spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(asset_index),
                texture_atlas: game_textures.card_sheet.clone(),
                transform: Transform {
                translation: Vec3::new(cords.dealer_deal_pos_x, -270.0, cords.dealer_deal_pos_z),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
                ..Default::default()
                },
                ..Default::default()
            }).insert(Card);
            cords.dealer_deal_pos_x += CARD_SHIFT;
            cords.dealer_deal_pos_z += 1.0;
            card_piles.dealer_hand.push(card);
            // if dummy card was played, move coordinates so the next real card will be played on top of it
            if card_piles.dealer_hand.len() == 2 { cords.dealer_deal_pos_x -= CARD_SHIFT;}
            if card_piles.player_hand.len() + card_piles.dealer_hand.len() < 5 { blackjack_state.set(BlackjackState::InitialDraw).unwrap(); } else { blackjack_state.set(BlackjackState::DealerTurn).unwrap(); }
        },
    }
}

fn initial_draw(
    card_piles: ResMut<CardPiles>,
    mut blackjack_state: ResMut<State<BlackjackState>>,
) {
    if card_piles.player_hand.len() == 2 {
        if card_piles.dealer_hand.len() == 2 {
                blackjack_state.set(BlackjackState::PlayerTurn).unwrap();
            } else {
                blackjack_state.set(BlackjackState::DealerDraw).unwrap();
            }
        } else {
            blackjack_state.set(BlackjackState::PlayerDraw).unwrap();
        }
}