use crate::grid::grid::{Grid, CellType};
use crate::shape::Shape;

pub fn find_valid_placements(
    grid: &Grid,
    shape: &Shape,
    _player_id: u8
) -> Vec<(usize, usize)> {
    let mut valid_positions: Vec<(usize, usize)> = Vec::new();

    // For small boards, just check everything
    if grid.rows * grid.cols <= 10000 {
        let max_row = grid.rows.saturating_sub(shape.height) + 1;
        let max_col = grid.cols.saturating_sub(shape.width) + 1;
        
        for row in 0..max_row {
            for col in 0..max_col {
                if is_valid_placement(grid, shape, row, col) {
                    valid_positions.push((row, col));
                }
            }
        }
        return valid_positions;
    }

    // For large boards, search near our territory only
    let mut checked = vec![vec![false; grid.cols]; grid.rows];
    
    for r in 0..grid.rows {
        for c in 0..grid.cols {
            if grid.cells[r][c] == CellType::Mine {
                // Check positions where piece could overlap this cell
                for &(dy, dx) in &shape.cells {
                    if r >= dy && c >= dx {
                        let top = r - dy;
                        let left = c - dx;
                        
                        if top + shape.height <= grid.rows 
                            && left + shape.width <= grid.cols 
                            && !checked[top][left] 
                        {
                            checked[top][left] = true;
                            if is_valid_placement(grid, shape, top, left) {
                                valid_positions.push((top, left));
                            }
                        }
                    }
                }
            }
        }
    }

    valid_positions
}

fn is_valid_placement(
    grid: &Grid,
    shape: &Shape,
    top_row: usize,
    left_col: usize,
) -> bool {
    let mut my_overlap_count = 0;

    for &(dy, dx) in &shape.cells {
        let row = top_row + dy;
        let col = left_col + dx;

        if row >= grid.rows || col >= grid.cols {
            return false;
        }

        match grid.cells[row][col] {
            CellType::Enemy => return false,
            CellType::Mine => {
                my_overlap_count += 1;
                if my_overlap_count > 1 {
                    return false;
                }
            }
            CellType::Empty => {}
        }
    }

    my_overlap_count == 1
}