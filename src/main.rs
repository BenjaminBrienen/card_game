#![feature(default_free_fn)]

use {
	rand::{
		distributions::{
			Distribution,
			Standard,
		},
		Rng,
	},
	std::{
		collections::HashMap,
		default::default,
	},
};

const MAX_CARDS_SMALL: u8 = 7;

fn main()
{
	let shuffled_deck: Vec<Card> = shuffle();

	println!("{shuffled_deck:?}");
}

fn shuffle() -> Vec<Card>
{
	let mut deck: HashMap<Card, u8> = HashMap::new();
	let mut shuffled: Vec<Card> = default();
	for _ in 0..50
	{
		loop
		{
			let new_card: Card = rand::random();
			let chosen_card_count = deck.entry(new_card).or_insert(0);
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

impl Distribution<Card> for Standard
{
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Card
	{
		match rng.gen_range(0..8)
		{
			// rand 0.8
			0 => Card::One,
			1 => Card::Two,
			2 => Card::Three,
			3 => Card::Four,
			4 => Card::Five,
			5 => Card::Six,
			6 => Card::Seven,
			7 => Card::Ten,
			_ => unreachable!(),
		}
	}
}
