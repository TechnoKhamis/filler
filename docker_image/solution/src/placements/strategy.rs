use crate::grid::grid::{Grid, CellType};
use crate::shape::Shape;

pub fn choose_best_placement(
    grid: &Grid,
    shape: &Shape,
    valid_positions: &[(usize, usize)],
    player_id: u8
) -> Option<(usize, usize)> {
    if valid_positions.is_empty() {
        return None;
    }

    // FOR LARGE BOARDS: Use simpler, faster strategy
    if grid.rows * grid.cols > 5000 {
        return choose_best_simple(grid, shape, valid_positions);
    }

    // FOR SMALL BOARDS: Use complex strategy
    choose_best_complex(grid, shape, valid_positions)
}

// SIMPLE FAST STRATEGY (for large boards)
fn choose_best_simple(
    grid: &Grid,
    shape: &Shape,
    valid_positions: &[(usize, usize)]
) -> Option<(usize, usize)> {
    let mut best_pos = valid_positions[0];
    let mut best_score = 0;

    for &(row, col) in valid_positions {
        let mut new_cells = 0;
        
        for &(dy, dx) in &shape.cells {
            let r = row + dy;
            let c = col + dx;
            
            if grid.cells[r][c] == CellType::Empty {
                new_cells += 1;
            }
        }

        if new_cells > best_score {
            best_score = new_cells;
            best_pos = (row, col);
        }
    }

    Some(best_pos)
}

// COMPLEX STRATEGY (for small boards)
fn choose_best_complex(
    grid: &Grid,
    shape: &Shape,
    valid_positions: &[(usize, usize)]
) -> Option<(usize, usize)> {
    let enemy_positions: Vec<(usize, usize)> = find_enemy_positions(grid);

    let mut best_pos = valid_positions[0];
    let mut best_score = i64::MIN;

    for &(row, col) in valid_positions {
        let score = score_placement(grid, shape, row, col, &enemy_positions);
        if score > best_score {
            best_score = score;
            best_pos = (row, col);
        }
    }

    Some(best_pos)
}

fn find_enemy_positions(grid: &Grid) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    for r in 0..grid.rows {
        for c in 0..grid.cols {
            if grid.cells[r][c] == CellType::Enemy {
                positions.push((r, c));
            }
        }
    }
    positions
}

fn score_placement(
    grid: &Grid,
    shape: &Shape,
    row: usize,
    col: usize,
    enemy_positions: &[(usize, usize)]
) -> i64 {
    let mut score: i64 = 0;
    let mut new_cells = 0;
    let mut min_enemy_distance = usize::MAX;

    for &(dy, dx) in &shape.cells {
        let r = row + dy;
        let c = col + dx;
        
        if grid.cells[r][c] == CellType::Empty {
            new_cells += 1;
        }

        // Only check distance to closest 10 enemies (optimization)
        for &(er, ec) in enemy_positions.iter().take(10) {
            let dist = manhattan_distance(r, c, er, ec);
            if dist < min_enemy_distance {
                min_enemy_distance = dist;
            }
        }
    }

    score += new_cells * 100;
    
    if min_enemy_distance < usize::MAX {
        score += (1000 / (min_enemy_distance + 1)) as i64;
    }

    score
}

fn manhattan_distance(r1: usize, c1: usize, r2: usize, c2: usize) -> usize {
    let dr = if r1 > r2 { r1 - r2 } else { r2 - r1 };
    let dc = if c1 > c2 { c1 - c2 } else { c2 - c1 };
    dr + dc
}