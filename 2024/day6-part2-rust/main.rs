use std::{
    fs::File, io::{BufRead, BufReader}, thread, time::Duration, usize
};

#[derive(Debug)]
enum MapObject {
    Guard,
    Ground,
    Obstacle,
    HitObstacle,
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
    virtual_guard: Guard,
}

impl State {
    fn move_guard(&mut self) {
        match self.guard.direction {
            GuardDirection::Up => self.move_up(),
            GuardDirection::Right => self.move_right(),
            GuardDirection::Down => self.move_down(),
            GuardDirection::Left => self.move_left(),
        }
    }
    fn move_virtual_guard(&mut self) {
        match self.virtual_guard.direction {
            GuardDirection::Up => self.move_virtual_up(),
            GuardDirection::Right => self.move_virtual_right(),
            GuardDirection::Down => self.move_virtual_down(),
            GuardDirection::Left => self.move_virtual_left(),
        }
    }
    fn move_virtual_up(&mut self) {
        self.virtual_guard.y -= 1;
    }
    fn move_virtual_down(&mut self) {
        self.virtual_guard.y += 1;
    }
    fn move_virtual_left(&mut self) {
        self.virtual_guard.x -= 1;
    }
    fn move_virtual_right(&mut self) {
        self.virtual_guard.x += 1;
    }
    fn move_up(&mut self) {
        match self.map.objects[self.guard.y - 1][self.guard.x] {
            MapObject::Guard => (),
            _ => {
                self.map.objects[self.guard.y - 1][self.guard.x] = MapObject::Guard;
                self.guard.visited_positions += 1;
            }
        }
        self.guard.y -= 1;
        self.virtual_guard.y -= 1;
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
        self.virtual_guard.y += 1;
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
        self.virtual_guard.x -= 1;
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
        self.virtual_guard.x += 1;
    }
    fn next_move_will_leave_map(&self) -> bool {
        match self.guard.direction {
            GuardDirection::Up => return self.guard.y == 0,
            GuardDirection::Right => return self.guard.x == self.map.columns - 1,
            GuardDirection::Down => return self.guard.y == self.map.rows - 1,
            GuardDirection::Left => return self.guard.x == 0,
        }
    }
    fn virtual_next_move_will_leave_map(&self) -> bool {
        match self.virtual_guard.direction {
            GuardDirection::Up => return self.virtual_guard.y == 0,
            GuardDirection::Right => return self.virtual_guard.x == self.map.columns - 1,
            GuardDirection::Down => return self.virtual_guard.y == self.map.rows - 1,
            GuardDirection::Left => return self.virtual_guard.x == 0,
        }
    }
    fn would_turning_here_create_loop(&mut self) -> bool {
        self.virtual_guard = Guard {
            direction: match self.guard.direction {
                GuardDirection::Up => GuardDirection::Up,
                GuardDirection::Right => GuardDirection::Right,
                GuardDirection::Down => GuardDirection::Down,
                GuardDirection::Left => GuardDirection::Left,
            },
            x: self.guard.x,
            y: self.guard.y,
            visited_positions: 0,
        };

        let initial_x = self.guard.x;
        let initial_y = self.guard.y;
        match self.virtual_guard.direction {
            GuardDirection::Up => {
                if initial_y == 0 {
                    return false;
                }
                self.map.objects[initial_y - 1][initial_x] = MapObject::HitObstacle;
            }
            GuardDirection::Right => {
                if initial_x == self.map.columns - 1 {
                    return false;
                }
                self.map.objects[initial_y][initial_x + 1] = MapObject::HitObstacle;
            }
            GuardDirection::Down => {
                if initial_y == self.map.rows - 1 {
                    return false;
                }
                self.map.objects[initial_y + 1][initial_x] = MapObject::HitObstacle;
            }
            GuardDirection::Left => {
                if initial_x == 0 {
                    return false;
                }
                self.map.objects[initial_y][initial_x - 1] = MapObject::HitObstacle;
            }
        }

        println!("starting virtual loop");
        println!("starting position {initial_x}|{initial_y}");
        let mut loop_iters = 0;
        loop {
            println!("current position {}|{}", self.virtual_guard.x, self.virtual_guard.y);
            if self.virtual_guard.x == initial_x
                && self.virtual_guard.y == initial_y
                && loop_iters != 0
            {
                println!("found loop");
                return true;
            }
            loop_iters += 1;
            if self.virtual_next_move_will_leave_map() {
                // println!("next virtual move will leave map");
                return false;
            }
            if self.virtual_next_move_will_hit_obstacle() {
                // println!("rotating virtual guard");
                self.virtual_guard.rotate_90_degrees();
            }

            // println!("moving virtual guard in direction {:?}", self.virtual_guard.direction);
            self.move_virtual_guard();
            if loop_iters > 200 {
                thread::sleep(Duration::from_millis(4000));
                println!("infinite somehow");
            }
        }
        // match self.guard.direction {
        //     GuardDirection::Up => {
        //         let mut current_x = self.guard.x;
        //         while current_x < self.map.columns - 1 {
        //             current_x += 1;
        //             match self.map.objects[self.guard.y][current_x] {
        //                 MapObject::HitObstacle => return true,
        //                 _ => (),
        //             }
        //         }
        //         false
        //     }
        //     GuardDirection::Right => {
        //         let mut current_y = self.guard.y;
        //         while current_y < self.map.rows - 1 {
        //             current_y += 1;
        //             match self.map.objects[current_y][self.guard.x] {
        //                 MapObject::HitObstacle => return true,
        //                 _ => (),
        //             }
        //         }
        //         false
        //     }
        //     GuardDirection::Down => {
        //         let mut current_x = self.guard.x;
        //         while current_x != 0 {
        //             current_x -= 1;
        //             match self.map.objects[self.guard.y][current_x] {
        //                 MapObject::HitObstacle => return true,
        //                 _ => (),
        //             }
        //         }
        //         false
        //     }
        //     GuardDirection::Left => {
        //         let mut current_y = self.guard.y;
        //         while current_y != 0 {
        //             current_y -= 1;
        //             match self.map.objects[current_y][self.guard.x] {
        //                 MapObject::HitObstacle => return true,
        //                 _ => (),
        //             }
        //         }
        //         false
        //     }
        // }
    }
    fn virtual_next_move_will_hit_obstacle(&mut self) -> bool {
        let next_x;
        let next_y;
        match self.virtual_guard.direction {
            GuardDirection::Up => {
                next_x = self.virtual_guard.x;
                next_y = self.virtual_guard.y - 1;
            }
            GuardDirection::Right => {
                next_x = self.virtual_guard.x + 1;
                next_y = self.virtual_guard.y;
            }
            GuardDirection::Down => {
                next_x = self.virtual_guard.x;
                next_y = self.virtual_guard.y + 1;
            }
            GuardDirection::Left => {
                next_x = self.virtual_guard.x - 1;
                next_y = self.virtual_guard.y;
            }
        }
        match self.map.objects[next_y][next_x] {
            MapObject::Obstacle => {
                return true;
            }
            MapObject::HitObstacle => {
                return true;
            }
            _ => return false,
        }
    }
    fn next_move_will_hit_obstacle(&mut self) -> bool {
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
    let mut potential_loops = 0;

    loop {
        if state.next_move_will_leave_map() {
            break;
        }
        if state.next_move_will_hit_obstacle() {
            state.guard.rotate_90_degrees();
            state.virtual_guard.rotate_90_degrees();
        }
        if state.would_turning_here_create_loop() {
            potential_loops += 1;
        }
        state.move_guard();
    }

    println!("{}", state.guard.visited_positions);
    println!("{}", potential_loops);
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
        virtual_guard: Guard {
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
                                state.virtual_guard.direction = GuardDirection::Up;
                                state.virtual_guard.x = cols_travelled;
                                state.guard.x = cols_travelled;
                                state.virtual_guard.y = state.map.rows;
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
