extern crate rand;

use rand::Rng;

#[derive(Debug, PartialEq, Eq)]

pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
        Self::translate(rng.gen_range(1, 4))
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => unreachable!()
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        Self::translate(rng.gen_range(1, 13))
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => unreachable!()
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    card.rank == Rank::Ace && card.suit == Suit::Spade
}