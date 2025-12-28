use crate::grid::grid::{Grid, CellType};
use crate::shape::Shape;

pub fn find_valid_placements(
    grid: &Grid,
    shape: &Shape,
    player_id: u8
) -> Vec<(usize, usize)> {
    let mut valid_positions: Vec<(usize, usize)> = Vec::new();

    // OPTIMIZATION: Find player's territory first
    let mut my_positions: Vec<(usize, usize)> = Vec::new();
    for r in 0..grid.rows {
        for c in 0..grid.cols {
            if grid.cells[r][c] == CellType::Mine {
                my_positions.push((r, c));
            }
        }
    }

    // CRITICAL FIX: Only search within radius of existing territory
    let search_radius = 10; // Adjust this if needed
    let mut candidates: Vec<(usize, usize)> = Vec::new();

    for &(my_r, my_c) in &my_positions {
        for dr in -(search_radius as isize)..=(search_radius as isize) {
            for dc in -(search_radius as isize)..=(search_radius as isize) {
                let r = (my_r as isize + dr).max(0) as usize;
                let c = (my_c as isize + dc).max(0) as usize;

                // Check if shape fits
                if r + shape.height <= grid.rows && c + shape.width <= grid.cols {
                    candidates.push((r, c));
                }
            }
        }
    }

    // Remove duplicates
    candidates.sort_unstable();
    candidates.dedup();

    // Now check only these candidates
    for (row, col) in candidates {
        if is_valid_placement(grid, shape, row, col, player_id) {
            valid_positions.push((row, col));
        }
    }

    valid_positions
}

fn is_valid_placement(
    grid: &Grid,
    shape: &Shape,
    top_row: usize,
    left_col: usize,
    player_id: u8
) -> bool {
    let mut my_overlap_count = 0;

    // Check each cell of the shape
    for &(dy, dx) in &shape.cells {
        let row = top_row + dy;
        let col = left_col + dx;

        // Check bounds
        if row >= grid.rows || col >= grid.cols {
            return false;
        }

        // Check what's at this position
        match grid.cells[row][col] {
            CellType::Enemy => {
                return false;  // Touching enemy!
            }
            CellType::Mine => {
                my_overlap_count += 1;
                if my_overlap_count > 1 {
                    return false;  // Too many overlaps!
                }
            }
            CellType::Empty => {
                // Empty is fine
            }
        }
    }

    // Must overlap EXACTLY 1
    my_overlap_count == 1
}