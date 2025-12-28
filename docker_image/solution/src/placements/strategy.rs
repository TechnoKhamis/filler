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

    // ALWAYS use fast strategy to avoid timeout
    choose_best_simple(grid, shape, valid_positions)
}

// SIMPLE FAST STRATEGY
fn choose_best_simple(
    grid: &Grid,
    shape: &Shape,
    valid_positions: &[(usize, usize)]
) -> Option<(usize, usize)> {
    let enemy_positions: Vec<(usize, usize)> = find_enemy_positions(grid);
    
    let mut best_pos = valid_positions[0];
    let mut best_score = i64::MIN;

    for &(row, col) in valid_positions {
        let mut new_cells = 0;
        let mut min_enemy_dist = usize::MAX;
        
        for &(dy, dx) in &shape.cells {
            let r = row + dy;
            let c = col + dx;
            
            if grid.cells[r][c] == CellType::Empty {
                new_cells += 1;
            }

            // Find closest enemy (only check first 5 enemies for speed)
            for &(er, ec) in enemy_positions.iter().take(5) {
                let dist = manhattan_distance(r, c, er, ec);
                if dist < min_enemy_dist {
                    min_enemy_dist = dist;
                }
            }
        }

        // Scoring: prioritize getting close to enemy, then territory
        let mut score = new_cells as i64 * 100;
        
        if min_enemy_dist < usize::MAX {
            score += (10000 / (min_enemy_dist + 1)) as i64;
        }

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

fn manhattan_distance(r1: usize, c1: usize, r2: usize, c2: usize) -> usize {
    let dr = if r1 > r2 { r1 - r2 } else { r2 - r1 };
    let dc = if c1 > c2 { c1 - c2 } else { c2 - c1 };
    dr + dc
}