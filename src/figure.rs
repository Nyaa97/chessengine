use serde::Deserialize;

pub struct Piece {
	pub figure: Box<dyn Figures>,
}

#[derive(Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Colour {
	Black,
	White,
}

#[derive(PartialEq, Eq)]
pub struct Figure {
	pub colour: Colour,
	pub val: isize,
}

impl From<Colour> for isize {
	fn from(val: Colour) -> Self {
		match val {
			Colour::Black => -1,
			Colour::White => 1,
		}
	}
}

impl Figure {
	pub fn new(colour: Colour, val: isize) -> Self {
		Figure {
			colour,
			val,
		}
	}

	pub fn get_mvs(
		&self,
		board: &[[Option<&Piece>; 8]; 8],
		from: &(usize, usize),
		dir: (isize, isize),
		lim: isize,
		vec: &mut Vec<(usize, usize)>,
	) {
		for i in 1..lim {
			let (x, y) = (from.0 as isize + i * dir.0, from.1 as isize + i * dir.1);
			if x < 0 || x > 7 || y < 0 || y > 7 {
				break;
			}
			let (x, y) = (x as usize, y as usize);
			if board[x][y].is_some() {
				if self.colour != *board[x][y].unwrap().figure.get_colour() {
					vec.push((x, y));
				}
				break;
			}
			vec.push((x, y));
		}
	}
}

pub trait Figures {
	fn new(colour: Colour) -> Self
	where
		Self: Sized;
	fn get_colour(&self) -> &Colour;
	fn get_mvs(&self, board: &[[Option<&Piece>; 8]; 8], from: (usize, usize)) -> Vec<(usize, usize)>;
	fn get_val(&self) -> &isize;
	fn get_symbol(&self) -> &str;
}

#[macro_export]
macro_rules! get_mvs {
	($self:expr, $board:expr, $from:expr, $lim:expr, [$($dir:expr), *]) => {
		{
			let mut out: Vec<(usize, usize)> = Vec::new();
			$( $self.prop.get_mvs($board, &$from, $dir, $lim, &mut out ); )*
			out
		}
	};
}
