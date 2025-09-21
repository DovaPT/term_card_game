#![allow(dead_code)]
#![feature(random)]

use std::{collections::VecDeque, fmt::Display};

#[derive(Clone, Copy, Debug)]
enum CardSuit {
    Hearts,
    Spades,
    Diamonds,
    Clubs
}

impl Display for CardSuit{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            CardSuit::Hearts => 'H',
            CardSuit::Spades => 'S',
            CardSuit::Diamonds => 'D',
            CardSuit::Clubs => 'C',
        })
    }
}

#[derive(Clone, Copy, Debug)]
enum CardValue {
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

impl Display for CardValue {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match self {
        CardValue::Ace => "A",
        CardValue::Two => "2",
        CardValue::Three => "3",
        CardValue::Four => "4",
        CardValue::Five =>  "5",
        CardValue::Six => "6",
        CardValue::Seven => "7",
        CardValue::Eight => "8",
        CardValue::Nine => "9",
        CardValue::Ten => "10",
        CardValue::Jack => "J",
        CardValue::Queen => "Q",
        CardValue::King => "K",
    })
}
}

impl From<u8> for CardValue{
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Ace,
            1 => Self::Two,
            2 => Self::Three,
            3 => Self::Four,
            4 => Self::Five,
            5 => Self::Six,
            6 => Self::Seven,
            7 => Self::Eight,
            8 => Self::Nine,
            9 => Self::Ten,
            10 => Self::Jack,
            11 => Self::Queen,
            12 => Self::King,
            v => panic!("{} has no card value assigned to it", v)
        }
    }
}

impl From<u8> for CardSuit{
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Hearts,
            1 => Self::Spades,
            2 => Self::Diamonds,
            3 => Self::Clubs,
            s => panic!("{} has no card suit assigned to it", s)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Card {
    suit: CardSuit,
    value: CardValue,
}

impl Display for Card{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        write!(f,"{1}{0}", self.value, self.suit)
    }
}

pub struct Table {
    deck: VecDeque<Card>,
    discard: VecDeque<Card>,
}

pub struct Player {}

impl Table {
    pub fn new(amount: usize) -> Self {
        let mut deck = VecDeque::<Card>::with_capacity(52 * amount);
        let discard = VecDeque::<Card>::with_capacity(52 * amount);
        for suit in (0..4).map(|a: u8| CardSuit::from(a)) {
            for value in (0..=12).map(|a: u8| CardValue::from(a)) {
                let card = Card { suit, value };
                deck.push_back(card);
            }
        }
        let mut table = Self { deck, discard };
        table.shuffle_deck();
        table
    }

    pub fn draw(&mut self) -> Card {
        match self.deck.pop_front() {
            Some(card) => card,
            None => self.combine_decks().draw(),
        }
    }

    pub fn shuffle_deck(&mut self) -> &mut Self {
        use std::random::random;

        for i in 0..self.deck.len() {
            let loc = random::<usize>(..) % self.deck.len();
            (self.deck[i], self.deck[loc]) = (self.deck[loc], self.deck[i]);
        }
        self
    }

    pub fn combine_decks(&mut self) -> &mut Self {
        self.deck.append(&mut self.discard);
        self
    }
}
