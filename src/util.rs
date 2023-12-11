use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Ord, PartialOrd)]
pub enum CardKind {
    Jack,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Ord, PartialOrd)]
pub enum HandType {
    Highcard,
    Pair,
    TwoPair,
    TreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Debug, PartialEq, Eq, Ord)]
pub struct Hand {
    cards: Vec<CardKind>,
    hand_type: HandType,
    pub bid: i32,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.cmp(&other.hand_type) {
            std::cmp::Ordering::Equal => Some(
                self.cards
                    .iter()
                    .zip(other.cards.iter())
                    .find_map(|(card, other)| match card.cmp(other) {
                        std::cmp::Ordering::Equal => None,
                        order => Some(order),
                    })
                    .unwrap(),
            ),
            order => Some(order),
        }
    }
}

impl Hand {
    pub fn new(cards: Vec<CardKind>, number: i32) -> Self {
        let hand_type = rank_hand(&cards);
        Hand {
            cards,
            hand_type,
            bid: number,
        }
    }
}

pub fn rank_hand(cards: &[CardKind]) -> HandType {
    let mut hand = HashMap::new();

    for card in cards {
        *hand.entry(card).or_default() += 1;
    }
    let mut temp_value = 0;

    hand.iter_mut()
        .filter(|(&k, _)| *k == CardKind::Jack)
        .for_each(|(_, v)| {
            temp_value = *v;
            *v = 0
        });

    if let Some(next_highest_card) = hand.iter().max_by_key(|&(_, value)| value) {
        *hand.entry(next_highest_card.0).or_default() += temp_value;
    }

    if hand.values().any(|v| *v == 5) {
        HandType::FiveOfKind
    } else if hand.values().any(|v| *v == 4) {
        HandType::FourOfKind
    } else if hand.values().any(|v| *v == 3) {
        if hand.values().any(|v| *v == 2) {
            HandType::FullHouse
        } else {
            HandType::TreeOfKind
        }
    } else if hand.values().filter(|&v| *v == 2).count() == 2 {
        HandType::TwoPair
    } else if hand.values().filter(|&v| *v == 2).count() == 1 {
        HandType::Pair
    } else {
        HandType::Highcard
    }
}

pub fn match_card(card: char) -> CardKind {
    match card {
        '2' => CardKind::Two,
        '3' => CardKind::Three,
        '4' => CardKind::Four,
        '5' => CardKind::Five,
        '6' => CardKind::Six,
        '7' => CardKind::Seven,
        '8' => CardKind::Eight,
        '9' => CardKind::Nine,
        'T' => CardKind::Ten,
        'J' => CardKind::Jack,
        'Q' => CardKind::Queen,
        'K' => CardKind::King,
        'A' => CardKind::Ace,
        _ => panic!("Invalid card"),
    }
}
