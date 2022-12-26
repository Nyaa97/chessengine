use std::collections::HashMap;

use indicatif::{ProgressBar, ProgressStyle};

use crate::figure::{Colour, Piece};
use crate::Board;

#[derive(Clone)]
pub struct Graph<'a> {
	pub board: Board<'a>,
	val: isize,
}

impl<'a> Graph<'a> {
	pub fn new(board: Board<'a>) -> Self {
		let val = board.get_val();
		Graph {
			board,
			val,
		}
	}

	pub fn next(&self) -> Vec<(((usize, usize), (usize, usize)), Graph)> {
		let mut out = Vec::new();
		for i in 0..8 {
			for j in 0..8 {
				let vec = self.board.get_mvs((i, j));
				for x in vec {
					let mut cp = Self {
						board: self.board.clone(),
						val: 0,
					};
					cp.board.mv((i, j), x);
					cp.board.change_mv();
					cp.val = cp.board.get_val();
					cp.board.gen_code();
					out.push((((i, j), x), cp));
				}
			}
		}

		let vec = self.board.get_castling();
		for x in vec {
			let mut cp = Self {
				board: self.board.clone(),
				val: 0,
			};
			cp.board.mv(x.0.0, x.0.1);
			cp.board.mv(x.1.0, x.1.1);
			cp.board.change_mv();
			cp.val = cp.board.get_val();
			cp.board.gen_code();
			out.push((x.0, cp));
		}

		out
	}

	fn alphabeta_rec(
		&self,
		limit: usize,
		mut a: isize,
		mut b: isize,
		lookup: &mut HashMap<String, (usize, isize)>,
	) -> isize {
		if limit <= 0 || self.val > 500 || self.val < -500 {
			return self.val;
		}
		if lookup.contains_key(&self.board.code) {
			let res = lookup[&self.board.code];
			if res.0 >= limit {
				return res.1;
			}
		}

		if self.board.colour == Colour::White {
			let mut val = -10000;
			let mut code = String::new();
			for (_, next) in self.next().drain(..) {
				let tmp = next.alphabeta_rec(limit - 1, a, b, lookup);
				if tmp > val {
					val = tmp;
					code = self.board.code.clone();
				}
				if val >= b {
					break;
				}
				if val > a {
					a = val;
				}
			}
			lookup.insert(code, (limit, val));
			return val;
		}
		else {
			let mut val = 10000;
			let mut code = String::new();
			for (_, next) in self.next().drain(..) {
				let tmp = next.alphabeta_rec(limit - 1, a, b, lookup);
				if tmp < val {
					val = tmp;
					code = self.board.code.clone();
				}
				if val <= a {
					break;
				}
				if val < b {
					b = val;
				}
			}
			lookup.insert(code, (limit, val));
			return val;
		}
	}

	fn alphabeta(
		&self,
		limit: usize,
		mut a: isize,
		mut b: isize,
		lookup: &mut HashMap<String, (usize, isize)>,
	) -> Vec<(((usize, usize), (usize, usize)), Graph)> {
		let mut out = Vec::new();
		if self.board.colour == Colour::White {
			let mut val = -10000;
			for (mv, mut next) in self.next().drain(..) {
				let tmp = next.alphabeta_rec(limit - 1, a, b, lookup);
				next.val = tmp;
				if tmp >= val {
					if tmp > val {
						val = tmp;
						out.clear();
					}
					out.push((mv, next));
				}
				if val >= b {
					break;
				}
				if val > a {
					a = val;
				}
			}
		}
		else {
			let mut val = 10000;
			for (mv, mut next) in self.next().drain(..) {
				let tmp = next.alphabeta_rec(limit - 1, a, b, lookup);
				next.val = tmp;
				if tmp < val {
					val = tmp;
					out.clear();
				}
				if tmp <= val {
					out.push((mv, next));
				}
				if val <= a {
					break;
				}
				if val < b {
					b = val;
				}
			}
		}
		return out;
	}

	pub fn predict(
		&self,
		limit: usize,
		figures: &HashMap<&str, Option<&Piece>>,
		lookup: &mut HashMap<String, (usize, isize)>,
	) -> (isize, ((usize, usize), (usize, usize))) {
		println!("Predict:");

		let candidates = self.alphabeta(limit, -10000, 10000, lookup);

		if candidates.len() > 1 {
			if self.board.colour == Colour::White {
				Graph::subpredict(Colour::Black, Colour::White, limit, candidates, figures, lookup).unwrap()
			}
			else {
				Graph::subpredict(Colour::White, Colour::Black, limit, candidates, figures, lookup).unwrap()
			}
		}
		else if !candidates.is_empty() {
			(candidates[0].1.val, candidates[0].0)
		}
		else {
			panic!("forfeit");
		}
	}

