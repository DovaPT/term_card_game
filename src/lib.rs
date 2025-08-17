#![allow(dead_code)]

#[derive(Clone, Copy, Debug)]
enum CardSuit {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}

#[derive(Clone, Copy, Debug)]
enum CardValue {
    One,
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
    Ace,
}

#[derive(Clone, Copy, Debug)]
pub struct Card {
    suit: CardSuit,
    value: CardValue,
}

pub struct Table {
    deck: Vec<Card>,
    discard: Vec<Card>,
}

impl Table {
    pub fn new(amount: usize) -> Self {
        let mut deck = Vec::<Card>::with_capacity(52 * amount);
        let discard = Vec::<Card>::with_capacity(52 * amount);
        for suit in (0..4).map(|a: u8| unsafe { std::mem::transmute(a) }) {
            for value in (0..14).map(|a: u8| unsafe { std::mem::transmute(a) }) {
                let card = Card { suit, value };
                deck.push(card);
            }
        }
        Self { deck, discard }
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.deck.pop()
    }

    pub fn shuffle(&self) -> () {
        
    }
}
