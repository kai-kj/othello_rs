use super::prelude::*;
use std::fmt::Write;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Board {
    pub black: BitBoard,
    pub white: BitBoard,
    pub to_play: Player,
}

impl Board {
    pub fn new(black: BitBoard, white: BitBoard, to_play: Player) -> Board {
        Board {
            black,
            white,
            to_play,
        }
    }

    pub fn start() -> Board {
        Board::new(
            bb!(
                0 0 0 0 0 0 0 0,
                0 0 0 0 0 0 0 0,
                0 0 0 0 0 0 0 0,
                0 0 0 0 1 0 0 0,
                0 0 0 1 0 0 0 0,
                0 0 0 0 0 0 0 0,
                0 0 0 0 0 0 0 0,
                0 0 0 0 0 0 0 0,
            ),
            bb!(
                0 0 0 0 0 0 0 0,
                0 0 0 0 0 0 0 0,
                0 0 0 0 0 0 0 0,
                0 0 0 1 0 0 0 0,
                0 0 0 0 1 0 0 0,
                0 0 0 0 0 0 0 0,
                0 0 0 0 0 0 0 0,
                0 0 0 0 0 0 0 0,
            ),
            Player::Black,
        )
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

        Self::s_ray(me, op, 0x7f7f7f7f7f7f7f7fu64, pos, |b| b << 1, &mut flips);
        Self::s_ray(me, op, 0xfefefefefefefefeu64, pos, |b| b >> 1, &mut flips);
        Self::s_ray(me, op, 0xffffffffffffffffu64, pos, |b| b << 8, &mut flips);
        Self::s_ray(me, op, 0xffffffffffffffffu64, pos, |b| b >> 8, &mut flips);
        Self::s_ray(me, op, 0x7f7f7f7f7f7f7f7fu64, pos, |b| b << 9, &mut flips);
        Self::s_ray(me, op, 0xfefefefefefefefeu64, pos, |b| b << 7, &mut flips);
        Self::s_ray(me, op, 0x7f7f7f7f7f7f7f7fu64, pos, |b| b >> 7, &mut flips);
        Self::s_ray(me, op, 0xfefefefefefefefeu64, pos, |b| b >> 9, &mut flips);

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

        Self::l_ray(me, op, 0x7f7f7f7f7f7f7f7fu64, empty, |b| b << 1, &mut moves);
        Self::l_ray(me, op, 0xfefefefefefefefeu64, empty, |b| b >> 1, &mut moves);
        Self::l_ray(me, op, 0xffffffffffffffffu64, empty, |b| b << 8, &mut moves);
        Self::l_ray(me, op, 0xffffffffffffffffu64, empty, |b| b >> 8, &mut moves);
        Self::l_ray(me, op, 0x7f7f7f7f7f7f7f7fu64, empty, |b| b << 9, &mut moves);
        Self::l_ray(me, op, 0xfefefefefefefefeu64, empty, |b| b << 7, &mut moves);
        Self::l_ray(me, op, 0x7f7f7f7f7f7f7f7fu64, empty, |b| b >> 7, &mut moves);
        Self::l_ray(me, op, 0xfefefefefefefefeu64, empty, |b| b >> 9, &mut moves);

        moves
    }

    pub fn count_pieces(&self, player: Player) -> usize {
        let me = match player {
            Player::Black => self.black,
            Player::White => self.white,
        };

        me.count_ones() as usize
    }

    pub fn pretty(&self, indent: &str) -> Result<String, std::fmt::Error> {
        let mut output = String::new();

        writeln!(output, "{}   a b c d e f g h", indent)?;

        let legal = self.legal();

        for i in 0..8 {
            write!(output, "{}{} \x1B[42m", indent, i + 1)?;

            for j in 0..8 {
                match self.get(Position::index(i, j)) {
                    Some(player) => {
                        write!(output, "{} ●\x1B[39m", player.ascii_color_code())?;
                    }
                    None => {
                        if legal.get(Position::index(i, j)) {
                            write!(output, "{} •\x1B[39m", self.to_play.ascii_color_code())?;
                        } else {
                            write!(output, "\x1B[37m •\x1B[39m")?;
                        }
                    }
                }
            }

            write!(output, "\x1B[49m")?;

            if i < 7 {
                writeln!(output)?;
            }
        }

        Ok(output)
    }

