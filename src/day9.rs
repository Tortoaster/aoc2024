use std::mem;

pub fn solve_a(input: &str) -> u64 {
    let mut disk = parse_input(input);

    for index in (0..disk.len()).rev() {
        let (left, right) = disk.split_at_mut(index);
        if let Some(right) = right.first_mut() {
            match left.iter_mut().find(|n| n.is_none()) {
                None => break,
                Some(left) => mem::swap(left, right),
            }
        }
    }

    checksum(&disk) as u64
}

pub fn solve_b(input: &str) -> u64 {
    let mut disk = parse_input(input);

    let mut files = Vec::new();
    let mut spaces = Vec::new();
    let mut disk_remainder = &mut *disk;

    for (index, n) in input
        .chars()
        .flat_map(|c| c.to_string().parse::<usize>())
        .enumerate()
    {
        let (space, remainder) = disk_remainder.split_at_mut(n);
        disk_remainder = remainder;
        if index & 1 == 0 {
            files.push(space);
        } else {
            spaces.push(space);
        }
    }

    for (file_index, file) in files.into_iter().enumerate().rev() {
        if let Some(index) = spaces
            .iter()
            .take(file_index)
            .position(|space| file.len() <= space.len())
        {
            let space = spaces.remove(index);
            let (space_fit, space_remainder) = space.split_at_mut(file.len());
            spaces.insert(index, space_remainder);
            file.swap_with_slice(space_fit);
        }
    }

    checksum(&disk) as u64
}

fn parse_input(input: &str) -> Vec<Option<usize>> {
    input
        .chars()
        .flat_map(|c| c.to_string().parse::<u64>())
        .enumerate()
        .flat_map(|(index, n)| (0..n).map(move |_| (index & 1 == 0).then_some(index / 2)))
        .collect()
}

fn checksum(numbers: &[Option<usize>]) -> usize {
    numbers
        .iter()
        .enumerate()
        .filter_map(|(index, n)| n.as_ref().map(|n| (index, *n)))
        .map(|(index, n)| index * n)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{solve_a, solve_b};

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_a() {
        assert_eq!(solve_a(INPUT), 1928);
    }

    #[test]
    fn test_b() {
        assert_eq!(solve_b(INPUT), 2858);
    }
}
