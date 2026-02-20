pub fn annotate(garden: &[&str]) -> Vec<String> {
    let height = garden.len();
    if height == 0 {
        return Vec::new();
    }
    let width = garden[0].len();

    let mut result = Vec::with_capacity(height);

    for r in 0..height {
        let mut row_string = String::with_capacity(width);
        for c in 0..width {
            if garden[r].as_bytes()[c] == b'*' {
                row_string.push('*');
                continue;
            }
            let mut flowers = 0;
            let row_start = r.saturating_sub(1);
            let row_end = (r + 1).min(height - 1);

            let col_start = c.saturating_sub(1);
            let col_end = (c + 1).min(width - 1);

            for neighbor_r in row_start..=row_end {
                for neighbor_c in col_start..=col_end {
                    if garden[neighbor_r].as_bytes()[neighbor_c] == b'*' {
                        flowers += 1;
                    }
                }
            }

            if flowers > 0 {
                row_string.push(std::char::from_digit(flowers, 10).unwrap());
            } else {
                row_string.push(' ');
            }
        }
        result.push(row_string);
    }
    result
}
