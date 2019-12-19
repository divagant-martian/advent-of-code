use intcode::get_data_from_path;
use intcode::program::{Int, Program};
use std::collections::{HashMap, HashSet};
use std::io::{self, Read, Write};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, color, cursor, style};

const INFINITY: Int = Int::max_value();

struct Explorer<R, W: Write> {
    robot: (Int, Int),
    target: (Int, Int),
    stdout: W,
    stdin: R,
    visited: HashMap<(Int, Int), Tile>,
    pending: Vec<(Int, Int)>,
    distances: HashMap<(Int, Int), Int>,
    predecesors: HashMap<(Int, Int), Int>,
    robot_out: Receiver<Int>,
    robot_in: Sender<Int>,
}

#[derive(Copy, Clone)]
enum Direction {
    Up = 1,
    Right = 4,
    Left = 3,
    Down = 2,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Tile {
    Robot,
    OpenSpace,
    Wall,
    Target,
}

impl<R: Read, W: Write> Explorer<R, W> {
    fn new(
        stdin: R,
        stdout: W,
        robot_in: Sender<Int>,
        robot_out: Receiver<Int>,
    ) -> Explorer<R, RawTerminal<W>> {
        Explorer {
            robot: (35, 10),
            target: (INFINITY, INFINITY),
            stdout: stdout.into_raw_mode().unwrap(),
            stdin: stdin,
            visited: HashMap::new(),
            pending: vec![],
            distances: HashMap::new(),
            predecesors: HashMap::new(),
            robot_in,
            robot_out,
        }
    }

    fn start(&mut self) {
        self.init();

        loop {
            // read a single byte from stdin
            let mut b = [0];
            self.stdin.read(&mut b).unwrap();

            match b[0] {
                b'a' => self.move_robot(Direction::Left),
                b's' => self.move_robot(Direction::Down),
                b'w' => self.move_robot(Direction::Up),
                b'd' => self.move_robot(Direction::Right),
                b'q' => return,
                _ => (),
            }

            self.stdout.flush().unwrap();
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
        self.update();
    }

    fn update(&mut self) {
        write!(self.stdout, "{}", cursor::Hide);
        self.stdout.flush().unwrap();
    }
    fn move_robot(&mut self, dir: Direction) {
        self.robot_in.send(dir as Int).unwrap();
        // 0: The repair droid hit a wall. Its position has not changed.
        // 1: The repair droid has moved one step in the requested direction.
        // 2: The repair droid has moved one step in the requested direction;
        //    its new position is the location of the oxygen system.

        if let Ok(out) = self.robot_out.recv() {
            match out {
                0 => self.mark(Tile::Wall, Some(dir)),
                1 => {
                    self.mark(Tile::OpenSpace, None); // remove the robot
                    self.robot = self.get_position(dir); // update the robots pos
                    self.mark(Tile::Robot, None);
                }
                2 => {
                    self.mark(Tile::OpenSpace, None); // remove the robot
                    self.robot = self.get_position(dir); // update robot's pos
                    self.target = self.robot; // set the target
                    self.mark(Tile::Robot, None);
                }
                _ => panic!("the robot sent {}", out),
            }
        }
        self.update();
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

        write!(self.stdout, "{}", cursor::Goto(x as u16, y as u16));
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
                );
            } else {
                // trophy icon
                write!(
                    self.stdout,
                    "{}{}{}",
                    color::Fg(cyan),
                    '๏',
                    color::Fg(color::Reset)
                );
            }
        } else {
            let icon = match kind {
                Tile::OpenSpace => '.',
                Tile::Robot => '*',
                Tile::Wall => '█',
                Tile::Target => '๏',
            };
            write!(self.stdout, "{}", icon);
        }
        write!(
            self.stdout,
            "{}",
            cursor::Goto(self.robot.0 as u16, self.robot.1 as u16)
        );
    }
}

impl<R, W: Write> Drop for Explorer<R, W> {
    fn drop(&mut self) {
        // When done, restore the defaults to avoid messing with the terminal.
        write!(
            self.stdout,
            "{}{}{}",
            clear::All,
            style::Reset,
            cursor::Goto(1, 1)
        )
        .unwrap();
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
    let thread_handle = thread::spawn(move || {
        let mut program = Program::new(&data, input_receiver, output_sender);
        program.run();
    });

    explorer.start();

    // let mut explorer = Explorer::new()
}
