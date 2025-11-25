use serde::{Serialize, Deserialize};
use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub enum Piece {
  King,
  Queen,
  Bishop,
  Knight,
  Rook,
}

static STANDARD_ROW: [Piece; 8] = [
  Piece::Rook,
  Piece::Knight,
  Piece::Bishop,
  Piece::Queen,
  Piece::King,
  Piece::Bishop,
  Piece::Knight,
  Piece::Rook
];

fn check_if_array_is_valid(array: &[Piece; 8]) -> bool {
  let mut n_of_rook_before_king = 0;
  for piece in (*array).iter() {
    if let Piece::Rook = *piece {
      n_of_rook_before_king += 1;
    }
    if let Piece::King = *piece {
      return n_of_rook_before_king == 1;
    }
  }
  false
}

pub fn rand_pieces() -> [Piece; 8] {
    // Get a thread-local random number generator (Rng)
    let mut rng = thread_rng();

    let mut row = STANDARD_ROW.clone();

    loop {
      row.shuffle(&mut rng);
      if check_if_array_is_valid(&row) {
        break;
      }
    }
    row
}