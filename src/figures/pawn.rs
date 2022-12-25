use crate::figure::*;

pub struct Pawn {
	prop: Figure,
}

impl Figures for Pawn {
	fn new(colour: Colour) -> Self {
		Self {
			prop: Figure::new(colour, 1),
		}
	}

	fn get_colour(&self) -> &Colour { &self.prop.colour }

	fn get_symbol(&self) -> &str {
		if self.prop.colour == Colour::White {
			"P"
		}
		else {
			"p"
		}
	}

	fn get_val(&self) -> &isize { &self.prop.val }

	fn get_mvs(&self, board: &[[Option<&Piece>; 8]; 8], from: (usize, usize)) -> Vec<(usize, usize)> {
		let mut out: Vec<(usize, usize)> = Vec::new();

		if self.prop.colour == Colour::White {
			if board[from.0][from.1 + 1].is_none() {
				out.push((from.0, from.1 + 1));
				if from.1 == 1 && board[from.0][from.1 + 2].is_none() {
					out.push((from.0, from.1 + 2));
				}
			}
			if from.0 > 0
				&& board[from.0 - 1][from.1 + 1].is_some_and(|x| *x.figure.get_colour() == Colour::Black)
			{
				out.push((from.0 - 1, from.1 + 1));
			}
			if from.0 < 7
				&& board[from.0 + 1][from.1 + 1].is_some_and(|x| *x.figure.get_colour() == Colour::Black)
			{
				out.push((from.0 + 1, from.1 + 1));
			}
		}
		else {
			if board[from.0][from.1 - 1].is_none() {
				out.push((from.0, from.1 - 1));
				if from.1 == 6 && board[from.0][from.1 - 2].is_none() {
					out.push((from.0, from.1 - 2));
				}
			}
			if from.0 > 0
				&& board[from.0 - 1][from.1 - 1].is_some_and(|x| *x.figure.get_colour() == Colour::White)
			{
				out.push((from.0 - 1, from.1 - 1));
			}
			if from.0 < 7
				&& board[from.0 + 1][from.1 - 1].is_some_and(|x| *x.figure.get_colour() == Colour::White)
			{
				out.push((from.0 + 1, from.1 - 1));
			}
		}

		out
	}
}
