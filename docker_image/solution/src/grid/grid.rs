// src/grid/grid.rs

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellType {
    Empty,
    Mine,
    Enemy,
}

pub struct Grid {
    pub rows: usize,
    pub cols: usize,
    pub cells: Vec<Vec<CellType>>,
}

impl Grid {
    pub fn from_lines(lines: &[String], player_id : u8) -> Option<Self> {
        if lines.is_empty() {
            return None;
        }

        let mut grid: Vec<Vec<CellType>> = Vec::new();
        let mut found_header = false;

        for line in lines {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                continue;
            }

            // Skip header but mark that we've seen it
            if trimmed.starts_with("Anfield") {
                found_header = true;
                continue;
            }

            if !found_header {
                continue;
            }

            // Skip column number lines
            if trimmed.chars().all(|c| c.is_ascii_digit() || c.is_whitespace()) {
                continue;
            }

            // Parse the row (skip row number prefix)
            let row_str: String = line
                .chars()
                .skip_while(|c| c.is_ascii_digit() || c.is_whitespace())
                .collect();

            if row_str.is_empty() {
                continue;
            }

            let mut row: Vec<CellType> = Vec::new();

            for ch in row_str.chars() {
                if ch == ' ' {
                    continue;
                }
                let cell = classify_cell(ch, player_id );
                row.push(cell);
            }

            if !row.is_empty() {
                grid.push(row);
            }
        }

        if grid.is_empty() {
            return None;
        }

        let rows = grid.len();
        let cols = grid[0].len();

        Some(Grid { rows, cols, cells: grid })
    }
}

fn classify_cell(ch: char, player_id : u8) -> CellType {
    match ch {
        '.' => CellType::Empty,

        '@' | 'a' => {
            if player_id  == 1 {
                CellType::Mine
            } else {
                CellType::Enemy
            }
        }

        '$' | 's' => {
            if player_id  == 2 {
                CellType::Mine
            } else {
                CellType::Enemy
            }
        }

        _ => CellType::Empty,
    }
}