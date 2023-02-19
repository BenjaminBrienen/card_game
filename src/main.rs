#![feature(default_free_fn)]
#![feature(let_chains)]
#![feature(is_some_and)]
#![warn(clippy::all)]
#![warn(rustc::all)]
#![allow(unknown_lints)]

use {
	crate::{
		card::shuffle,
		player::{
			Player,
			Players,
		},
	},
	card::{
		Cards,
		StoredCard,
	},
	vec1::vec1,
};

pub mod card;
pub mod player;

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
	let mut round_number = 0u32;
	while !is_any_player_winning(players)
	{
		round_number += 1;
		println!("Round {round_number}:");
		for player in players.iter_mut()
		{
			player_turn(player, shuffled_deck);
		}
		println!();
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
		if player_index_and_score.1 > winning_player_index_and_score.1
		{
			winning_player_index_and_score = player_index_and_score;
		}
	}
	&players[winning_player_index_and_score.0]
}

#[cfg(test)]
pub mod tests
{
	use crate::{
		card::{
			shuffle,
			Cards,
			StoredCard,
		},
		*,
	};

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
