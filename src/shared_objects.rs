use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl std::ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, position: Position) -> Position {
        Position {
            x: self.x + position.x,
            y: self.y + position.y,
        }
    }
}
impl std::ops::Sub<Position> for Position {
    type Output = Position;

    fn sub(self, position: Position) -> Position {
        self + -position
    }
}
impl std::ops::Neg for Position {
    type Output = Position;

    fn neg(self) -> Position {
        Position {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn next(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    pub fn offset(self) -> Position {
        match self {
            Direction::Up => Position { x: 0, y: -1 },
            Direction::Down => Position { x: 0, y: 1 },
            Direction::Left => Position { x: -1, y: 0 },
            Direction::Right => Position { x: 1, y: 0 },
        }
    }
    pub fn all() -> Vec<Direction> {
        vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }
}

impl std::ops::Add<Direction> for Position {
    type Output = Position;

    /// Adds a direction to the current position
    fn add(self, direction: Direction) -> Position {
        let offset = direction.offset();
        Position {
            x: self.x + offset.x,
            y: self.y + offset.y,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SparseGrid {
    pub entries: HashMap<Position, char>,
    pub max_row: i32,
    pub max_col: i32,
}

impl SparseGrid {
    pub fn new(max_row: i32, max_col: i32) -> Self {
        Self {
            entries: HashMap::new(),
            max_row,
            max_col,
        }
    }
    pub fn is_inside(&self, pos: &Position) -> bool {
        pos.x >= 0 && pos.x < self.max_col && pos.y >= 0 && pos.y < self.max_row
    }
    pub fn is_free_tile(&self, pos: &Position) -> bool {
        !self.entries.contains_key(pos) && self.is_inside(pos)
    }

    pub fn set(&mut self, pos: Position, tile: char) {
        self.entries.insert(pos, tile);
    }

    pub fn from_input_data(input_data: &Vec<Vec<char>>) -> Self {
        let max_row = input_data.len() as i32;
        let max_col = input_data[0].len() as i32;
        let mut sparse_grid = SparseGrid::new(max_row, max_col);
        for y in 0..max_row {
            for x in 0..max_col {
                let tile = input_data[y as usize][x as usize] as char;
                if tile != '.' {
                    sparse_grid.set(Position { x, y }, tile);
                }
            }
        }
        sparse_grid
    }

    pub fn get_unique_entries(&self) -> Vec<char> {
        let mut unique_entries = vec![];
        for (_, &tile) in &self.entries {
            if !unique_entries.contains(&tile) {
                unique_entries.push(tile);
            }
        }
        unique_entries
    }

    pub fn get_positions(&self, tile: char) -> Vec<Position> {
        self.entries
            .iter()
            .filter(|(_, &t)| t == tile)
            .map(|(&pos, _)| pos)
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub data: Vec<Vec<T>>,
    pub width: usize,
    pub height: usize,
}

impl<T: Copy> Grid<T> {
    pub fn new(width: usize, height: usize, default_value: T) -> Self {
        let mut data = vec![];
        for _ in 0..height {
            let mut row = vec![];
            for _ in 0..width {
                row.push(default_value);
            }
            data.push(row);
        }
        Self {
            data,
            width,
            height,
        }
    }

    pub fn get(&self, pos: &Position) -> Option<&T> {
        if pos.x < 0 || pos.x >= self.width as i32 || pos.y < 0 || pos.y >= self.height as i32 {
            None
        } else {
            Some(&self.data[pos.y as usize][pos.x as usize])
        }
    }

    pub fn set(&mut self, pos: &Position, value: T) {
        if pos.x >= 0 && pos.x < self.width as i32 && pos.y >= 0 && pos.y < self.height as i32 {
            self.data[pos.y as usize][pos.x as usize] = value;
        }
    }
    pub fn set_xy(&mut self, pos_x: usize, pos_y: usize, value: T) {
        if pos_x < self.width && pos_y < self.height {
            self.data[pos_y][pos_x] = value;
        }
    }
}
