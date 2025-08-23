#![allow(dead_code)]
#![feature(random)]

use std::{collections::VecDeque, fmt::Display};

#[derive(Clone, Copy, Debug)]
enum CardSuit {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}

impl Display for CardSuit{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            CardSuit::Hearts => '\u{2665}',
            CardSuit::Spades => '\u{2660}',
            CardSuit::Diamonds => '\u{2666}',
            CardSuit::Clubs => '\u{2663}',
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
        for suit in (0..4).map(|a: u8| unsafe { std::mem::transmute(a) }) {
            for value in (0..=12).map(|a: u8| unsafe { std::mem::transmute(a) }) {
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
