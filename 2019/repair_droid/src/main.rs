use intcode::get_data_from_path;
use intcode::program::{Int, Program};
use std::collections::{HashMap, VecDeque};
use std::io::{self, Read, Write};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, color, cursor, style};
use Direction::*;
use Tile::*;

const INFINITY: Int = Int::max_value();
const SOURCE: (Int, Int) = (35, 25);
const TARGET_NOT_FOUND: (Int, Int) = (INFINITY, INFINITY);

struct Explorer<R, W: Write> {
    robot: (Int, Int),
    target: (Int, Int),
    stdout: W,
    stdin: R,
    visited: HashMap<(Int, Int), Tile>,
    pending: VecDeque<(Int, Int)>,
    distances: HashMap<(Int, Int), Int>,
    predecesors: HashMap<(Int, Int), Direction>,
    robot_out: Receiver<Int>,
    robot_in: Sender<Int>,
    no_target: bool,
    source: (Int, Int),
}

#[derive(Copy, Clone)]
enum Direction {
    Up = 1,
    Right = 4,
    Left = 3,
    Down = 2,
}

impl Direction {
    fn oposite(&self) -> Direction {
        match self {
            Up => Down,
            Left => Right,
            Right => Left,
            Down => Up,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Tile {
    OpenSpace,
    Wall,
    Target,
    Robot,
}

impl<R: Read, W: Write> Explorer<R, W> {
    fn new(
        stdin: R,
        stdout: W,
        robot_in: Sender<Int>,
        robot_out: Receiver<Int>,
    ) -> Explorer<R, RawTerminal<W>> {
        Explorer {
            robot: SOURCE,
            target: TARGET_NOT_FOUND,
            stdout: stdout.into_raw_mode().unwrap(),
            stdin: stdin,
            visited: HashMap::new(),
            pending: VecDeque::new(),
            distances: HashMap::new(),
            predecesors: HashMap::new(),
            robot_in,
            robot_out,
            no_target: false,
            source: SOURCE,
        }
    }

    fn start(&mut self) {
        self.init();
        while let Some(next) = self.pending.pop_front() {
            self.go_to(next);

            if next == self.target {
                self.visited.insert(next, Target);

                self.visited.clear();
                self.pending.clear();
                self.predecesors.clear();
                self.distances.clear();
                self.source = next;
                self.target = TARGET_NOT_FOUND;
                self.no_target = true;
                self.distances.insert(next, 0);
                // break;
            }

            // if it was in the queue it is either an open space or the target
            self.visited.insert(next, OpenSpace);

            for dir in &[Up, Right, Down, Left] {
                // explore each direction
                let dir = *dir;
                let n = self.get_position(dir);
                if !self.visited.contains_key(&n) {
                    let (pos, kind) = self.explore(dir);

                    if kind == Wall {
                        self.visited.insert(pos, kind);
                        continue;
                    }

                    let alt_distance = self.distances.get(&next).unwrap() + 1;
                    if &alt_distance < self.distances.get(&pos).unwrap_or(&INFINITY) {
                        self.distances.insert(pos, alt_distance);
                        self.predecesors.insert(pos, dir.oposite());
                        if !self.pending.contains(&pos) {
                            self.pending.push_back(pos);
                        }
                    }

                    if kind == Target {
                        self.target = pos;
                    }
                }
            }
            // wait for an Enter
            self.update();
            // let mut b = [0];
            // self.stdin.read(&mut b).unwrap();
            //
            // match b[0] {
            //     b'q' => return,
            //     _ => (),
            // }
        }
    }

    fn init(&mut self) {
        write!(
            self.stdout,
            "{}{}",
            clear::All,
            cursor::Goto(self.robot.0 as u16, self.robot.1 as u16)
        )
        .unwrap();
        self.mark(Tile::Robot, None);
        self.pending.push_front(self.robot);
        self.distances.insert(self.robot, 0);
        self.update();
    }

    fn update(&mut self) {
        write!(self.stdout, "{}", cursor::Hide).unwrap();
        self.stdout.flush().unwrap();
    }

    fn go_to(&mut self, pos: (Int, Int)) {
        self.mark(OpenSpace, None);
        // go back to source
        while let Some(&dir) = self.predecesors.get(&self.robot) {
            self.move_robot(dir);
        }
        assert_eq!(self.robot, self.source);

        // go from source to pos
        let mut current = pos;
        let mut directions = vec![];
        while let Some(dir) = self.predecesors.get(&current) {
            let (mut x, mut y) = current;
            match dir {
                Direction::Up => y -= 1,
                Direction::Down => y += 1,
                Direction::Left => x -= 1,
                Direction::Right => x += 1,
            };
            current = (x, y);
            directions.push(dir.oposite());
        }
        for dir in directions.iter().rev() {
            self.move_robot(*dir);
        }

        assert_eq!(self.robot, pos);
        self.mark(Robot, None);
    }

    /// Explore returns what is found in the direction given by dir in the form
    /// (x, y) is of kind k. Leaves the robot in its original position
    fn explore(&mut self, dir: Direction) -> ((Int, Int), Tile) {
        let robot_backup = self.robot;

        let new_info = self.move_robot(dir);
        if self.robot != robot_backup {
            self.move_robot(dir.oposite());
        }

        new_info
    }

    /// moves the robot and returns the new information gathered in the form
    /// (x, y) is of kind k:Tile
    fn move_robot(&mut self, dir: Direction) -> ((Int, Int), Tile) {
        self.robot_in.send(dir as Int).unwrap();
        if let Ok(out) = self.robot_out.recv() {
            return match out {
                0 => {
                    self.mark(Wall, Some(dir));
                    (self.get_position(dir), Wall)
                }
                1 => {
                    self.mark(OpenSpace, None);
                    self.robot = self.get_position(dir);
                    (self.robot, OpenSpace)
                }
                2 => {
                    self.mark(OpenSpace, None);
                    if self.no_target {
                        self.robot = self.get_position(dir);
                    } else {
                        self.target = self.get_position(dir);
                        self.robot = self.target;
                    }
                    (self.robot, Target)
                }
                _ => panic!("robot got crazy: {}", out),
            };
        }
        unreachable!("robot did not answer");
    }

    fn get_position(&self, dir: Direction) -> (Int, Int) {
        let (mut x, mut y) = self.robot;
        match dir {
            Direction::Up => y -= 1,
            Direction::Down => y += 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        }
        (x, y)
    }

    fn mark(&mut self, kind: Tile, dir: Option<Direction>) {
        let (x, y) = if let Some(d) = dir {
            self.get_position(d)
        } else {
            self.robot
        };

        write!(self.stdout, "{}", cursor::Goto(x as u16, y as u16)).unwrap();
        // if in target paint the background cyan when over that tile,
        // or the trophy otherwise
        if (x, y) == self.target {
            let cyan = color::Cyan;
            if (x, y) == self.robot {
                //background
                write!(
                    self.stdout,
                    "{}{}{}",
                    color::Bg(cyan),
                    '*',
                    color::Bg(color::Reset)
                )
                .unwrap();
            } else {
                // trophy icon
                write!(
                    self.stdout,
                    "{}{}{}",
                    color::Fg(cyan),
                    '๏',
                    color::Fg(color::Reset)
                )
                .unwrap();
            }
        } else {
            let icon = match kind {
                Tile::OpenSpace => '.',
                Tile::Robot => '*',
                Tile::Wall => '█',
                Tile::Target => '๏',
            };
            write!(self.stdout, "{}", icon).unwrap();
        }
        write!(
            self.stdout,
            "{}",
            cursor::Goto(self.robot.0 as u16, self.robot.1 as u16)
        )
        .unwrap();
    }
}

impl<R, W: Write> Drop for Explorer<R, W> {
    fn drop(&mut self) {
        // When done, restore the defaults to avoid messing with the terminal.
        let size = termion::terminal_size().unwrap();
        write!(
            self.stdout,
            "{}{}{}\r",
            style::Reset,
            cursor::Goto(1, size.1),
            cursor::Show
        )
        .unwrap();
        // println!("{:?}", self.distances.get(&self.target));
        println!("{:?}", self.distances.values().max());
    }
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let (input_sender, input_receiver) = channel();
    let (output_sender, output_receiver) = channel();
    let mut explorer = Explorer::new(stdin.lock(), stdout.lock(), input_sender, output_receiver);

    let data = get_data_from_path(
        "/home/freyja/Documents/opensource/adventofcode/repair_droid/data/input.txt",
    );
    thread::spawn(move || {
        let mut program = Program::new(&data, input_receiver, output_sender);
        program.run();
    });

    explorer.start();

    // let mut explorer = Explorer::new()
}
