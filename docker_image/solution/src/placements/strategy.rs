// src/placements/strategy.rs
// Aggressive blocking strategy: Rush to enemy, block them, take the rest
// (Same logic as friend's game.rs)

use crate::grid::grid::{Grid, CellType};
use crate::shape::Shape;

pub fn choose_best_placement(
    grid: &Grid,
    shape: &Shape,
    valid_positions: &[(usize, usize)],
    _player_id: u8
) -> Option<(usize, usize)> {
    if valid_positions.is_empty() || shape.cells.is_empty() || grid.rows == 0 || grid.cols == 0 {
        return None;
    }

    // Precompute coordinates
    let mut enemy_coords: Vec<(usize, usize)> = Vec::new();
    let mut my_coords: Vec<(usize, usize)> = Vec::new();
    
    for y in 0..grid.rows {
        for x in 0..grid.cols {
            match grid.cells[y][x] {
                CellType::Enemy => enemy_coords.push((y, x)),
                CellType::Mine => my_coords.push((y, x)),
                _ => {}
            }
        }
    }

    if my_coords.is_empty() {
        return None;
    }

    // Find the closest enemy cell to any of my cells
    let (closest_my, closest_enemy, min_distance) = find_closest_pair(&my_coords, &enemy_coords);
    
    // Calculate the direction vector from my closest cell to enemy's closest cell
    let target_direction = if !enemy_coords.is_empty() {
        (
            closest_enemy.0 as isize - closest_my.0 as isize,
            closest_enemy.1 as isize - closest_my.1 as isize,
        )
    } else {
        // No enemy visible, head toward center
        let center = (grid.rows / 2, grid.cols / 2);
        let my_center = calculate_centroid(&my_coords);
        (
            center.0 as isize - my_center.0 as isize,
            center.1 as isize - my_center.1 as isize,
        )
    };

    // Find the frontier cells (my cells that can have pieces placed adjacent to them)
    let frontier = find_frontier(&my_coords, grid);

    let mut best_pos: Option<(usize, usize)> = None;
    let mut best_score: i64 = i64::MIN;

    for &(top_y, left_x) in valid_positions {
        let score = score_placement(
            grid, shape, top_y, left_x,
            &enemy_coords, &frontier,
            target_direction, min_distance, closest_enemy
        );

        if score > best_score {
            best_score = score;
            best_pos = Some((top_y, left_x));
        }
    }

    best_pos
}

fn find_closest_pair(
    my_coords: &[(usize, usize)],
    enemy_coords: &[(usize, usize)],
) -> ((usize, usize), (usize, usize), usize) {
    if enemy_coords.is_empty() || my_coords.is_empty() {
        let my_first = my_coords.first().copied().unwrap_or((0, 0));
        return (my_first, (0, 0), usize::MAX);
    }

    let mut best_my = my_coords[0];
    let mut best_enemy = enemy_coords[0];
    let mut best_dist = usize::MAX;

    for &(my, mx) in my_coords {
        for &(ey, ex) in enemy_coords {
            let dist = (my as isize - ey as isize).unsigned_abs()
                + (mx as isize - ex as isize).unsigned_abs();
            if dist < best_dist {
                best_dist = dist;
                best_my = (my, mx);
                best_enemy = (ey, ex);
            }
        }
    }

    (best_my, best_enemy, best_dist)
}

fn calculate_centroid(coords: &[(usize, usize)]) -> (usize, usize) {
    if coords.is_empty() {
        return (0, 0);
    }
    let sum_y: usize = coords.iter().map(|(y, _)| *y).sum();
    let sum_x: usize = coords.iter().map(|(_, x)| *x).sum();
    (sum_y / coords.len(), sum_x / coords.len())
}

fn find_frontier(my_coords: &[(usize, usize)], grid: &Grid) -> Vec<(usize, usize)> {
    let mut frontier = Vec::new();
    const DIRS: &[(isize, isize)] = &[(1, 0), (-1, 0), (0, 1), (0, -1)];
    
    for &(y, x) in my_coords {
        for &(dy, dx) in DIRS {
            let ny = y as isize + dy;
            let nx = x as isize + dx;
            
            if ny >= 0 && nx >= 0 
                && (ny as usize) < grid.rows 
                && (nx as usize) < grid.cols
                && grid.cells[ny as usize][nx as usize] == CellType::Empty 
            {
                frontier.push((y, x));
                break;
            }
        }
    }
    frontier
}

