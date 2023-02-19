#![feature(default_free_fn)]
#![feature(let_chains)]
#![feature(is_some_and)]

use {
	rand::Rng,
	std::{
		collections::{
			HashMap,
			HashSet,
		},
		default::default,
	},
	vec1::{
		vec1,
		Vec1,
	},
};

const MAX_CARDS_SMALL: u8 = 7;

fn main()
{
	let mut players = vec1! {Player::new("Benjamin"), Player::new("Nick")};
	let mut shuffled_deck: Cards = shuffle();

	let winner = play_game(&mut players, &mut shuffled_deck);
	println!("Winner is {winner:?}");
}

/// Returns the winner of the game.
fn play_game<'a>(
	players: &'a mut Players<'a>,
	shuffled_deck: &mut Cards,
) -> &'a Player<'a>
{
	let round_number = 0u32;
	while !is_any_player_winning(&players)
	{
		println!("Round {round_number}:");
		for mut player in players.iter_mut()
		{
			player_turn(&mut player, shuffled_deck);
		}
		println!("");
	}
	get_winning_player(players)
}

fn player_turn(
	player: &mut Player,
	deck: &mut Cards,
)
{
	let card = player.draw(deck);
	if let Some(card) = card
	{
		if let Some(stored_card) = player.store(card)
		&& let StoredCard::PairedCard(_) = stored_card
		{
			player.play(&stored_card)
		}
	}
}

fn is_any_player_winning(players: &Players) -> bool
{
	players
		.iter()
		.map(|player| player.get_score())
		.max()
		.is_some_and(|sum| sum >= 20)
}

fn get_winning_player<'a>(players: &'a Players<'a>) -> &'a Player<'a>
{
	let mut winning_player_index_and_score: (usize, u8) = (0, 0);
	for player_index_and_score in players
		.iter()
		.enumerate()
		.map(|(index, player)| (index, player.get_score()))
	{
		if &player_index_and_score.1 > &winning_player_index_and_score.1
		{
			winning_player_index_and_score = player_index_and_score;
		}
	}
	&players[winning_player_index_and_score.0]
}

type Cards = Vec<Card>;
type Players<'a> = Vec1<Player<'a>>;

impl Player<'_>
{
	fn draw(
		self: &mut Self,
		sample: &mut Cards,
	) -> Option<Card>
	{
		sample.pop()
	}

	fn store(
		self: &mut Self,
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

	fn play(
		self: &mut Self,
		card: &StoredCard,
	)
	{
		println!("{} played their {card:?}.", self.name);
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

fn shuffle() -> Cards
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

		let mut shuffled_deck: Cards = shuffle();
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
