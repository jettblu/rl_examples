use rand::Rng;

pub struct Blackjack {
    pub dealer: Dealer,
    pub player: Player,
    pub is_player_turn: bool,
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
        }
    }

    pub fn reset(&mut self) {
        self.dealer = Dealer::new();
        self.player = Player::new();
        self.is_player_turn = true;
    }

    pub fn step(&mut self, action: usize) -> f64 {
        if self.is_player_turn {
            match action {
                0 => self.player.draw_card(),
                1 => {
                    self.is_player_turn = false;
                }
                _ => panic!("Invalid action"),
            }
        } else {
            while self.dealer.get_sum() < 17 {
                self.dealer.draw_card();
            }
        }
        if self.is_player_turn {
            if self.player.did_bust() {
                return -1.0;
            }
        } else {
            if self.dealer.did_bust() {
                return 1.0;
            } else if self.dealer.get_sum() > self.player.get_sum() {
                return -1.0;
            } else if self.dealer.get_sum() < self.player.get_sum() {
                return 1.0;
            } else {
                return 0.0;
            }
        }
        0.0
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
