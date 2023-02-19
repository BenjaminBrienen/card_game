use {
	crate::card::*,
	std::{
		collections::HashSet,
		default::default,
	},
	vec1::Vec1,
};

pub type Players<'a> = Vec1<Player<'a>>;

impl<'a> Player<'a>
{
	pub fn new(name: &'a str) -> Self
	{
		Player {
			unplayed_cards: default(),
			played_cards: default(),
			name,
		}
	}

	pub fn draw(
		&mut self,
		sample: &mut Cards,
	) -> Option<Card>
	{
		sample.pop()
	}

	pub fn store(
		&mut self,
		card: Card,
	) -> Option<StoredCard>
	{
		if self.unplayed_cards.remove(&StoredCard::PairedCard(card))
		{
			println!("{} overstored the {card:?} in their hand, losing all three cards.", self.name);
			None
		}
		else if self.unplayed_cards.remove(&StoredCard::UnpairedCard(card))
		{
			println!("{} paired up the {card:?} in their hand.", self.name);
			let rtn = StoredCard::PairedCard(card);
			self.unplayed_cards.insert(rtn);
			Some(rtn)
		}
		else
		{
			println!("{} added {card:?} to their hand.", self.name);
			let rtn = StoredCard::UnpairedCard(card);
			self.unplayed_cards.insert(rtn);
			Some(rtn)
		}
	}

	pub fn play(
		&mut self,
		card: &StoredCard,
	)
	{
		println!("{} played their {card:?}.", self.name);
		self.unplayed_cards.remove(card);
		self.played_cards.push(*card)
	}

	pub fn get_score(&self) -> u8
	{
		self
			.played_cards
			.iter()
			.map(|card| -> u8 { card.get_score() })
			.sum()
	}
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Player<'a>
{
	unplayed_cards: HashSet<StoredCard>,
	played_cards:   Vec<StoredCard>,
	name:           &'a str,
}
