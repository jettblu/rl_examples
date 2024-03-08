use rand::Rng;

use crate::environment::Environment;

pub struct Blackjack {
    pub dealer: Dealer,
    pub player: Player,
    pub is_player_turn: bool,
    pub step_count: usize,
}

pub struct Dealer {
    hand: Vec<Card>,
    sum: i32,
}

pub struct Player {
    hand: Vec<Card>,
    usable_ace: bool,
    sum: i32,
}

pub struct Card {
    value: i32,
    suit: String,
}

pub trait BlackJackPlayer {
    fn draw_card(&mut self);
    fn get_sum(&self) -> i32;
    fn did_bust(&self) -> bool {
        self.get_sum() > 21
    }
}

impl Blackjack {
    pub fn new() -> Blackjack {
        Blackjack {
            dealer: Dealer::new(),
            player: Player::new(),
            is_player_turn: true,
            step_count: 0,
        }
    }
}

impl Environment for Blackjack {
    fn reset(&mut self) {
        self.dealer = Dealer::new();
        self.player = Player::new();
        // draw two cards for player and dealer
        self.player.draw_card();
        self.dealer.draw_card();
        self.player.draw_card();
        self.dealer.draw_card();
        println!("Starting Player sum: {}", self.player.get_sum());
        println!("Starting Dealer sum: {}", self.dealer.get_sum());
        self.is_player_turn = true;
    }

    fn step(&mut self, action: usize) -> f64 {
        println!("Action: {}", action);

        if self.is_player_turn {
            match action {
                // hit case
                0 => self.player.draw_card(),
                // stay case
                1 => {
                    self.is_player_turn = false;
                }
                _ => panic!("Invalid action"),
            }
        } else {
            println!("Dealer's turn");
            // play fixed dealer strategy if not player's turn
            while self.dealer.get_sum() < 17 {
                self.dealer.draw_card();
            }
        }
        // return reward
        if self.is_terminal() {
            if self.player.did_bust() {
                return -1.0;
            } else if self.dealer.did_bust() {
                return 1.0;
            } else if self.player.get_sum() > self.dealer.get_sum() {
                return 1.0;
            } else if self.player.get_sum() < self.dealer.get_sum() {
                return -1.0;
            } else {
                return 0.0;
            }
        }
        0.0
    }

    fn get_state(&self) -> usize {
        let player_sum = self.player.get_sum();
        let dealer_sum = self.dealer.get_sum();
        let usable_ace = self.player.usable_ace;
        // here we are using a simple encoding of the state
        let state = (player_sum - 12) * 9 * 2 + (dealer_sum - 2) * 2 + (usable_ace as i32);
        state as usize
    }

    fn get_actions(&self) -> Vec<usize> {
        vec![0, 1]
    }

    fn is_terminal(&self) -> bool {
        println!("Player sum: {}", self.player.get_sum());
        println!("Dealer sum: {}", self.dealer.get_sum());
        self.player.did_bust() ||
            self.dealer.did_bust() ||
            (!self.is_player_turn && self.dealer.get_sum() >= 17)
    }

    fn get_number_of_possible_actions(&self) -> usize {
        2
    }

    fn get_number_of_possible_states(&self) -> usize {
        9 * 10 * 2
    }

    fn get_total_number_of_actions_taken(&self) -> usize {
        self.step_count
    }
}

impl Dealer {
    pub fn new() -> Dealer {
        Dealer {
            hand: Vec::new(),
            sum: 0,
        }
    }
}

impl Player {
    pub fn new() -> Player {
        Player {
            hand: Vec::new(),
            usable_ace: false,
            sum: 0,
        }
    }
}

impl BlackJackPlayer for Dealer {
    fn draw_card(&mut self) {
        // draw card from deck
        let mut card = random_card();
        // if card is an ace and the sum is less than or equal to 10, add 11 to the sum
        if card.value == 1 && self.sum <= 10 {
            card.value = 11;
        }
        self.sum += card.value;
        self.hand.push(card);
    }

    fn get_sum(&self) -> i32 {
        self.sum
    }
}

impl BlackJackPlayer for Player {
    fn draw_card(&mut self) {
        // draw card from deck
        let mut card = random_card();
        // if card is an ace and the sum is less than or equal to 10, add 11 to the sum
        if card.value == 1 && self.sum <= 10 {
            card.value = 11;
            self.usable_ace = true;
        }
        self.sum += card.value;
        self.hand.push(card);
    }

    fn get_sum(&self) -> i32 {
        self.sum
    }
}

fn random_card() -> Card {
    let mut rng = rand::thread_rng();
    let value = rng.gen_range(1..11);
    let suit = String::from("hearts");
    Card {
        value,
        suit,
    }
}
