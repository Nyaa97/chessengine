use crate::figure::*;
use crate::get_mvs;

pub struct King {
	prop: Figure,
}

impl Figures for King {
	fn new(colour: Colour) -> Self {
		Self {
			prop: Figure::new(colour, 1000),
		}
	}

	fn get_colour(&self) -> &Colour { &self.prop.colour }

	fn get_val(&self) -> &isize { &self.prop.val }

	fn get_symbol(&self) -> &str {
		if self.prop.colour == Colour::White {
			"K"
		}
		else {
			"k"
		}
	}

	fn get_mvs(&self, board: &[[Option<&Piece>; 8]; 8], from: (usize, usize)) -> Vec<(usize, usize)> {
		get_mvs!(self, board, from, 2, [
			(-1, -1),
			(-1, 0),
			(-1, 1),
			(0, -1),
			(0, 1),
			(1, -1),
			(1, 0),
			(1, 1)
		])
	}
}
