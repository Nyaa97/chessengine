use crate::figure::*;
use crate::get_mvs;

pub struct Knight {
	prop: Figure,
}

impl Figures for Knight {
	fn new(colour: Colour) -> Self {
		Self {
			prop: Figure::new(colour, 3),
		}
	}

	fn get_colour(&self) -> &Colour { &self.prop.colour }

	fn get_val(&self) -> &isize { &self.prop.val }

	fn get_symbol(&self) -> &str {
		if self.prop.colour == Colour::White {
			"N"
		}
		else {
			"n"
		}
	}

	fn get_mvs(&self, board: &[[Option<&Piece>; 8]; 8], from: (usize, usize)) -> Vec<(usize, usize)> {
		get_mvs!(self, board, from, 2, [
			(-2, -1),
			(-2, 1),
			(-1, -2),
			(-1, 2),
			(1, -2),
			(1, 2),
			(2, -1),
			(2, 1)
		])
	}
}