fn score_placement(
    grid: &Grid,
    shape: &Shape,
    top_y: usize,
    left_x: usize,
    enemy_coords: &[(usize, usize)],
    frontier: &[(usize, usize)],
    target_direction: (isize, isize),
    current_min_distance: usize,
    closest_enemy: (usize, usize),
) -> i64 {
    let rows = grid.rows;
    let cols = grid.cols;
    
    // Calculate where this placement puts us
    let mut piece_cells: Vec<(usize, usize)> = Vec::new();
    let mut new_territory: i64 = 0;
    let mut adjacent_to_enemy: i64 = 0;
    
    const DIRS: &[(isize, isize)] = &[(1, 0), (-1, 0), (0, 1), (0, -1)];

    for &(dy, dx) in &shape.cells {
        let ay = top_y + dy;
        let ax = left_x + dx;
        piece_cells.push((ay, ax));

        if grid.cells[ay][ax] == CellType::Empty {
            new_territory += 1;
        }

        // Check for enemy adjacency
        for &(dyy, dxx) in DIRS {
            let ny = ay as isize + dyy;
            let nx = ax as isize + dxx;
            
            if ny >= 0 && nx >= 0 && (ny as usize) < rows && (nx as usize) < cols {
                if grid.cells[ny as usize][nx as usize] == CellType::Enemy {
                    adjacent_to_enemy += 1;
                }
            }
        }
    }

    // Calculate the "most forward" point of this placement
    let mut best_advance: i64 = i64::MIN;
    let mut min_dist_to_enemy: usize = usize::MAX;
    
    for &(py, px) in &piece_cells {
        // How much does this cell advance toward target?
        // Use dot product with normalized direction
        let advance = if target_direction.0 != 0 || target_direction.1 != 0 {
            let norm = ((target_direction.0 * target_direction.0 + target_direction.1 * target_direction.1) as f64).sqrt();
            if norm > 0.0 {
                // Project movement onto target direction
                let move_y = py as isize - frontier.first().map(|f| f.0 as isize).unwrap_or(0);
                let move_x = px as isize - frontier.first().map(|f| f.1 as isize).unwrap_or(0);
                ((move_y * target_direction.0 + move_x * target_direction.1) as f64 / norm) as i64
            } else {
                0
            }
        } else {
            0
        };
        
        if advance > best_advance {
            best_advance = advance;
        }

        // Distance to closest enemy
        for &(ey, ex) in enemy_coords {
            let d = (py as isize - ey as isize).unsigned_abs()
                + (px as isize - ex as isize).unsigned_abs();
            if d < min_dist_to_enemy {
                min_dist_to_enemy = d;
            }
        }
    }

    // Distance to the closest enemy cell we identified
    let dist_to_target = {
        let (ty, tx) = closest_enemy;
        let mut min_d = usize::MAX;
        for &(py, px) in &piece_cells {
            let d = (py as isize - ty as isize).unsigned_abs()
                + (px as isize - tx as isize).unsigned_abs();
            if d < min_d {
                min_d = d;
            }
        }
        min_d
    };

    // SCORING STRATEGY:
    // 1. If far from enemy (distance > 5): RUSH - minimize distance
    // 2. If close to enemy (distance <= 5): BLOCK - stay adjacent, expand around them
    
    if current_min_distance > 5 {
        // RUSH MODE: Get to enemy ASAP
        // Heavily reward reducing distance
        let distance_reduction = current_min_distance as i64 - min_dist_to_enemy as i64;
        let closeness_score = 1000000 / (min_dist_to_enemy as i64 + 1);
        
        closeness_score * 100           // Getting close is everything
        + distance_reduction * 50000    // Reward reducing distance
        + best_advance * 1000           // Reward advancing toward target
        + new_territory * 10            // Territory is almost irrelevant
        + adjacent_to_enemy * 100000    // If we can touch enemy, amazing!
    } else {
        // BLOCK MODE: We're close - now surround and contain
        let closeness_score = 100000 / (min_dist_to_enemy as i64 + 1);
        
        adjacent_to_enemy * 50000       // Stay glued to enemy
        + closeness_score * 50          // Stay close
        + new_territory * 2000          // Now territory matters
        + best_advance * 500            // Still advance when possible
        - (dist_to_target as i64) * 100 // Don't drift away from target
    }
}