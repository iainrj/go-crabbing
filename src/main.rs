#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.suit)
    }
}

impl Card {
    fn new(rank: Rank, suit: Suit) -> Self {
        Card { rank, suit }
    }
}

struct Deck {
    cards: Vec<Card>,
}

use core::fmt;

use rand::prelude::SliceRandom;
use rand::thread_rng;

impl Deck {
    fn new() -> Self {
        let mut cards = Vec::new();

        for suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for rank in &[
                Rank::Ace,
                Rank::Two,
                Rank::Three,
                Rank::Four,
                Rank::Five,
                Rank::Six,
                Rank::Seven,
                Rank::Eight,
                Rank::Nine,
                Rank::Ten,
                Rank::Jack,
                Rank::Queen,
                Rank::King,
            ] {
                cards.push(Card::new(*rank, *suit));
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}
fn main() {
    println!("Let's go crabbing!");
    let mut deck = Deck::new();

    deck.shuffle();

    if let Some(card) = deck.draw() {
        println!("Drew card: {:?} of {:?}", card.rank, card.suit)
    }
}
