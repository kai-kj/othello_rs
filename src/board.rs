use crate::prelude::*;

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

pub struct Board {
    pub to_play: Player,
    pub black: BitBoard,
    pub white: BitBoard,
}

impl Board {
    pub fn new() -> Board {
        Board {
            to_play: Player::Black,
            black: bb!(
                0 0 0 0 0 0 0 0,
                0 0 0 0 0 0 0 0,
                0 0 0 0 0 0 0 0,
                0 0 0 0 1 0 0 0,
                0 0 0 1 0 0 0 0,
                0 0 0 0 0 0 0 0,
                0 0 0 0 0 0 0 0,
                0 0 0 0 0 0 0 0,
            ),
            white: bb!(
                0 0 0 0 0 0 0 0,
                0 0 0 0 0 0 0 0,
                0 0 0 0 0 0 0 0,
                0 0 0 1 0 0 0 0,
                0 0 0 0 1 0 0 0,
                0 0 0 0 0 0 0 0,
                0 0 0 0 0 0 0 0,
                0 0 0 0 0 0 0 0,
            ),
        }
    }

    pub fn get(&self, position: Position) -> Option<Player> {
        match (self.black.get(position), self.white.get(position)) {
            (false, false) => None,
            (true, false) => Some(Player::Black),
            (false, true) => Some(Player::White),
            (true, true) => panic!("{} is occupied by both players", position),
        }
    }

    pub fn set(&mut self, position: Position) {
        let pos = 1u64 << ((7 - position.row) * 8 + position.col);

        let (mut me, mut op) = match self.to_play {
            Player::Black => (self.black, self.white),
            Player::White => (self.white, self.black),
        };

        let mut flips: u64 = 0;

        macro_rules! ray {
            ($shift:expr, $mask:expr) => {{
                let mut x = $shift(pos) & $mask;
                let mut f = 0u64;

                while x != 0u64 && (x & op) != 0u64 {
                    f |= x;
                    x = $shift(x) & $mask;
                }

                if (x & me) != 0u64 {
                    flips |= f;
                }
            }};
        }

        ray!(|b| b << 1, 0x7f7f7f7f7f7f7f7fu64);
        ray!(|b| b >> 1, 0xfefefefefefefefeu64);
        ray!(|b| b << 8, 0xffffffffffffffffu64);
        ray!(|b| b >> 8, 0xffffffffffffffffu64);
        ray!(|b| b << 9, 0x7f7f7f7f7f7f7f7fu64);
        ray!(|b| b << 7, 0xfefefefefefefefeu64);
        ray!(|b| b >> 7, 0x7f7f7f7f7f7f7f7fu64);
        ray!(|b| b >> 9, 0xfefefefefefefefeu64);

        me |= pos | flips;
        op &= !flips;

        match self.to_play {
            Player::Black => {
                self.black = me;
                self.white = op;
                self.to_play = Player::White;
            }
            Player::White => {
                self.white = me;
                self.black = op;
                self.to_play = Player::Black;
            }
        }
    }

    pub fn legal(&self) -> BitBoard {
        let (me, op) = match self.to_play {
            Player::Black => (self.black, self.white),
            Player::White => (self.white, self.black),
        };

        let empty = !(me | op);
        let mut moves = 0u64;

        macro_rules! ray {
            ($shift:expr, $mask:expr) => {{
                let mut x = $shift(me) & $mask & op;

                while x != 0u64 {
                    let next = $shift(x) & $mask;
                    let candidates = next & empty;

                    moves |= candidates;

                    x = next & op;
                }
            }};
        }

        ray!(|b| b << 1, 0x7f7f7f7f7f7f7f7fu64);
        ray!(|b| b >> 1, 0xfefefefefefefefeu64);
        ray!(|b| b << 8, 0xffffffffffffffffu64);
        ray!(|b| b >> 8, 0xffffffffffffffffu64);
        ray!(|b| b << 9, 0x7f7f7f7f7f7f7f7fu64);
        ray!(|b| b << 7, 0xfefefefefefefefeu64);
        ray!(|b| b >> 7, 0x7f7f7f7f7f7f7f7fu64);
        ray!(|b| b >> 9, 0xfefefefefefefefeu64);

        moves
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Board({}, {:x}, {:x})",
            self.to_play, self.black, self.white
        )
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Board(")?;
        writeln!(f, "  {} to play", self.to_play)?;
        writeln!(f, "     a b c d e f g h")?;

        let legal = self.legal();

        for i in 0..8 {
            write!(f, "  {} \x1B[42m", i + 1)?;
            for j in 0..8 {
                match self.get(Position::index(i, j)) {
                    Some(player) => write!(f, "{} ●\x1B[39m", player.ascii_color_code())?,
                    None => {
                        if legal.get(Position::index(i, j)) {
                            write!(f, "{} •\x1B[39m", self.to_play.ascii_color_code())?
                        } else {
                            write!(f, "\x1B[37m •\x1B[39m")?
                        }
                    }
                }
            }
            writeln!(f, "\x1B[49m")?;
        }

        writeln!(f, ")")?;
        Ok(())
    }
}
