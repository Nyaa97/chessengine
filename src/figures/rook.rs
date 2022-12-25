use crate::figure::*;
use crate::get_mvs;

pub struct Rook {
	prop: Figure,
}

impl Figures for Rook {
	fn new(colour: Colour) -> Self {
		Self {
			prop: Figure::new(colour, 5),
		}
	}

	fn get_colour(&self) -> &Colour { &self.prop.colour }

	fn get_val(&self) -> &isize { &self.prop.val }

	fn get_symbol(&self) -> &str {
		if self.prop.colour == Colour::White {
			"R"
		}
		else {
			"r"
		}
	}

	fn get_mvs(&self, board: &[[Option<&Piece>; 8]; 8], from: (usize, usize)) -> Vec<(usize, usize)> {
		get_mvs!(self, board, from, 8, [(-1, 0), (0, -1), (0, 1), (1, 0)])
	}
}
