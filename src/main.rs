mod deck;
use deck::DeckInGame;

enum PokerHand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    Straight,
    Flush,
    FullHouse,
    FourOfKind,
    StraightFlush,
    RoyalFlush,
}

pub struct Poker {
    pub players: i32,
    pub hand_size: i32,
    pub cards_on_table: i32,
    pub deck: DeckInGame,
}

impl Poker {
    fn new(players: i32, hand_size: i32, cards_on_table: i32) -> Poker {
        let deck = DeckInGame::new();

        Poker {
            players,
            hand_size,
            cards_on_table,
            deck,
        }
    }

    fn new_texas_hold_em(players: i32) -> Poker {
        let deck = DeckInGame::new();
        Poker {
            players,
            hand_size: 2,
            cards_on_table: 5,
            deck,
        }
    }
}

fn main() {
    let mut poker = Poker::new_texas_hold_em(4);
    let mut poker2 = Poker::new_texas_hold_em(4);
    poker.deck.shuffle_cards();
    let mut i: u128 = 0;
    while i < u128::MAX {
        poker2.deck.shuffle_cards();
        if i % 10000 == 0 {
            println!("{}", i);
        }
        if poker.deck.cards == poker2.deck.cards {
            panic!("decks are equa");
        }
        i += 1;
    }

    let card = poker.deck.top_card();
    let card = poker2.deck.top_card();
    // let card = deck.top_card();
    // let card = deck.top_card();
    // deck.shuffle_cards();

    // println!("{:#?}", card);
    println!("{:#?}", poker.deck.cards);
    poker.deck.shuffle_cards();
    println!("pasa");
    println!("{:#?}", poker.deck.cards_dealt);
    // println!("{:#?}", card);
}
