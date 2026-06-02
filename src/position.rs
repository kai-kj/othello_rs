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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(Position::parse("a1").unwrap(), Position::index(0, 0));
        assert_eq!(Position::parse("a8").unwrap(), Position::index(7, 0));
        assert_eq!(Position::parse("h1").unwrap(), Position::index(0, 7));
        assert_eq!(Position::parse("h8").unwrap(), Position::index(7, 7));

        assert_eq!(Position::parse("d4").unwrap(), Position::index(3, 3));
        assert_eq!(Position::parse("d5").unwrap(), Position::index(4, 3));
        assert_eq!(Position::parse("e4").unwrap(), Position::index(3, 4));
        assert_eq!(Position::parse("e5").unwrap(), Position::index(4, 4));
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Position::index(0, 0)), "a1");
        assert_eq!(format!("{}", Position::index(7, 0)), "a8");
        assert_eq!(format!("{}", Position::index(0, 7)), "h1");
        assert_eq!(format!("{}", Position::index(7, 7)), "h8");

        assert_eq!(format!("{}", Position::index(3, 3)), "d4");
        assert_eq!(format!("{}", Position::index(4, 3)), "d5");
        assert_eq!(format!("{}", Position::index(3, 4)), "e4");
        assert_eq!(format!("{}", Position::index(4, 4)), "e5");
    }
}
