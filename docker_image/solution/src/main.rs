// src/main.rs

mod input;
mod grid;
mod shape;
mod placements;

use std::io::{self, BufRead, Write};
use crate::grid::grid::{Grid, CellType};
use crate::shape::Shape;
use crate::placements::validator::find_valid_placements;
use crate::placements::strategy::choose_best_placement;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 1) Detect which player we are
    let player_id = loop {
        match lines.next() {
            Some(Ok(line)) => {
                if let Some(num) = input::reader::get_player_id(&line) {
                    break num;
                }
                // Ignore unrelated lines until we find the exec line
            }
            _ => {
                // No input
                return;
            }
        }
    };

    // 2) Main game loop: each iteration = one turn
    'game_loop: loop {
        // Collect Anfield block
        let mut board_lines: Vec<String> = Vec::new();

        // Find "Anfield" header
        let header = loop {
            match lines.next() {
                Some(Ok(line)) => {
                    if line.trim_start().starts_with("Anfield") {
                        break line;
                    }
                    // Ignore other lines until we see Anfield
                }
                _ => {
                    // No more data, game over
                    return;
                }
            }
        };

        board_lines.push(header.clone());

        // Read until we see "Piece" header
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
                    // EOF before piece, stop
                    return;
                }
            }
        }

        // Parse the board from the collected lines
        let grid = match Grid::from_lines(&board_lines, player_id) {
            Some(g) => g,
            None => break 'game_loop,
        };

        // Collect piece block: header + height lines
        let mut piece_lines: Vec<String> = Vec::new();
        piece_lines.push(piece_header.clone());

        let height = {
            let trimmed = piece_header.trim();
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 3 {
                parts[2]
                    .trim_end_matches(':')
                    .parse::<usize>()
                    .unwrap_or(0)
            } else {
                0
            }
        };

        for _ in 0..height {
            match lines.next() {
                Some(Ok(line)) => piece_lines.push(line),
                _ => {
                    // Incomplete piece, stop the game
                    break 'game_loop;
                }
            }
        }

        let shape = match Shape::from_lines(&piece_lines) {
            Some(s) => s,
            None => break 'game_loop,
        };

        // Find valid placements and choose best
        let valid_positions = find_valid_placements(&grid, &shape, player_id);

        let (out_row, out_col) = match choose_best_placement(&grid, &shape, &valid_positions, player_id) {
            Some((y, x)) => {
                eprintln!("[DEBUG] Found placement at row={}, col={}", y, x);
                (y, x)
            }
            None => {
                eprintln!("[DEBUG] No valid placement found! Board: {}x{}, Piece: {}x{}", 
                    grid.rows, grid.cols, shape.height, shape.width);
                eprintln!("[DEBUG] Piece cells: {} filled", shape.cells.len());
                eprintln!("[DEBUG] My territory cells: {}", 
                    grid.cells.iter().flatten().filter(|&&c| c == CellType::Mine).count());
                (0usize, 0usize) // fallback if no valid placement
            }
        };

        // Output in "X Y" format where X=column, Y=row
        println!("{} {}", out_col, out_row);
        let _ = io::stdout().flush();
    }
}