    fn s_ray(me: u64, op: u64, mask: u64, pos: u64, shift: impl Fn(u64) -> u64, flips: &mut u64) {
        let mut x = shift(pos) & mask;
        let mut f = 0u64;

        while x != 0u64 && (x & op) != 0u64 {
            f |= x;
            x = shift(x) & mask;
        }

        if (x & me) != 0u64 {
            *flips |= f;
        }
    }

    fn l_ray(me: u64, op: u64, mask: u64, empty: u64, shift: impl Fn(u64) -> u64, moves: &mut u64) {
        let mut x = shift(me) & mask & op;

        while x != 0u64 {
            let next = shift(x) & mask;
            let candidates = next & empty;

            *moves |= candidates;

            x = next & op;
        }
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
        writeln!(f, "{}", self.pretty("  ")?)?;
        writeln!(f, ")")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set() {
        let mut board = Board::start();

        assert_eq!(
            board,
            Board::new(
                bb!(
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 1 0 0 0,
                    0 0 0 1 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                ),
                bb!(
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 0 1 0 0 0 0,
                    0 0 0 0 1 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                ),
                Player::Black,
            )
        );

        board.set(Position::parse("c4").unwrap());

        assert_eq!(
            board,
            Board::new(
                bb!(
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 1 1 1 0 0 0,
                    0 0 0 1 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                ),
                bb!(
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 1 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                ),
                Player::White,
            )
        );

        board.set(Position::parse("c3").unwrap());

        assert_eq!(
            board,
            Board::new(
                bb!(
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 1 0 1 0 0 0,
                    0 0 0 1 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                ),
                bb!(
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 1 0 0 0 0 0,
                    0 0 0 1 0 0 0 0,
                    0 0 0 0 1 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                ),
                Player::Black,
            )
        );

        board.set(Position::parse("f5").unwrap());

        assert_eq!(
            board,
            Board::new(
                bb!(
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 1 0 1 0 0 0,
                    0 0 0 1 1 1 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                ),
                bb!(
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 1 0 0 0 0 0,
                    0 0 0 1 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                    0 0 0 0 0 0 0 0,
                ),
                Player::White,
            )
        );
    }

    #[test]
    fn test_legal() {
        let mut board = Board::start();

        assert_eq!(
            board.legal(),
            bb!(
                0 0 0 0 0 0 0 0,
                0 0 0 0 0 0 0 0,
                0 0 0 1 0 0 0 0,
                0 0 1 0 0 0 0 0,
                0 0 0 0 0 1 0 0,
                0 0 0 0 1 0 0 0,
                0 0 0 0 0 0 0 0,
                0 0 0 0 0 0 0 0,
            ),
        );

        board.set(Position::parse("c4").unwrap());

        assert_eq!(
            board.legal(),
            bb!(
                0 0 0 0 0 0 0 0,
                0 0 0 0 0 0 0 0,
                0 0 1 0 1 0 0 0,
                0 0 0 0 0 0 0 0,
                0 0 1 0 0 0 0 0,
                0 0 0 0 0 0 0 0,
                0 0 0 0 0 0 0 0,
                0 0 0 0 0 0 0 0,
            ),
        );

        board.set(Position::parse("c3").unwrap());

        assert_eq!(
            board.legal(),
            bb!(
                0 0 0 0 0 0 0 0,
                0 0 1 0 0 0 0 0,
                0 0 0 1 0 0 0 0,
                0 0 0 0 0 0 0 0,
                0 0 0 0 0 1 0 0,
                0 0 0 0 1 0 0 0,
                0 0 0 0 0 0 0 0,
                0 0 0 0 0 0 0 0,
            ),
        );

        board.set(Position::parse("f5").unwrap());

        assert_eq!(
            board.legal(),
            bb!(
                0 0 0 0 0 0 0 0,
                0 0 0 0 0 0 0 0,
                0 0 0 0 0 0 0 0,
                0 1 0 0 0 1 0 0,
                0 0 1 0 0 0 0 0,
                0 0 0 1 0 1 0 0,
                0 0 0 0 0 0 0 0,
                0 0 0 0 0 0 0 0,
            ),
        );
    }
}
