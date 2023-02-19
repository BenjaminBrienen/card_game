use {
	rand::Rng,
	std::{
		collections::HashMap,
		default::default,
	},
};

pub type Cards = Vec<Card>;
const MAX_CARDS_SMALL: u8 = 7;

pub fn shuffle() -> Cards
{
	let mut deck: HashMap<Card, u8> = HashMap::new();
	let mut shuffled: Cards = default();
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
pub enum Card
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
pub enum StoredCard
{
	UnpairedCard(Card),
	PairedCard(Card),
}

impl StoredCard
{
	pub fn get_score(self: Self) -> u8
	{
		match self
		{
			StoredCard::UnpairedCard(card) => card as u8,
			StoredCard::PairedCard(card) => 2 * card as u8,
		}
	}
}
