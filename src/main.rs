#![feature(default_free_fn)]

use {
	rand::Rng,
	std::{
		collections::{
			HashMap,
			HashSet,
		},
		default::default,
	},
};

const MAX_CARDS_SMALL: u8 = 7;

fn main() {}

type StackOfCards = Vec<Card>;

impl Player<'_>
{
	fn draw(self: &mut Self, sample: &mut StackOfCards) -> Option<Card> { sample.pop() }

	fn store(self: &mut Self, card: Card)
	{
		if self.unplayed_cards.remove(&StoredCard::PairedCard(card))
		{
			println!("Overstored {card:?}");
		}
		else if self.unplayed_cards.remove(&StoredCard::UnpairedCard(card))
		{
			println!("Paired {card:?}");
			self.unplayed_cards.insert(StoredCard::PairedCard(card));
		}
		else
		{
			println!("Added {card:?}");
			self.unplayed_cards.insert(StoredCard::UnpairedCard(card));
		}
	}

	fn play(self: &mut Self, card: &StoredCard)
	{
		self.unplayed_cards.remove(card);
		self.played_cards.push(*card)
	}

	fn get_score(self: &Self) -> u8
	{
		self
			.played_cards
			.iter()
			.map(|card| -> u8 { card.get_score() })
			.sum()
	}
}

fn shuffle() -> StackOfCards
{
	let mut deck: HashMap<Card, u8> = HashMap::new();
	let mut shuffled: StackOfCards = default();
	for _ in 0..50
	{
		loop
		{
			let new_card = match rand::thread_rng().gen_range(0..8)
			{
				0 => Card::One,
				1 => Card::Two,
				2 => Card::Three,
				3 => Card::Four,
				4 => Card::Five,
				5 => Card::Six,
				6 => Card::Seven,
				7 => Card::Ten,
				_ => unreachable!(),
			};
			let chosen_card_count = deck.entry(new_card).or_insert(0);
			// println!("new_card: {new_card:?}, chosen_card_count: {chosen_card_count}");
			let is_new_small = new_card != Card::Ten && *chosen_card_count < MAX_CARDS_SMALL;
			let is_new_ten = new_card == Card::Ten && *chosen_card_count == 0;
			if is_new_small || is_new_ten
			{
				*chosen_card_count = *chosen_card_count + 1u8;
				shuffled.push(new_card);
				break
			}
		}
	}
	shuffled
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Player<'a>
{
	is_finished:    bool,
	unplayed_cards: HashSet<StoredCard>,
	played_cards:   Vec<StoredCard>,
	name:           &'a str,
}

impl<'a> Player<'a>
{
	fn new(name: &'a str) -> Self
	{
		Player {
			unplayed_cards: default(),
			played_cards: default(),
			is_finished: false,
			name,
		}
	}
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
enum Card
{
	One   = 1,
	Two   = 2,
	Three = 3,
	Four  = 4,
	Five  = 5,
	Six   = 6,
	Seven = 7,
	Ten   = 10,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
enum StoredCard
{
	UnpairedCard(Card),
	PairedCard(Card),
}

impl StoredCard
{
	fn get_score(self: Self) -> u8
	{
		match self
		{
			StoredCard::UnpairedCard(card) => card as u8,
			StoredCard::PairedCard(card) => 2 * card as u8,
		}
	}
}

#[cfg(test)]
pub mod tests
{
	use crate::*;

	#[test]
	pub fn test_2_players_drawing_storing_playing()
	{
		let player_names: Vec<&str> = vec!["Benjamin", "Nick"];
		let mut players: Vec<Player> = player_names
			.iter()
			.map(|name| -> Player { Player::new(name) })
			.collect();

		let mut shuffled_deck: StackOfCards = shuffle();
		shuffled_deck = dbg!(shuffled_deck);

		let player_one = players.get_mut(0).unwrap();

		let card_one = player_one.draw(&mut shuffled_deck).unwrap();
		player_one.store(card_one);
		dbg!(player_one.get_score());

		let player_two = players.get_mut(1).unwrap();
		let card_two = player_two.draw(&mut shuffled_deck).unwrap();
		player_two.store(card_two);
		player_two.store(card_two);
		player_two.play(&StoredCard::UnpairedCard(card_two));
		dbg!(player_two.get_score());

		let card_remaining_in_deck = shuffled_deck.len();
		dbg!(card_remaining_in_deck);
	}
}
