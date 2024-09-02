
use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        //List of suits
        let suits = ["Hearts" , "Spades" , "Diamonds", "Clubs"];
        //List of values
        let values = ["Ace" , "Two" , "Three" ,"Four" , "Five" , "Six" , "Seven" , "Eight"];
        // List of cards
        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }
        Deck { cards }
    }

    fn shuffle(&mut self) {

        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
        
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String>{
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();    

    //deck.shuffle();

    //Probably need to add error handling!!!
    let cards = deck.deal(3);

    
    println!("Here is your hand: {:#?}" , cards);
    println!("Here is your deck: {:#?}" , deck);
}