	fn subpredict(
		colour: Colour,
		colour_org: Colour,
		limit: usize,
		candidates: Vec<(((usize, usize), (usize, usize)), Graph)>,
		figures: &HashMap<&str, Option<&Piece>>,
		lookup: &mut HashMap<String, (usize, isize)>,
	) -> Option<(isize, ((usize, usize), (usize, usize)))> {
		println!("Subpredict: {:?} {} tests", colour, candidates.len());

		let pb = ProgressBar::new(candidates.len().try_into().unwrap());
		pb.set_style(
			ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}]")
				.unwrap()
				.progress_chars("#>-"),
		);

		let mut options = Vec::new();

		if colour == Colour::White {
			for (mv, next) in candidates {
				let mut val = -10000;
				pb.inc(1);
				let mut subcandidates = Vec::new();
				for (_, subgraph) in next.alphabeta(limit, -10000, 10000, lookup) {
					if subgraph.val > val {
						subcandidates.clear();
						val = subgraph.val;
					}

					if subgraph.val >= val {
						let mut arr = [[None; 8]; 8];

						for i in 0..8 {
							for j in 0..8 {
								arr[i][j] = subgraph.board.arr[i][j]
									.map(|v| figures[v.figure.get_symbol()])
									.unwrap_or(None);
							}
						}

						subcandidates.push((mv, Graph {
							board: {
								let mut x = Board {
									arr,
									colour: Colour::Black,
									castling: subgraph.board.castling,
									figures,
									code: "".to_string(),
								};
								x.gen_code();
								x
							},
							val: subgraph.val,
						}));
					}
				}
				options.push((mv, val, subcandidates));
			}
		}
		else {
			for (mv, next) in candidates {
				let mut val = 10000;
				pb.inc(1);
				let mut subcandidates = Vec::new();
				for (_, subgraph) in next.alphabeta(limit, -10000, 10000, lookup) {
					if subgraph.val < val {
						subcandidates.clear();
						val = subgraph.val;
					}

					if subgraph.val == val {
						let mut arr = [[None; 8]; 8];

						for i in 0..8 {
							for j in 0..8 {
								arr[i][j] = subgraph.board.arr[i][j]
									.map(|v| figures[v.figure.get_symbol()])
									.unwrap_or(None);
							}
						}

						subcandidates.push((mv, Graph {
							board: {
								let mut x = Board {
									arr,
									colour: Colour::White,
									castling: subgraph.board.castling,
									figures,
									code: "".to_string(),
								};
								x.gen_code();
								x
							},
							val: subgraph.val,
						}));
					}
				}
				options.push((mv, val, subcandidates))
			}
		}

		pb.finish_and_clear();

		let mut subcandidates = Vec::new();
		let mut target = 0;
		let mut leftoption = 0;
		let mut lastoption = ((0, 0), (0, 0));
		let mut options2 = Vec::new();

		if colour == Colour::White {
			let mut tmp_vec = Vec::new();
			for x in &options {
				if lastoption != x.0 {
					lastoption = x.0;
					target = x.1;
					if !tmp_vec.is_empty() {
						options2.append(&mut tmp_vec);
						tmp_vec.clear();
					}
				}
				if target < x.1 {
					tmp_vec.clear()
				}
				if target <= x.1 {
					tmp_vec.push(x);
				}
			}
		}
		else {
			let mut tmp_vec = Vec::new();
			for x in &options {
				if lastoption != x.0 {
					lastoption = x.0;
					target = x.1;
					if !tmp_vec.is_empty() {
						options2.append(&mut tmp_vec);
						tmp_vec.clear();
					}
				}
				if target > x.1 {
					tmp_vec.clear()
				}
				if target >= x.1 {
					tmp_vec.push(x);
				}
			}
		}

		options2.sort_by_key(|f| f.1);
		if colour_org == Colour::White {
			target = options2.last().unwrap().1;
		}
		else {
			target = options2[0].1;
		}

		for option in options2 {
			if option.1 == target {
				if option.0 != lastoption {
					leftoption += 1;
					lastoption = option.0;
				}
				for x in 0..option.2.len() {
					subcandidates.push(option.2[x].clone());
				}
			}
		}

		println!("Options left: {}", leftoption);
		if subcandidates.len() > 1 {
			if leftoption == 1 {
				return Some((target, subcandidates[0].0));
			}
			if colour == Colour::Black {
				Graph::subpredict(Colour::White, colour_org, limit, subcandidates, figures, lookup)
			}
			else {
				Graph::subpredict(Colour::Black, colour_org, limit, subcandidates, figures, lookup)
			}
		}
		else if !subcandidates.is_empty() {
			Some((subcandidates[0].1.val, subcandidates[0].0))
		}
		else {
			None
		}
	}
}
