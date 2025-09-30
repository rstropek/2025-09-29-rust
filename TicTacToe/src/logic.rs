#[derive(Copy, Clone, PartialEq)]
pub enum Player {
    X,
    O,
}

pub struct Playingfield {
    current_player: Player,
    grid: [[Option<Player>; 3]; 3],
}

#[derive(Debug)]
pub enum MoveError {
    CellOccupied,
}

pub enum WinnerStatus {
    NoWinner,
    Winner(Player),
    Draw,
}

impl Playingfield {
    pub fn new() -> Self {
        Self {
            current_player: Player::X,
            grid: [[None; 3]; 3],
        }
    }

    pub fn make_move(&mut self, coordinate: Coordinate) -> Result<(), MoveError> {
        // Check if cell is already occupied
        if self.grid[coordinate.row][coordinate.col].is_some() {
            return Err(MoveError::CellOccupied);
        }

        // Place the current player's mark
        self.grid[coordinate.row][coordinate.col] = Some(self.current_player);

        // Switch to the other player
        self.current_player = match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };

        Ok(())
    }

    pub fn get_winner(&self) -> WinnerStatus {
        // Check rows
        for row in 0..3 {
            if let Some(player) = self.grid[row][0] {
                if self.grid[row][1] == Some(player) && self.grid[row][2] == Some(player) {
                    return WinnerStatus::Winner(player);
                }
            }
        }

        // Check columns
        for col in 0..3 {
            if let Some(player) = self.grid[0][col] {
                if self.grid[1][col] == Some(player) && self.grid[2][col] == Some(player) {
                    return WinnerStatus::Winner(player);
                }
            }
        }

        // Check diagonal (top-left to bottom-right)
        if let Some(player) = self.grid[0][0] {
            if self.grid[1][1] == Some(player) && self.grid[2][2] == Some(player) {
                return WinnerStatus::Winner(player);
            }
        }

        // Check diagonal (top-right to bottom-left)
        if let Some(player) = self.grid[0][2] {
            if self.grid[1][1] == Some(player) && self.grid[2][0] == Some(player) {
                return WinnerStatus::Winner(player);
            }
        }

        // Check for draw (all cells filled)
        let is_full = self.grid.iter().all(|row| row.iter().all(|cell| cell.is_some()));
        if is_full {
            return WinnerStatus::Draw;
        }

        WinnerStatus::NoWinner
    }

    pub fn draw_on_console(&self) {
        println!("\n  A   B   C");
        for (row_idx, row) in self.grid.iter().enumerate() {
            print!("{} ", row_idx + 1);
            for (col_idx, cell) in row.iter().enumerate() {
                let symbol = match cell {
                    Some(Player::X) => 'X',
                    Some(Player::O) => 'O',
                    None => ' ',
                };
                if col_idx < 2 {
                    print!("{} | ", symbol);
                } else {
                    print!("{}", symbol);
                }
            }
            println!();
            if row_idx < 2 {
                println!("  ---------");
            }
        }
        println!();
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Coordinate {
    row: usize,
    col: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseCoordinateError {
    InvalidLength,
    InvalidColumn,
    InvalidRow,
    InvalidBoth,
}

impl Coordinate {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn from_str(input: &str) -> Result<Self, ParseCoordinateError> {
        // Example input: "A1", "B2", "C3"
        // Handle wrong input gracefully

        // 1. Check length, must be 2
        let bytes = input.as_bytes();
        if bytes.len() != 2 {
            return Err(ParseCoordinateError::InvalidLength);
        }
        
        // 2. First char must be A, B, or C
        // 4. Translate A/B/C to columns 0/1/2
        let col = match bytes[0] {
            b'A' => Some(0),
            b'B' => Some(1),
            b'C' => Some(2),
            _ => None,
        };

        // 3. Second char must be 1, 2, or 3
        // 5. Translate 1/2/3 to rows 0/1/2
        let row = match bytes[1] {
            b'1' => Some(0),
            b'2' => Some(1),
            b'3' => Some(2),
            _ => None,
        };

        // Check for errors, prioritizing InvalidBoth
        match (col, row) {
            (None, None) => Err(ParseCoordinateError::InvalidBoth),
            (None, _) => Err(ParseCoordinateError::InvalidColumn),
            (_, None) => Err(ParseCoordinateError::InvalidRow),
            (Some(c), Some(r)) => Ok(Self::new(r, c)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinate_from_str_all_valid() {
        // Test all 9 valid coordinates using loops
        let columns = [('A', 0), ('B', 1), ('C', 2)];
        let rows = [('1', 0), ('2', 1), ('3', 2)];
        
        for (col_char, col_idx) in columns {
            for (row_char, row_idx) in rows {
                let input = format!("{}{}", col_char, row_char);
                assert_eq!(
                    Coordinate::from_str(&input),
                    Ok(Coordinate::new(row_idx, col_idx))
                );
            }
        }
    }

    #[test]
    fn test_coordinate_from_str_all_errors() {
        // Test invalid length
        assert_eq!(Coordinate::from_str(""), Err(ParseCoordinateError::InvalidLength));
        assert_eq!(Coordinate::from_str("A"), Err(ParseCoordinateError::InvalidLength));
        assert_eq!(Coordinate::from_str("A12"), Err(ParseCoordinateError::InvalidLength));
        
        // Test invalid column
        assert_eq!(Coordinate::from_str("D1"), Err(ParseCoordinateError::InvalidColumn));
        assert_eq!(Coordinate::from_str("Z1"), Err(ParseCoordinateError::InvalidColumn));
        assert_eq!(Coordinate::from_str("a1"), Err(ParseCoordinateError::InvalidColumn));
        
        // Test both invalid column and row
        assert_eq!(Coordinate::from_str("D0"), Err(ParseCoordinateError::InvalidBoth));
        assert_eq!(Coordinate::from_str("Z9"), Err(ParseCoordinateError::InvalidBoth));

        // Test invalid row
        assert_eq!(Coordinate::from_str("A0"), Err(ParseCoordinateError::InvalidRow));
        assert_eq!(Coordinate::from_str("A4"), Err(ParseCoordinateError::InvalidRow));
        assert_eq!(Coordinate::from_str("A9"), Err(ParseCoordinateError::InvalidRow));
    }

    #[test]
    fn test_make_move() {
        let mut field = Playingfield::new();
        
        // Make a valid move
        let coord = Coordinate::new(0, 0);
        assert!(field.make_move(coord).is_ok());
        
        // Try to move to the same cell
        let coord = Coordinate::new(0, 0);
        assert!(matches!(field.make_move(coord), Err(MoveError::CellOccupied)));
    }

    #[test]
    fn test_winner_row() {
        let mut field = Playingfield::new();
        
        // X wins on first row
        field.make_move(Coordinate::new(0, 0)).unwrap(); // X
        field.make_move(Coordinate::new(1, 0)).unwrap(); // O
        field.make_move(Coordinate::new(0, 1)).unwrap(); // X
        field.make_move(Coordinate::new(1, 1)).unwrap(); // O
        field.make_move(Coordinate::new(0, 2)).unwrap(); // X
        
        assert!(matches!(field.get_winner(), WinnerStatus::Winner(Player::X)));
    }

    #[test]
    fn test_winner_column() {
        let mut field = Playingfield::new();
        
        // O wins on first column
        field.make_move(Coordinate::new(0, 1)).unwrap(); // X
        field.make_move(Coordinate::new(0, 0)).unwrap(); // O
        field.make_move(Coordinate::new(1, 1)).unwrap(); // X
        field.make_move(Coordinate::new(1, 0)).unwrap(); // O
        field.make_move(Coordinate::new(0, 2)).unwrap(); // X
        field.make_move(Coordinate::new(2, 0)).unwrap(); // O
        
        assert!(matches!(field.get_winner(), WinnerStatus::Winner(Player::O)));
    }

    #[test]
    fn test_winner_diagonal() {
        let mut field = Playingfield::new();
        
        // X wins on diagonal
        field.make_move(Coordinate::new(0, 0)).unwrap(); // X
        field.make_move(Coordinate::new(0, 1)).unwrap(); // O
        field.make_move(Coordinate::new(1, 1)).unwrap(); // X
        field.make_move(Coordinate::new(0, 2)).unwrap(); // O
        field.make_move(Coordinate::new(2, 2)).unwrap(); // X
        
        assert!(matches!(field.get_winner(), WinnerStatus::Winner(Player::X)));
    }

    #[test]
    fn test_draw() {
        let mut field = Playingfield::new();
        
        // Fill the board with no winner
        field.make_move(Coordinate::new(0, 0)).unwrap(); // X
        field.make_move(Coordinate::new(0, 1)).unwrap(); // O
        field.make_move(Coordinate::new(0, 2)).unwrap(); // X
        field.make_move(Coordinate::new(1, 0)).unwrap(); // O
        field.make_move(Coordinate::new(1, 1)).unwrap(); // X
        field.make_move(Coordinate::new(2, 0)).unwrap(); // O
        field.make_move(Coordinate::new(1, 2)).unwrap(); // X
        field.make_move(Coordinate::new(2, 2)).unwrap(); // O
        field.make_move(Coordinate::new(2, 1)).unwrap(); // X
        
        assert!(matches!(field.get_winner(), WinnerStatus::Draw));
    }
}