use std::fmt::Write;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn index(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn parse(s: &str) -> Option<Self> {
        let col = s.chars().nth(0)?.to_ascii_lowercase() as i32 - 'a' as i32;
        let row = s.chars().nth(1)?.to_ascii_lowercase() as i32 - '1' as i32;

        if col >= 0 && col < 8 && row >= 0 && row < 8 {
            Some(Self::index(row as usize, col as usize))
        } else {
            None
        }
    }
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Move({}, {})", self.row, self.col)
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            ('a' as i32 + self.col as i32) as u8 as char,
            ('1' as i32 + self.row as i32) as u8 as char,
        )
    }
}

pub type BitBoard = u64;

pub trait BitBoardExt {
    fn get(&self, position: Position) -> bool;
    fn pretty(&self) -> String;
}

impl BitBoardExt for BitBoard {
    fn get(&self, position: Position) -> bool {
        (self >> ((7 - position.row) * 8 + position.col)) & 1 != 0
    }

    fn pretty(&self) -> String {
        let mut output = String::new();

        writeln!(output, "BitBoard(").unwrap();

        for i in 0..8 {
            let line = self >> ((7 - i) * 8) & 0xff;
            let line = format!("{:08b}", (line as u8).reverse_bits());
            writeln!(
                output,
                "  {},",
                line.split("").collect::<Vec<_>>().join(" ").trim()
            )
            .unwrap();
        }

        writeln!(output, ")").unwrap();

        output
    }
}

macro_rules! bb {
    (
        $r0c0:tt $r0c1:tt $r0c2:tt $r0c3:tt $r0c4:tt $r0c5:tt $r0c6:tt $r0c7:tt,
        $r1c0:tt $r1c1:tt $r1c2:tt $r1c3:tt $r1c4:tt $r1c5:tt $r1c6:tt $r1c7:tt,
        $r2c0:tt $r2c1:tt $r2c2:tt $r2c3:tt $r2c4:tt $r2c5:tt $r2c6:tt $r2c7:tt,
        $r3c0:tt $r3c1:tt $r3c2:tt $r3c3:tt $r3c4:tt $r3c5:tt $r3c6:tt $r3c7:tt,
        $r4c0:tt $r4c1:tt $r4c2:tt $r4c3:tt $r4c4:tt $r4c5:tt $r4c6:tt $r4c7:tt,
        $r5c0:tt $r5c1:tt $r5c2:tt $r5c3:tt $r5c4:tt $r5c5:tt $r5c6:tt $r5c7:tt,
        $r6c0:tt $r6c1:tt $r6c2:tt $r6c3:tt $r6c4:tt $r6c5:tt $r6c6:tt $r6c7:tt,
        $r7c0:tt $r7c1:tt $r7c2:tt $r7c3:tt $r7c4:tt $r7c5:tt $r7c6:tt $r7c7:tt,
    ) => {{
        paste::paste! {
            [<
                0b0
                $r0c7 $r0c6 $r0c5 $r0c4 $r0c3 $r0c2 $r0c1 $r0c0
                $r1c7 $r1c6 $r1c5 $r1c4 $r1c3 $r1c2 $r1c1 $r1c0
                $r2c7 $r2c6 $r2c5 $r2c4 $r2c3 $r2c2 $r2c1 $r2c0
                $r3c7 $r3c6 $r3c5 $r3c4 $r3c3 $r3c2 $r3c1 $r3c0
                $r4c7 $r4c6 $r4c5 $r4c4 $r4c3 $r4c2 $r4c1 $r4c0
                $r5c7 $r5c6 $r5c5 $r5c4 $r5c3 $r5c2 $r5c1 $r5c0
                $r6c7 $r6c6 $r6c5 $r6c4 $r6c3 $r6c2 $r6c1 $r6c0
                $r7c7 $r7c6 $r7c5 $r7c4 $r7c3 $r7c2 $r7c1 $r7c0
                u64
            >]
        }
    }};
}

pub(crate) use bb;

#[cfg(test)]
mod tests {}
