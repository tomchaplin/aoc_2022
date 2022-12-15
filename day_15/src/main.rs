use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Reading {
    sensor: (isize, isize),
    beacon: (isize, isize),
    distance: isize,
}

impl Reading {
    fn in_sensor_range(&self, pos: &(isize, isize)) -> bool {
        manhattan_distance(&self.sensor, pos) <= self.distance
    }

    fn get_sensor_perimeter<'a>(
        &'a self,
        grid_size: usize,
    ) -> impl Iterator<Item = (isize, isize)> + 'a {
        ((-self.distance - 1)..=(self.distance + 1))
            .flat_map(move |y_offset| {
                let y_offset_abs = y_offset.abs();
                let x_offset_abs = self.distance + 1 - y_offset_abs;
                let two_x_positions =
                    vec![self.sensor.0 - x_offset_abs, self.sensor.0 + x_offset_abs];
                let y_pos = self.sensor.1 + y_offset;
                two_x_positions.into_iter().map(move |x| (x, y_pos))
            })
            .filter(move |pos| is_in_grid(pos, grid_size))
    }
}

fn outside_all_ranges(readings: &Vec<Reading>, pos: &(isize, isize)) -> bool {
    !readings.iter().any(|r| r.in_sensor_range(pos))
}

fn is_in_grid(pos: &(isize, isize), grid_size: usize) -> bool {
    0 <= pos.0 && pos.0 <= grid_size as isize && 0 <= pos.1 && pos.1 <= grid_size as isize
}

fn get_range(reading: &Reading, row: isize) -> impl Iterator<Item = isize> {
    let y_offset = reading.sensor.1.abs_diff(row) as isize;
    let reduced_distance = reading.distance - y_offset;
    let x_base = reading.sensor.0;
    (x_base - reduced_distance)..=(x_base + reduced_distance)
}

fn manhattan_distance(p1: &(isize, isize), p2: &(isize, isize)) -> isize {
    let x_diff = p1.0.abs_diff(p2.0) as isize;
    let y_diff = p1.1.abs_diff(p2.1) as isize;
    x_diff + y_diff
}

fn parse_line(line: String) -> Reading {
    let line_vec: Vec<&str> = line.split(" ").collect();
    let sensor_x: isize = line_vec[2]
        .replace("x=", "")
        .replace(",", "")
        .parse()
        .unwrap();
    let sensor_y: isize = line_vec[3]
        .replace("y=", "")
        .replace(":", "")
        .parse()
        .unwrap();
    let beacon_x: isize = line_vec[8]
        .replace("x=", "")
        .replace(",", "")
        .parse()
        .unwrap();
    let beacon_y: isize = line_vec[9].replace("y=", "").parse().unwrap();
    let sensor = (sensor_x, sensor_y);
    let beacon = (beacon_x, beacon_y);
    let distance = manhattan_distance(&sensor, &beacon);
    Reading {
        sensor,
        beacon,
        distance,
    }
}

fn read_lines() -> impl Iterator<Item = Reading> {
    let file = File::open("./data/input").expect("Input file not found");
    let lines = io::BufReader::new(file).lines().map(|line| line.unwrap());
    lines.map(parse_line)
}

fn part_a() {
    let readings: Vec<Reading> = read_lines().collect();
    let row = 2000000;

    let becaons_on_row: Vec<isize> = readings
        .iter()
        .filter(|r| r.beacon.1 == row)
        .map(|r| r.beacon.0)
        .collect();

    let positions = readings
        .iter()
        .map(|r| get_range(r, row))
        .flatten()
        .filter(|x| !becaons_on_row.contains(&x))
        .unique()
        .count();

    println!("{}", positions);
}

fn part_b() {
    let readings: Vec<Reading> = read_lines().collect();
    let grid_size = 4000000;
    let answer = readings
        .iter()
        .flat_map(|r| r.get_sensor_perimeter(grid_size))
        .find(|pos| outside_all_ranges(&readings, pos))
        .expect("No answer");
    let signal = answer.0 * 4000000 + answer.1;
    println!("{:?}", answer);
    println!("{}", signal);
}

fn main() {
    part_a();
    part_b();
}
