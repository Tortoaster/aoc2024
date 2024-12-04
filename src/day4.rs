pub fn solve_a(input: &str) -> u64 {
    let matrix: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut total = 0;
    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            if matrix[y][x] == 'X' {
                if matrix[y].get(x + 1) == Some(&'M')
                    && matrix[y].get(x + 2) == Some(&'A')
                    && matrix[y].get(x + 3) == Some(&'S')
                {
                    total += 1;
                }
                if matrix.get(y + 1).map(|row| row[x]) == Some('M')
                    && matrix.get(y + 2).map(|row| row[x]) == Some('A')
                    && matrix.get(y + 3).map(|row| row[x]) == Some('S')
                {
                    total += 1;
                }
                if matrix.get(y + 1).and_then(|row| row.get(x + 1)) == Some(&'M')
                    && matrix.get(y + 2).and_then(|row| row.get(x + 2)) == Some(&'A')
                    && matrix.get(y + 3).and_then(|row| row.get(x + 3)) == Some(&'S')
                {
                    total += 1;
                }
            } else if matrix[y][x] == 'S' {
                if matrix[y].get(x + 1) == Some(&'A')
                    && matrix[y].get(x + 2) == Some(&'M')
                    && matrix[y].get(x + 3) == Some(&'X')
                {
                    total += 1;
                }
                if matrix.get(y + 1).map(|row| row[x]) == Some('A')
                    && matrix.get(y + 2).map(|row| row[x]) == Some('M')
                    && matrix.get(y + 3).map(|row| row[x]) == Some('X')
                {
                    total += 1;
                }
                if matrix.get(y + 1).and_then(|row| row.get(x + 1)) == Some(&'A')
                    && matrix.get(y + 2).and_then(|row| row.get(x + 2)) == Some(&'M')
                    && matrix.get(y + 3).and_then(|row| row.get(x + 3)) == Some(&'X')
                {
                    total += 1;
                }
            }
            if matrix[y].get(x + 3) == Some(&'X')
                && matrix.get(y + 1).and_then(|row| row.get(x + 2)) == Some(&'M')
                && matrix.get(y + 2).and_then(|row| row.get(x + 1)) == Some(&'A')
                && matrix.get(y + 3).map(|row| row[x]) == Some('S')
            {
                total += 1;
            }
            if matrix[y].get(x + 3) == Some(&'S')
                && matrix.get(y + 1).and_then(|row| row.get(x + 2)) == Some(&'A')
                && matrix.get(y + 2).and_then(|row| row.get(x + 1)) == Some(&'M')
                && matrix.get(y + 3).map(|row| row[x]) == Some('X')
            {
                total += 1;
            }
        }
    }
    total
}

pub fn solve_b(input: &str) -> u64 {
    let matrix: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut total = 0;
    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            if matrix[y][x] == 'A'
                && (matrix
                    .get(y.wrapping_sub(1))
                    .and_then(|row| row.get(x.wrapping_sub(1)))
                    == Some(&'M')
                    && matrix.get(y + 1).and_then(|row| row.get(x + 1)) == Some(&'S')
                    || matrix
                        .get(y.wrapping_sub(1))
                        .and_then(|row| row.get(x.wrapping_sub(1)))
                        == Some(&'S')
                        && matrix.get(y + 1).and_then(|row| row.get(x + 1)) == Some(&'M'))
                && (matrix.get(y.wrapping_sub(1)).and_then(|row| row.get(x + 1)) == Some(&'M')
                    && matrix.get(y + 1).and_then(|row| row.get(x.wrapping_sub(1))) == Some(&'S')
                    || matrix.get(y.wrapping_sub(1)).and_then(|row| row.get(x + 1)) == Some(&'S')
                        && matrix.get(y + 1).and_then(|row| row.get(x.wrapping_sub(1)))
                            == Some(&'M'))
            {
                total += 1;
            }
        }
    }
    total
}
