pub fn solve_a(input: &str) -> u64 {
    let matrix: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut words = Vec::new();

    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() - 3 {
            words.push([
                matrix[y][x],
                matrix[y][x + 1],
                matrix[y][x + 2],
                matrix[y][x + 3],
            ]);
        }
    }

    for y in 0..matrix.len() - 3 {
        for x in 0..matrix[0].len() {
            words.push([
                matrix[y][x],
                matrix[y + 1][x],
                matrix[y + 2][x],
                matrix[y + 3][x],
            ]);
        }
    }

    for y in 0..matrix.len() - 3 {
        for x in 0..matrix[0].len() - 3 {
            words.push([
                matrix[y][x],
                matrix[y + 1][x + 1],
                matrix[y + 2][x + 2],
                matrix[y + 3][x + 3],
            ]);
            words.push([
                matrix[y + 3][x],
                matrix[y + 2][x + 1],
                matrix[y + 1][x + 2],
                matrix[y][x + 3],
            ]);
        }
    }

    words
        .into_iter()
        .filter(|&word| word == ['X', 'M', 'A', 'S'] || word == ['S', 'A', 'M', 'X'])
        .count() as u64
}

pub fn solve_b(input: &str) -> u64 {
    let matrix: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut total = 0;
    for y in 1..matrix.len() - 1 {
        for x in 1..matrix[0].len() - 1 {
            if matrix[y][x] == 'A'
                && (matrix[y - 1][x - 1] == 'M' && matrix[y + 1][x + 1] == 'S'
                    || matrix[y - 1][x - 1] == 'S' && matrix[y + 1][x + 1] == 'M')
                && (matrix[y - 1][x + 1] == 'M' && matrix[y + 1][x - 1] == 'S'
                    || matrix[y - 1][x + 1] == 'S' && matrix[y + 1][x - 1] == 'M')
            {
                total += 1;
            }
        }
    }
    total
}
