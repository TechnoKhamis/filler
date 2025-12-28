use super::Shape;

impl Shape {
    pub fn from_lines(lines: &[String]) -> Option<Self> {
        if lines.is_empty() {
            return None;
        }

        // Find and parse header
        let header = lines.iter()
            .find(|line| line.trim().starts_with("Piece"))?;

        let parts: Vec<&str> = header.split_whitespace().collect();
        if parts.len() < 3 {
            return None;
        }

        let width: usize = parts[1].parse().ok()?;
        let height: usize = parts[2].trim_end_matches(':').parse().ok()?;

        // Find header position
        let header_index = lines.iter()
            .position(|l| l.trim().starts_with("Piece"))?;

        // Collect pattern lines
        let mut pattern: Vec<String> = Vec::new();
        for line in lines.iter().skip(header_index + 1) {
            let trimmed = line.trim_end().to_string();
            if trimmed.is_empty() {
                continue;
            }
            pattern.push(trimmed);
            if pattern.len() == height {
                break;
            }
        }

        if pattern.is_empty() {
            return None;
        }

        // Find all filled cells
        let mut cells: Vec<(usize, usize)> = Vec::new();

        for (row_idx, row_str) in pattern.iter().enumerate() {
            for (col_idx, ch) in row_str.chars().enumerate() {
                if ch == 'O' || ch == 'o' || ch == '*' {
                    cells.push((row_idx, col_idx));
                }
            }
        }

        if cells.is_empty() {
            return None;
        }

        Some(Shape {
            width,
            height,
            cells,
        })
    }
}