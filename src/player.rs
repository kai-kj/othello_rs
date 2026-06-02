#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Player {
    Black,
    White,
}

impl Player {
    pub fn ascii_color_code(&self) -> &'static str {
        match self {
            Player::Black => "\x1B[30m",
            Player::White => "\x1B[97m",
        }
    }
}

impl std::fmt::Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::Black => write!(f, "Player::Black"),
            Player::White => write!(f, "Player::White"),
        }
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::Black => write!(f, "black"),
            Player::White => write!(f, "white"),
        }
    }
}
