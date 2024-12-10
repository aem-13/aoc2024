use std::{fs, usize};

type Input = Grid<u8>;

const RIGHT: (i32, i32) = (0, 1);
const DOWN: (i32, i32) = (1, 0);
const DIAG_RIGHT: (i32, i32) = (1, 1);
const DIAG_LEFT: (i32, i32) = (1, -1);

pub fn parse(path: &str) -> Input {
    let data = fs::read_to_string(path);
    let parsed = match data {
        Ok(raw) => raw,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };

    Grid::new(parsed)
}

pub fn part_one(input: &Input) -> u32 {
    let mut xmas_count = 0;

    let size = input.width as i32;
    for i in 0..size {
        xmas_count += find_xmas(input, (i, 0), RIGHT, size);
        xmas_count += find_xmas(input, (0, i), DOWN, size);
    }
    for i in 0..size - 3 {
        xmas_count += find_xmas(input, (0, i), DIAG_RIGHT, size - i);
        xmas_count += find_xmas(input, (i + 1, 0), DIAG_RIGHT, size - 1 - i);
        xmas_count += find_xmas(input, (0, size - 1 - i), DIAG_LEFT, size - i);
        xmas_count += find_xmas(input, (i + 1, size - 1), DIAG_LEFT, size - 1 - i);
    }
    xmas_count
}

pub fn part_two(input: &Input) -> u32 {
    let mut x_mas_count = 0;

    for width_index in 1..input.width - 1 {
        let w = width_index as usize;
        for height_index in 1..input.height - 1 {
            let h = height_index as usize;
            if *input.get(h, w) == b'A' {
                let top_left = *input.get(h - 1, w - 1);
                let top_right = *input.get(h - 1, w + 1);
                let lower_right = *input.get(h + 1, w + 1);
                let lower_left = *input.get(h + 1, w - 1);

                if top_left.abs_diff(lower_right) == 6 && top_right.abs_diff(lower_left) == 6 {
                    x_mas_count += 1;
                }
            }
        }
    }
    x_mas_count
}
fn find_xmas(grid: &Grid<u8>, mut grid_pos: (i32, i32), travel: (i32, i32), size: i32) -> u32 {
    let mut xmas_count = 0;
    let mut bytes = 0;

    for _ in 0..size {
        let letter = *grid.get(grid_pos.0 as usize, grid_pos.1 as usize) as u32;
        bytes = (bytes << 8) | letter;
        grid_pos = (grid_pos.0 + travel.0, grid_pos.1 + travel.1);
        if bytes == 0x584d4153 || bytes == 0x53414d58 {
            xmas_count += 1;
        }
    }
    xmas_count
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    height: u32,
    width: u32,
    data: Vec<Vec<T>>,
}

impl Grid<u8> {
    pub fn new(parsed: String) -> Self {
        let bytes: Vec<Vec<u8>> = parsed
            .lines()
            .map(|line| line.as_bytes().to_vec())
            .collect();
        let rows = bytes.len() as u32;
        let cols = bytes[0].len() as u32;

        Grid {
            height: rows,
            width: cols,
            data: bytes,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> &u8 {
        &self.data[row][col]
    }
}
