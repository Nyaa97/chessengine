use serde::Deserialize;

use crate::Castling;

#[derive(Deserialize)]
pub struct Config {
	pub colour: String,
	pub castling: Castling,
	pub arr: [[String; 8]; 8],
}
