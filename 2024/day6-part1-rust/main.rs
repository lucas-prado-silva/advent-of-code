use std::{
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

#[derive(Debug)]
enum MapObject {
    Guard,
    Ground,
    Obstacle,
}

#[derive(Debug)]
enum GuardDirection {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Map {
    objects: Vec<Vec<MapObject>>,
    rows: usize,
    columns: usize,
}

struct State {
    map: Map,
    guard: Guard,
}

impl State {
    fn move_up(&mut self) {
        match self.map.objects[self.guard.y - 1][self.guard.x] {
            MapObject::Guard => (),
            _ => {
                self.map.objects[self.guard.y - 1][self.guard.x] = MapObject::Guard;
                self.guard.visited_positions += 1;
            }
        }
        self.guard.y -= 1;
    }
    fn move_down(&mut self) {
        match self.map.objects[self.guard.y + 1][self.guard.x] {
            MapObject::Guard => (),
            _ => {
                self.map.objects[self.guard.y + 1][self.guard.x] = MapObject::Guard;
                self.guard.visited_positions += 1;
            }
        }
        self.guard.y += 1;
    }
    fn move_left(&mut self) {
        match self.map.objects[self.guard.y][self.guard.x - 1] {
            MapObject::Guard => (),
            _ => {
                self.map.objects[self.guard.y][self.guard.x - 1] = MapObject::Guard;
                self.guard.visited_positions += 1;
            }
        }
        self.guard.x -= 1;
    }
    fn move_right(&mut self) {
        match self.map.objects[self.guard.y][self.guard.x + 1] {
            MapObject::Guard => (),
            _ => {
                self.map.objects[self.guard.y][self.guard.x + 1] = MapObject::Guard;
                self.guard.visited_positions += 1;
            }
        }
        self.guard.x += 1;
    }
    fn next_move_will_leave_map(&self) -> bool {
        match self.guard.direction {
            GuardDirection::Up => return self.guard.y == 0,
            GuardDirection::Right => return self.guard.x == self.map.columns - 1,
            GuardDirection::Down => return self.guard.y == self.map.rows - 1,
            GuardDirection::Left => return self.guard.x == 0,
        }
    }
    fn next_move_will_hit_obstacle(&self) -> bool {
        let next_x;
        let next_y;
        match self.guard.direction {
            GuardDirection::Up => {
                next_x = self.guard.x;
                next_y = self.guard.y - 1;
            }
            GuardDirection::Right => {
                next_x = self.guard.x + 1;
                next_y = self.guard.y;
            }
            GuardDirection::Down => {
                next_x = self.guard.x;
                next_y = self.guard.y + 1;
            }
            GuardDirection::Left => {
                next_x = self.guard.x - 1;
                next_y = self.guard.y;
            }
        }
        match self.map.objects[next_y][next_x] {
            MapObject::Obstacle => {
                return true;
            }
            _ => return false,
        }
    }
}

struct Guard {
    direction: GuardDirection,
    x: usize,
    y: usize,
    visited_positions: usize,
}

impl Guard {
    fn rotate_90_degrees(&mut self) {
        match self.direction {
            GuardDirection::Up => self.direction = GuardDirection::Right,
            GuardDirection::Right => self.direction = GuardDirection::Down,
            GuardDirection::Down => self.direction = GuardDirection::Left,
            GuardDirection::Left => self.direction = GuardDirection::Up,
        }
    }
}

fn main() {
    let mut state = read_input("input1.txt");

    loop {
        match state.guard.direction {
            GuardDirection::Up => {
                if state.next_move_will_leave_map() {
                    break;
                } else if state.next_move_will_hit_obstacle() {
                    state.guard.rotate_90_degrees();
                } else {
                    state.move_up();
                }
            }
            GuardDirection::Right => {
                if state.next_move_will_leave_map() {
                    break;
                } else if state.next_move_will_hit_obstacle() {
                    state.guard.rotate_90_degrees();
                } else {
                    state.move_right();
                }
            }
            GuardDirection::Down => {
                if state.next_move_will_leave_map() {
                    break;
                } else if state.next_move_will_hit_obstacle() {
                    state.guard.rotate_90_degrees();
                } else {
                    state.move_down();
                }
            }
            GuardDirection::Left => {
                if state.next_move_will_leave_map() {
                    break;
                } else if state.next_move_will_hit_obstacle() {
                    state.guard.rotate_90_degrees();
                } else {
                    state.move_left();
                }
            }
        }
    }

    println!("{}", state.guard.visited_positions);
}

fn read_input(file_name: &str) -> State {
    let br = BufReader::new(File::open(file_name).unwrap());

    let mut state = State {
        map: Map {
            objects: Vec::new(),
            rows: 0,
            columns: 0,
        },
        guard: Guard {
            direction: GuardDirection::Up,
            x: 0,
            y: 0,
            visited_positions: 0,
        },
    };

    for line in br.lines() {
        match line {
            Ok(line) => {
                let mut cols_travelled = 0;
                state.map.objects.push(
                    line.chars()
                        .map(|c| {
                            if c == '^' {
                                state.guard.direction = GuardDirection::Up;
                                state.guard.x = cols_travelled;
                                state.guard.y = state.map.rows;
                                cols_travelled += 1;
                                state.guard.visited_positions += 1;
                                return MapObject::Guard;
                            }
                            if c == '#' {
                                cols_travelled += 1;
                                return MapObject::Obstacle;
                            }
                            cols_travelled += 1;
                            return MapObject::Ground;
                        })
                        .collect(),
                );
                if state.map.columns == 0 {
                    state.map.columns = line.len();
                }
                state.map.rows += 1;
            }
            Err(_) => (),
        }
    }

    state
}
