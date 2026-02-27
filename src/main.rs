use colored::ColoredString;
use rand::seq::SliceRandom;
use std::io;
use std::cmp;
use std::u8;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::cmp::Ordering;
use colored::Colorize;

enum TextType {
    Notification,
    Bad,
    Ok,
    Good,
    VeryGood,
    Money,
    Hearts,
    Diamonds,
    Clubs,
    Spades,
    BlackJoker,
    RedJoker,
    Dungeon,
    Shop,
    Lost,
    Won,
    Command,
}

impl TextType {
    fn stylize(self, text: &str) -> ColoredString {
        match self {
            Self::Notification => text.italic().truecolor(110, 110, 110),
            Self::Bad => text.red(),
            Self::Ok => text.yellow(),
            Self::Good => text.green(),
            Self::VeryGood => text.blue(),
            Self::Money => text.truecolor(200, 150, 25),
            Self::Hearts => text.truecolor(200, 0, 0),
            Self::Diamonds => text.truecolor(200, 75, 25),
            Self::Clubs => text.truecolor(25, 100, 25),
            Self::Spades => text.truecolor(25, 25, 100),
            Self::BlackJoker => text.truecolor(150, 25, 150),
            Self::RedJoker => text.truecolor(255, 25, 75),
            Self::Dungeon => text.bold().truecolor(0, 50, 75),
            Self::Shop => text.bold().truecolor(100, 50, 0),
            Self::Lost => text.bold().red(),
            Self::Won => text.bold().green(),
            Self::Command => text.truecolor(110, 110, 110),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, EnumIter, Eq)]
enum Rank {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum JokerColor {
    Red,
    Black,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CardType {
    Regular {
        suit: Suit,
        rank: Rank,
    },
    Joker {
        color: JokerColor,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Card {
    card_type: CardType,
}

impl Card {
    fn get_value(&self) -> u32 {
        match self.card_type {
            CardType::Regular { rank, .. } => {
                rank as u32
            }
            CardType::Joker { .. } => {
                15
            }
        }
    }

    fn display(&self) -> ColoredString {
        let text = match self.card_type {
            CardType::Regular { suit, rank } => {
                let suit_symbol = match suit {
                    Suit::Hearts => String::from("♥"),
                    Suit::Diamonds => String::from("♦"),
                    Suit::Clubs => String::from("♣"),
                    Suit::Spades => String::from("♠"),
                };
                let rank_symbol = match rank {
                    Rank::Ace => String::from("A"),
                    Rank::Jack => String::from("J"),
                    Rank::Queen => String::from("Q"),
                    Rank::King => String::from("K"),
                    _ => format!("{}", rank as u8),
                };
                format!("{}{}", rank_symbol, suit_symbol)
            },
            CardType::Joker{ .. } => {
                String::from("Jo")
            }
        };

        match self.card_type {
            CardType::Regular { suit, rank } => {
                match suit {
                    Suit::Hearts => TextType::Hearts.stylize(
                        &format!("{} ({})", match rank {
                            Rank::Four => "Drop",
                            Rank::Five => "Vial",
                            Rank::Six => "Cup",
                            Rank::Seven => "Bottle",
                            Rank::Eight => "Flask",
                            Rank::Nine => "Goblet",
                            Rank::Jack => "Glittering Dew",
                            Rank::Queen => "Golden Apple",
                            Rank::King => "Ambrosia",
                            _ => "",
                        }, text)
                    ),
                    Suit::Diamonds => TextType::Diamonds.stylize(
                        &format!("{} ({})", match rank {
                            Rank::Four => "Stick",
                            Rank::Five => "Club",
                            Rank::Six => "Dagger",
                            Rank::Seven => "Shortsword",
                            Rank::Eight => "Broadsword",
                            Rank::Nine => "Greatsword",
                            Rank::Ten => "Enchanted Blade",
                            Rank::Jack => "Bronze Anvil",
                            Rank::Queen => "Sterling Anvil",
                            Rank::King => "Cobalt Anvil",
                            _ => "",
                        }, text)
                    ),
                    Suit::Clubs => TextType::Clubs.stylize(
                        &format!("{} ({})", match rank {
                            Rank::Four => "Rat",
                            Rank::Five => "Slime",
                            Rank::Six => "Goblin",
                            Rank::Seven => "Spider",
                            Rank::Eight => "Rat King",
                            Rank::Nine => "Ogre",
                            Rank::Jack => "Giant",
                            Rank::Queen => "Goblin Chief",
                            Rank::King => "Demon",
                            _ => "",
                        }, text)
                    ),
                    Suit::Spades => TextType::Spades.stylize(
                        &format!("{} ({})", match rank {
                            Rank::Four => "Maggot",
                            Rank::Five => "Cursed Skull",
                            Rank::Six => "Skeleton",
                            Rank::Seven => "Zombie",
                            Rank::Eight => "Undead Squire",
                            Rank::Nine => "Undead Knight",
                            Rank::Jack => "Bone King",
                            Rank::Queen => "Amalgamation",
                            Rank::King => "Lich",
                            _ => "",
                        }, text)
                    ),
                }
            }
            CardType::Joker { color } => {
                match color {
                    JokerColor::Black => TextType::BlackJoker.stylize("Ice Spell (Jo)"),
                    JokerColor::Red => TextType::RedJoker.stylize("Fire Spell (Jo)"),
                }
            }
        }
    }

    fn upgrade(&self, amount: u8) -> Card {
        match &self.card_type {
            CardType::Regular { suit, rank } => {
                let new_value = *rank as u8 + amount;
                return Card {
                    card_type: CardType::Regular { suit: *suit, rank: match new_value {
                        1 => Rank::Ace,
                        2 => Rank::Two,
                        3 => Rank::Three,
                        4 => Rank::Four,
                        5 => Rank::Five,
                        6 => Rank::Six,
                        7 => Rank::Seven,
                        8 => Rank::Eight,
                        9 => Rank::Nine,
                        _ => Rank::Ten,
                    } }
                }
            },
            CardType::Joker { .. } => return self.clone(),
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.card_type {
            CardType::Regular { suit, rank } => match other.card_type {
                CardType::Regular { suit: other_suit, rank: other_rank } => {
                    rank.cmp(&other_rank).then_with(|| suit.cmp(&other_suit))
                }
                CardType::Joker { .. } => Ordering::Greater
            },
            CardType::Joker { color } => match other.card_type {
                CardType::Regular { .. } => Ordering::Less,
                CardType::Joker { color: other_color } => color.cmp(&other_color)
            }
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Game {
    dungeon: Vec<Card>,
    dungeon_discard: Vec<Card>,
    room: Vec<Card>,
    bosses: Vec<Card>,
    shop: Vec<Card>,
    shop_stock: Vec<Card>,
    shop_discard: Vec<Card>,
    health: u8,
    money: u32,
    weapon: Option<Card>,
    weapon_durability: u8,
    fled: bool,
    state: GameState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum GameState {
    Floor,
    Shop,
    Lost,
    Won,
}

impl Game {
    fn new() -> Self {
        let mut deck: Vec<Card> = Self::create_deck();
        deck.shuffle(&mut rand::rng());

        let mut dungeon = vec![];
        let mut bosses = vec![];
        let mut shop = vec![];
        for card in deck {
            match card.card_type {
                CardType::Regular { suit, rank } => {
                    match rank as u8 {
                        4..=9 => {
                            dungeon.push(card);
                        },
                        11..=13 => {
                            match suit {
                                Suit::Hearts | Suit::Diamonds => shop.push(card),
                                Suit::Clubs | Suit::Spades => bosses.push(card),
                            }
                        },
                        _ => { }
                    }
                },
                CardType::Joker { .. } => {
                    shop.push(card);
                }
            }
        }
        bosses.sort();

        Game {
            dungeon,
            dungeon_discard: vec![],
            room: vec![],
            bosses,
            shop,
            shop_stock: vec![],
            shop_discard: vec![],
            health: 12, 
            money: 5, 
            weapon: None,
            weapon_durability: u8::MAX, 
            fled: false, 
            state: GameState::Floor,
        }
    }

    fn start_floor(&mut self) {
        self.health = 12;
        self.weapon = None;
        self.weapon_durability = u8::MAX;

        self.dungeon.append(&mut self.room);
        self.dungeon.append(&mut self.dungeon_discard);
        self.dungeon.shuffle(&mut rand::rng());
    }

    fn create_deck() -> Vec<Card> {
        let mut deck = Vec::with_capacity(52);
        for suit in [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for rank in Rank::iter() {
                deck.push(Card {
                    card_type: CardType::Regular { suit, rank },
                });
            }
        }
        deck.push(Card {
            card_type: CardType::Joker { color: JokerColor::Black }
        });
        deck.push(Card {
            card_type: CardType::Joker { color: JokerColor::Red }
        });
        deck
    }

    fn refresh_room(&mut self, quiet: bool) {
        // restock room
        if self.room.len() <= 1 {
            let amount_add = cmp::min(4 - self.room.len(), self.dungeon.len());
            for _i in 0..amount_add {
                self.room.push(self.dungeon.remove(0));
            }

            if amount_add > 0 && self.state == GameState::Floor && !quiet {
                println!("{}", TextType::Notification.stylize("Restocked room"));
            }
        }    

        // check if lost
        if self.health <= 0 {
            self.state = GameState::Lost;
            println!("{}", TextType::Bad.stylize("You lost"));
            return
        }

        // check if won
        if self.dungeon.is_empty() && !self.room.iter().any(|card| 
            matches!(card.card_type, CardType::Regular { suit: Suit::Clubs | Suit::Spades, .. })) {
            
            println!("{}", TextType::Good.stylize("Floor complete!"));

            if self.bosses.is_empty() {
                self.state = GameState::Won;
            } else {
                self.state = GameState::Shop;
                for _i in 0..cmp::min(self.shop.len(), 4) {
                    self.shop_stock.push(self.shop.remove(0));
                }
            }
        }
    }

    fn display(&self) {
        match self.state {
            GameState::Floor => {
                println!("{}", TextType::Dungeon.stylize("===== Dungeon ====="));
                println!("{} card(s) left in Dungeon", self.dungeon.len());

                let health_text = format!("{}/12 HP", self.health);
                let health_color = match self.health {
                    0..=4 => TextType::Bad,
                    5..=8 => TextType::Ok,
                    9..=12 => TextType::Good,
                    _ => TextType::VeryGood,
                };
                let money_text = format!("${}", self.money);
                println!("{}, {}", health_color.stylize(health_text.as_str()), TextType::Money.stylize(money_text.as_str()));

                for i in 0..self.room.len() {
                    println!("{}. {}", i+1, self.room[i].display());
                }

                match &self.weapon {
                    Some(weapon) => {
                        print!("Weapon: {}", weapon.display());
                        if self.weapon_durability < u8::MAX {
                            print!(" ({} durability)", self.weapon_durability);
                        }
                        print!("\n");
                    }
                    None => { }
                }

                println!("{}", TextType::Command.stylize("Commands: use [1-4], flee, quit"));
            }
            GameState::Lost => {
                println!("{}", TextType::Lost.stylize("===== Game over ====="));
                println!("{}", TextType::Command.stylize("Commands: retry, quit"));
            }
            GameState::Shop => {
                println!("{}", TextType::Shop.stylize("===== Shop ====="));
                println!("{}", TextType::Money.stylize(format!("${}", self.money).as_str()));
                if !self.shop_stock.is_empty() {
                    for i in 0..cmp::min(self.shop_stock.len(), 4) {
                        let card = &self.shop_stock[i];
                        println!("{}. {} - {}", i+1, card.display(), TextType::Money.stylize(format!("${}", card.get_value()).as_str()));
                    }
                }
                
                println!("{}", TextType::Command.stylize("Commands: buy [card 1-4], continue, quit"));
            }
            GameState::Won => {
                println!("{}", TextType::Won.stylize("===== You win! ====="));
                println!("{}", TextType::Command.stylize("Commands: retry, quit"));
            }
        }
        print!("> ");
    }

    fn use_card(&mut self, mut room_idx: usize) {
        if room_idx == 0 || room_idx-1 >= self.room.len() {
            println!("{}", TextType::Bad.stylize(format!("No card in room slot {}", room_idx).as_str()));
            return
        }

        match self.room[room_idx-1].card_type {
            CardType::Joker { .. } => {
                println!("Choose a card to destroy:");
                print!("> ");
                io::Write::flush(&mut io::stdout()).unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                match input.trim().parse::<usize>() {
                    Ok(idx) => {
                        if idx == 0 || idx-1 >= self.room.len() {
                            println!("{}", TextType::Bad.stylize(format!("No card in room slot {}", room_idx).as_str()));
                            return
                        }
                        if idx == room_idx {
                            println!("{}", TextType::Bad.stylize("Cannot destroy itself"));
                            return
                        }

                        let v = self.room[idx-1].get_value().div_ceil(2);

                        println!("Destroyed {}, {}", self.room[idx-1].display(), TextType::Money.stylize(format!("+${}", v).as_str()));
                        self.money += v;
                        self.dungeon_discard.push(self.room.remove(idx-1));
                        if idx < room_idx {
                            room_idx -= 1;
                        }
                    }
                    Err(_) => {
                        println!("{}", TextType::Bad.stylize("Must enter a number between 1 and 4"));
                        return
                    }
                }
            }
            CardType::Regular { suit, rank } => match suit {
                Suit::Clubs | Suit::Spades => {
                    let mut barehanded = false;
                    match &self.weapon {
                        Some(weapon) => {
                            if self.weapon_durability > rank as u8 {
                                println!("Fought {} using {}", 
                                    self.room[room_idx-1].display(), 
                                    weapon.display());
                                let d: i16 = rank as i16 - weapon.get_value() as i16;
                                if d < 0 {
                                    self.money += d.abs() as u32;
                                    print!("{}\n", TextType::Money.stylize(format!("+${}", d.abs() as u32).as_str()));
                                } else if d > 0 {
                                    self.health = cmp::max(self.health as i16 - d as i16, 0) as u8;
                                    print!("{}", TextType::Bad.stylize(format!("-{} HP\n", d as u8).as_str()));
                                }
                                self.weapon_durability = rank as u8;
                            } else {
                                println!("{} {}", 
                                    weapon.display(),
                                    TextType::Bad.stylize("broke!"));
                                self.weapon = None;
                                barehanded = true;
                            }
                        },
                        None => { 
                            barehanded = true; 
                        }
                    }
                    if barehanded {
                        println!("Fought {} barehanded", self.room[room_idx-1].display());
                        self.health = cmp::max(self.health as i16 - rank as i16, 0) as u8;
                        print!("{}", TextType::Bad.stylize(format!("-{} HP\n", rank as u8).as_str()));
                    }
                }
                Suit::Hearts => {
                    if rank < Rank::Jack {
                        self.health = cmp::min(self.health + rank as u8, cmp::max(12, self.health));
                        println!("Imbibed {}", self.room[room_idx-1].display());
                        println!("{}", TextType::Good.stylize(format!("+{} HP", rank as u8).as_str()));
                    } else {
                        let absorption = (rank as u8 - Rank::Ten as u8) * 2;
                        self.health = 12 + absorption;
                        println!("Imbibed {}", self.room[room_idx-1].display());
                        println!("{}", TextType::Good.stylize(format!("Full heal + {} HP", absorption).as_str()));
                    }
                },
                Suit::Diamonds => {
                    if rank < Rank::Jack {
                        self.weapon = Some(self.room[room_idx-1].clone());
                        self.weapon_durability = u8::MAX;
                        println!("Equipped {}", self.room[room_idx-1].display())
                    } else {
                        match &self.weapon {
                            Some(weapon) => {
                                let repair = (rank as u8 - Rank::Ten as u8) * 2;
                                let upgrade = rank as u8 - Rank::Ten as u8;
                                if self.weapon_durability < u8::MAX {
                                    self.weapon_durability += repair;
                                }
                                self.weapon = Some(weapon.upgrade(upgrade));
                                println!("{}", TextType::Good.stylize(format!("Repaired {} durability, upgraded weapon", repair).as_str()));
                            },
                            None => {
                                println!("{}", TextType::Bad.stylize("No weapon equiped!"))
                            }
                        }
                        
                    }
                }
            }
        }

        self.dungeon_discard.push(self.room.remove(room_idx-1));
        self.fled = false;
    }

    fn flee(&mut self) {
        if self.room.len() < 4 {
            println!("{}", TextType::Bad.stylize("Can only flee from a full room"));
            return
        }
        if self.fled {
            println!("{}", TextType::Bad.stylize("Cannot flee twice in a row"));
            return
        }

        for _i in 0..4 {
            self.dungeon.push(self.room.pop().expect("ERR: Too few cards in room"));
        }
        self.fled = true;

        println!("{}", TextType::Bad.stylize("Fled from room!"));
    }

    fn buy_card(&mut self, shop_idx: usize) {
        if shop_idx == 0 || shop_idx-1 >= self.shop_stock.len() {
            println!("{}", TextType::Bad.stylize(format!("No card in shop slot {}", shop_idx).as_str()));
            return
        }

        let card = &self.shop_stock[shop_idx-1];
        if self.money >= card.get_value() {
            println!("{}, {} added to dungeon", TextType::Bad.stylize(format!("-${}", card.get_value()).as_str()), card.display());
            self.money -= card.get_value();
            self.dungeon.push(self.shop_stock.remove(shop_idx-1));
        } else {
            println!("{}", TextType::Bad.stylize("Can't afford card"));
        }
    }

    // debug
    fn steal_card(&mut self, shop_idx: usize) {
        if shop_idx == 0 || shop_idx-1 >= self.shop_stock.len() {
            println!("{}", TextType::Bad.stylize(format!("No card in shop slot {}", shop_idx).as_str()));
            return
        }

        self.dungeon.push(self.shop_stock.remove(shop_idx-1));
    }
}

fn main() {
    let mut game = Game::new();
    game.start_floor();
    game.refresh_room(true);

    loop {
        game.display();
        io::Write::flush(&mut io::stdout()).unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        match game.state {
            GameState::Floor => {
                match parts.as_slice() {
                    ["use", card] => {
                        match card.parse::<usize>() {
                            Ok(idx) => game.use_card(idx),
                            Err(_) => println!("{}", TextType::Bad.stylize("Must enter a number between 1 and 4")),
                        }
                    }
                    ["flee"] => game.flee(),
                    ["quit"] => break,
                    ["win"] => { // debug
                        println!("{}", TextType::Good.stylize("Floor complete!"));

                        if game.bosses.is_empty() {
                            game.state = GameState::Won;
                        } else {
                            game.state = GameState::Shop;
                            for _i in 0..cmp::max(game.shop.len(), 4) {
                                game.shop_stock.push(game.shop.remove(0));
                            }
                        }
                    }
                    _ => println!("{}", TextType::Bad.stylize("Invalid command")),
                }

                game.refresh_room(false);
            }
            GameState::Lost | GameState::Won => {
                match parts.as_slice() {
                    ["retry"] => {
                        game = Game::new();
                        game.start_floor();
                        game.refresh_room(true);
                    }
                    ["quit"] => break,
                    _ => println!("{}", TextType::Bad.stylize("Invalid command")),
                }
            }
            GameState::Shop => {
                match parts.as_slice() {
                    ["buy", card] => {
                        match card.parse::<usize>() {
                            Ok(idx) => game.buy_card(idx),
                            Err(_) => println!("{}", TextType::Bad.stylize("Must enter a number between 1 and 4")),
                        }
                    }
                    ["steal", card] => { // debug
                        match card.parse::<usize>() {
                            Ok(idx) => game.steal_card(idx),
                            Err(_) => println!("{}", TextType::Bad.stylize("Must enter a number between 1 and 4")),
                        }
                    }
                    ["continue"] => {
                        game.shop_discard.append(&mut game.shop_stock);
                        if game.shop.is_empty() {
                            println!("{}", TextType::Notification.stylize("Shop restocked"));
                            game.shop.append(&mut game.shop_discard);
                            game.shop.shuffle(&mut rand::rng());
                        }
                        println!("{} & {} added to dungeon", game.bosses[0].display(), game.bosses[1].display());
                        game.dungeon.push(game.bosses.remove(0));
                        game.dungeon.push(game.bosses.remove(0));

                        game.state = GameState::Floor;
                        game.start_floor();
                        game.refresh_room(true);
                    },
                    ["quit"] => break,
                    _ => println!("{}", TextType::Bad.stylize("Invalid command")),
                }
            }
        }

        // wait
        print!("{}", TextType::Command.stylize("[Press enter]"));
        io::Write::flush(&mut io::stdout()).unwrap();
        io::stdin().read_line(&mut input).unwrap();
    }
}
