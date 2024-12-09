use std::collections::VecDeque;

advent_of_code::solution!(9);

const RADIX: u32 = 10;

enum BlockType {
    File,
    Free,
}

impl BlockType {
    fn from_cursor(cursor: usize) -> Self {
        if cursor % 2 == 0 {
            BlockType::File
        } else {
            BlockType::Free
        }
    }
}

#[derive(Debug)]
struct DiskMap {
    data: VecDeque<Option<u32>>,
    total_empty: usize,
}

impl DiskMap {
    fn from_str(input: &str) -> Self {
        let mut data: VecDeque<Option<u32>> = VecDeque::new();
        let mut total_empty: usize = 0;

        input
            .trim()
            .chars()
            .enumerate()
            .fold(0, |id, (cursor, size_str)| {
                let size: u32 = size_str.to_digit(RADIX).expect("Digit should be parseable");
                match BlockType::from_cursor(cursor) {
                    BlockType::File => {
                        data.extend((0..size).map(|_| Some(id)));
                        id + 1
                    }
                    BlockType::Free => {
                        data.extend((0..size).map(|_| None));
                        total_empty += size as usize;
                        id
                    }
                }
            });

        DiskMap { data, total_empty }
    }

    fn calculate_checksum(&self) -> usize {
        let mut data = self.data.clone();
        let mut drained: VecDeque<Option<u32>> =
            data.drain(data.len() - self.total_empty..).rev().collect();

        data.iter().enumerate().fold(0, |acc, (idx, val)| {
            if let Some(file_id) = val {
                acc + idx * (*file_id as usize)
            } else {
                while let Some(None) = drained.front() {
                    drained.pop_front();
                }
                if let Some(Some(file_id)) = drained.pop_front() {
                    acc + idx * (file_id as usize)
                } else {
                    acc
                }
            }
        })
    }
}

#[derive(Debug)]
struct DiskBlockMap {
    data: VecDeque<(Option<u32>, usize)>,
}

impl DiskBlockMap {
    fn from_str(input: &str) -> Self {
        let mut data: VecDeque<(Option<u32>, usize)> = VecDeque::new();

        input
            .trim()
            .chars()
            .enumerate()
            .fold(0, |id, (cursor, size_str)| {
                let size: u32 = size_str.to_digit(RADIX).expect("Digit should be parseable");
                match BlockType::from_cursor(cursor) {
                    BlockType::File => {
                        data.push_back((Some(id), size as usize));
                        id + 1
                    }
                    BlockType::Free => {
                        data.push_back((None, size as usize));
                        id
                    }
                }
            });

        DiskBlockMap { data }
    }

    fn try_insert_into_free_space(
        &self,
        data: &mut VecDeque<(Option<u32>, usize)>,
        file_id: u32,
        file_size: usize,
    ) -> bool {
        for i in 0..data.len() {
            if let (None, free_size) = &mut data[i] {
                if *free_size >= file_size {
                    *free_size = free_size.saturating_sub(file_size);
                    data.insert(i, (Some(file_id), file_size));
                    return true;
                }
            }
        }
        false
    }

    fn calculate_checksum(&self) -> usize {
        let mut data = self.data.clone();
        let mut compressed: VecDeque<Option<u32>> = VecDeque::new();

        while let Some((file_id_option, file_size)) = data.pop_back() {
            match file_id_option {
                Some(file_id) => {
                    if !self.try_insert_into_free_space(&mut data, file_id, file_size) {
                        compressed.extend((0..file_size).map(|_| Some(file_id)));
                    } else {
                        compressed.extend((0..file_size).map(|_| None));
                    }
                }
                None => {
                    compressed.extend((0..file_size).map(|_| None));
                }
            }
        }

        compressed
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (idx, file_id_option)| {
                if let Some(file_id) = file_id_option {
                    acc + idx * (*file_id as usize)
                } else {
                    acc
                }
            })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let disk_map = DiskMap::from_str(input);
    let checksum = disk_map.calculate_checksum();

    Some(checksum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let disk_block_map = DiskBlockMap::from_str(input);
    let checksum = disk_block_map.calculate_checksum();

    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
