use std::{fmt};
use bevy::prelude::*;
//use rand::{thread_rng, Rng};
use crate::CardPiles;

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

#[derive(Copy, Clone)]
pub struct PlayingCard {
    suit: CardSuit,
    value: u8,
}

pub fn main(
	mut commands: Commands,
) {
    let mut deck = init_deck();
    //let mut dealer_hand: Vec<PlayingCard> = Vec::new();
    let mut player_hand: Vec<PlayingCard> = Vec::new();

    let card_piles = CardPiles {
        deck: deck,
        player_hand: player_hand,
    };

    commands.insert_resource(card_piles);

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
    println!("Initializing deck...");
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
    println!("Deck initialized! Deck has {} cards!", deck.len());
    return deck;
}

pub fn draw_card(deck: &mut Vec<PlayingCard>) -> PlayingCard {
    println!("Drawing a card...");
    let i = (rand::random::<f32>() * deck.len() as f32).floor() as usize;
    //let i = rand::thread_rng().gen_range(0..deck.len());
    let card = deck.remove(i);
    println!("Drew a a {} of {}s", card.value, card.suit);
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