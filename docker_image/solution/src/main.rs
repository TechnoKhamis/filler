mod input;
mod grid;
mod shape;
mod placements;

use std::io::{self, BufRead, Write};
use crate::grid::grid::Grid;
use crate::shape::Shape;
use crate::placements::validator::find_valid_placements;
use crate::placements::strategy::choose_best_placement;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 1) Read player number
    let player_id = loop {
        match lines.next() {
            Some(Ok(line)) => {
                if let Some(num) = input::reader::get_player_id(&line) {
                    break num;
                }
            }
            _ => {
                return;
            }
        }
    };

    // 2) Main game loop
    loop {
        // Collect board lines
        let mut board_lines: Vec<String> = Vec::new();

        // Find "Anfield" header
        let header = loop {
            match lines.next() {
                Some(Ok(line)) => {
                    if line.trim_start().starts_with("Anfield") {
                        break line;
                    }
                }
                _ => {
                    return;
                }
            }
        };

        board_lines.push(header);

        // Read until "Piece"
        let piece_header: String;
        loop {
            match lines.next() {
                Some(Ok(line)) => {
                    if line.trim_start().starts_with("Piece") {
                        piece_header = line;
                        break;
                    } else {
                        board_lines.push(line);
                    }
                }
                _ => {
                    return;
                }
            }
        }

        // Parse the grid
        let grid = match Grid::from_lines(&board_lines, player_id) {
            Some(g) => g,
            None => {
                return;
            }
        };

        // Collect piece lines
        let mut piece_lines: Vec<String> = Vec::new();
        piece_lines.push(piece_header.clone());

        // Parse height from header
        let height = {
            let parts: Vec<&str> = piece_header.split_whitespace().collect();
            if parts.len() >= 3 {
                parts[2]
                    .trim_end_matches(':')
                    .parse::<usize>()
                    .unwrap_or(0)
            } else {
                0
            }
        };

        // Read exactly 'height' more lines
        for _ in 0..height {
            match lines.next() {
                Some(Ok(line)) => piece_lines.push(line),
                _ => {
                    return;
                }
            }
        }

        // Parse the piece
        let shape = match Shape::from_lines(&piece_lines) {
            Some(s) => s,
            None => {
                return;
            }
        };

        // ========== FIND VALID PLACEMENTS ==========
        
        let valid_positions = find_valid_placements(&grid, &shape, player_id);

        // ========== CHOOSE BEST PLACEMENT ==========
        
        let (best_row, best_col) = match choose_best_placement(&grid, &shape, &valid_positions, player_id) {
            Some(pos) => pos,
            None => {
                (0, 0)  // Fallback
            }
        };

        // ========== OUTPUT MOVE ==========
        
        println!("{} {}", best_col, best_row);
        io::stdout().flush().unwrap();
    }
}