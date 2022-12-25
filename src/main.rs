#![feature(is_some_and)]

mod board;
mod clap;
mod figure;
mod figures;
mod graph;
mod json;

use std::collections::HashMap;

pub use board::*;
use ::clap::Parser;
use graph::Graph;

use crate::clap::CLI;
use crate::figure::*;
use crate::figures::*;

fn main() -> Result<(), ()> {
	let mut piece: HashMap<&str, Piece> = HashMap::new();
	piece.insert("b", Piece {
		figure: Box::new(Bishop::new(Colour::Black)),
	});
	piece.insert("B", Piece {
		figure: Box::new(Bishop::new(Colour::White)),
	});
	piece.insert("k", Piece {
		figure: Box::new(King::new(Colour::Black)),
	});
	piece.insert("K", Piece {
		figure: Box::new(King::new(Colour::White)),
	});
	piece.insert("n", Piece {
		figure: Box::new(Knight::new(Colour::Black)),
	});
	piece.insert("N", Piece {
		figure: Box::new(Knight::new(Colour::White)),
	});
	piece.insert("p", Piece {
		figure: Box::new(Pawn::new(Colour::Black)),
	});
	piece.insert("P", Piece {
		figure: Box::new(Pawn::new(Colour::White)),
	});
	piece.insert("q", Piece {
		figure: Box::new(Queen::new(Colour::Black)),
	});
	piece.insert("Q", Piece {
		figure: Box::new(Queen::new(Colour::White)),
	});
	piece.insert("r", Piece {
		figure: Box::new(Rook::new(Colour::Black)),
	});
	piece.insert("R", Piece {
		figure: Box::new(Rook::new(Colour::White)),
	});

	let mut piece2: HashMap<&str, Option<&Piece>> = HashMap::new();
	piece2.insert("b", Some(&piece["b"]));
	piece2.insert("B", Some(&piece["B"]));
	piece2.insert("k", Some(&piece["k"]));
	piece2.insert("K", Some(&piece["K"]));
	piece2.insert("n", Some(&piece["n"]));
	piece2.insert("N", Some(&piece["N"]));
	piece2.insert("p", Some(&piece["p"]));
	piece2.insert("P", Some(&piece["P"]));
	piece2.insert("q", Some(&piece["q"]));
	piece2.insert("Q", Some(&piece["Q"]));
	piece2.insert("r", Some(&piece["r"]));
	piece2.insert("R", Some(&piece["R"]));
	piece2.insert(" ", None);

	let arg = CLI::parse();

	let board = Board::from_path(arg.json.unwrap(), &piece2);


	let graph = Graph::new(board);

	let mut lookup: HashMap<String, (usize, isize)> = HashMap::new();

	let res = graph.predict(5, &piece2, &mut lookup);

	println!(
		"\nRatio: {}\nMove: {}{} -> {}{}",
		res.0,
		char::from_u32((res.1.0.0 + 65).try_into().unwrap()).unwrap(),
		res.1.0.1 + 1,
		char::from_u32((res.1.1.0 + 65).try_into().unwrap()).unwrap(),
		res.1.1.1 + 1
	);

	Ok(())
}
