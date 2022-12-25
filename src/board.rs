use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use serde::Deserialize;

use crate::figure::*;
use crate::json;

#[derive(PartialEq, Eq, Deserialize, Clone)]
pub struct Castling {
	bk: bool,
	bq: bool,
	wk: bool,
	wq: bool,
}

#[derive(Clone)]
pub struct Board<'a> {
	pub arr: [[Option<&'a Piece>; 8]; 8],
	pub colour: Colour,
	pub castling: Castling,
	pub figures: &'a HashMap<&'a str, Option<&'a Piece>>,
	pub code: String,
}

impl<'a> Board<'a> {
	pub fn from_path(path: PathBuf, figures: &'a HashMap<&str, Option<&'a Piece>>) -> Self {
		let json: json::Config =
			serde_json::from_reader(BufReader::new(File::open(path).unwrap())).unwrap();

		let mut arr: [[Option<&Piece>; 8]; 8] = [[None; 8]; 8];

		for i in 0..8 {
			for j in 0..8 {
				let x = json.arr[7 - i][j].as_str();
				arr[j][i] = figures[x];
			}
		}

		let colour;

		match json.colour.as_str() {
			"b" => colour = Colour::Black,
			"w" => colour = Colour::White,
			_ => panic!(),
		}
		let castling = json.castling;

		let mut code = String::new();
		for x in arr {
			for x in x {
				if x.is_some() {
					code += x.unwrap().figure.get_symbol();
				}
				else {
					code += " ";
				}
			}
		}

		Self {
			arr,
			colour,
			castling,
			figures,
			code
		}
	}

	pub fn change_mv(&mut self) {
		if self.colour == Colour::Black {
			self.colour = Colour::White;
		}
		else {
			self.colour = Colour::Black;
		}
	}

	pub fn mv(&mut self, from: (usize, usize), to: (usize, usize)) {
		self.arr[to.0][to.1] = self.arr[from.0][from.1];
		self.arr[from.0][from.1] = None;

		if self.arr[to.0][to.1].unwrap().figure.get_symbol().to_lowercase() == "p" {
			if to.1 == 0 && *self.arr[to.0][to.1].unwrap().figure.get_colour() == Colour::Black {
				self.arr[to.0][to.1] = self.figures["q"];
			}
			else if to.1 == 7 && *self.arr[to.0][to.1].unwrap().figure.get_colour() == Colour::White {
				self.arr[to.0][to.1] = self.figures["Q"];
			}
		}

		if from.1 == 0 {
			if from.0 == 0 {
				self.castling.wq = false;
			}
			else if from.0 == 4 {
				self.castling.wk = false;
				self.castling.wq = false;
			}
			else if from.0 == 7 {
				self.castling.wk = false;
			}
		}
		else if from.1 == 7 {
			if from.0 == 0 {
				self.castling.bq = false;
			}
			else if from.0 == 4 {
				self.castling.bk = false;
				self.castling.bq = false;
			}
			else if from.0 == 7 {
				self.castling.bk = false;
			}
		}
	}

	pub fn get_castling(
		&self,
	) -> Vec<(
		((usize, usize), (usize, usize)),
		((usize, usize), (usize, usize)),
	)> {
		let mut out = Vec::new();

		if self.castling.bk
			&& self.arr[6][7].is_none()
			&& self.arr[5][7].is_none()
			&& self.arr[4][7].unwrap().figure.get_colour() == &self.colour
			&& self.arr[7][7].unwrap().figure.get_colour() == &self.colour
		{
			out.push((((4, 7), (6, 7)), ((7, 7), (5, 7))));
		}
		if self.castling.bq
			&& self.arr[1][7].is_none()
			&& self.arr[2][7].is_none()
			&& self.arr[3][7].is_none()
			&& self.arr[4][7].unwrap().figure.get_colour() == &self.colour
			&& self.arr[0][7].unwrap().figure.get_colour() == &self.colour
		{
			out.push((((4, 7), (2, 7)), ((0, 7), (3, 7))));
		}
		if self.castling.wk
			&& self.arr[6][0].is_none()
			&& self.arr[5][0].is_none()
			&& self.arr[4][0].unwrap().figure.get_colour() == &self.colour
			&& self.arr[7][0].unwrap().figure.get_colour() == &self.colour
		{
			out.push((((4, 0), (6, 0)), ((7, 0), (5, 0))));
		}
		if self.castling.wq
			&& self.arr[1][0].is_none()
			&& self.arr[2][0].is_none()
			&& self.arr[3][0].is_none()
			&& self.arr[4][0].unwrap().figure.get_colour() == &self.colour
			&& self.arr[0][0].unwrap().figure.get_colour() == &self.colour
		{
			out.push((((4, 0), (2, 0)), ((0, 0), (3, 0))));
		}

		out
	}

	pub fn get_mvs(&self, from: (usize, usize)) -> Vec<(usize, usize)> {
		if self.arr[from.0][from.1].is_some()
			&& self.arr[from.0][from.1].unwrap().figure.get_colour() == &self.colour
		{
			return self.arr[from.0][from.1]
				.unwrap()
				.figure
				.get_mvs(&self.arr, from);
		}
		Vec::new()
	}

	pub fn get_val(&self) -> isize {
		let mut out = 0;

		for i in &self.arr {
			for j in i {
				if j.is_some() {
					let x = j.unwrap();
					out += isize::from(x.figure.get_colour().clone()) * x.figure.get_val();
				}
			}
		}

		out
	}

	pub fn gen_code(&mut self) {
		let mut code = String::new();
		for x in self.arr {
			for x in x {
				if x.is_some() {
					code += x.unwrap().figure.get_symbol();
				}
				else {
					code += " ";
				}
			}
		}
		self.code = code;
	}
}
