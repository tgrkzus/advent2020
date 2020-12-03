use super::*;

pub fn run(input: &Vec<String>) {
    first(input);
    second(input);
}

fn first(input: &Vec<String>) {
    let map = into_map(input);
    let result = with_slope(&map, 3, 1);
    println!("{}", result);
}

fn second(input: &Vec<String>) {
    let map = into_map(input);
    let result =
        with_slope(&map, 1, 1) *
        with_slope(&map, 3, 1) *
        with_slope(&map, 5, 1) *
        with_slope(&map, 7, 1) *
        with_slope(&map, 7, 2)
    ;
    println!("{}", result);
}

fn with_slope(map: &Map, slope_x: usize, slope_y: usize) -> i64 {
    let mut result = 0;
    let mut col = 0;
    let mut row = 0;
    while row < map.row_size {
        if map.get(row, col) {
            result += 1;
        }
        col += slope_x;
        row += slope_y;
    }
    result
}

fn into_map(input: &Vec<String>) -> Map {
    let mut map = Map::new(input[0].len(), input.len());
    input.iter()
        .map(|line| line.chars()
            .map(|c| c == '#')
            .collect()
        )
        .for_each(|row| map.add_row(row));
    map
}

struct Map {
    nodes: Vec<Vec<bool>>,
    col_size: usize,
    pub row_size: usize,
}

impl Map {
    pub fn new(col_size: usize, row_size: usize) -> Self
    {
        Self {
            nodes: Vec::new(),
            col_size,
            row_size,
        }
    }

    pub fn add_row(&mut self, row: Vec<bool>) {
        self.nodes.push(row)
    }

    pub fn get(&self, row: usize, col: usize) -> bool {
        self.nodes[row][col % self.col_size]
    }
}

