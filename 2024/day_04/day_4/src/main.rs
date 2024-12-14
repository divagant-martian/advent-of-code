#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Xmas {
    x,
    m,
    a,
    s,
}

struct Grid {
    grid: Vec<Xmas>,
    cols: usize,
}

impl Grid {
    fn new(data: &str) -> Grid {
        let mut lines = data.lines();
        let mut grid = Vec::new();
        let mut cols: usize = 0;

        while let Some(line) = lines.next() {
            let mut row_cols: usize = 0;
            for c in line.chars() {
                let l: Xmas = match c {
                    'X' => Xmas::x,
                    'M' => Xmas::m,
                    'A' => Xmas::a,
                    'S' => Xmas::s,
                    _ => panic!("must be XMAS !"),
                };
                grid.push(l);
                row_cols += 1;
            }
            if (cols != 0) && cols != row_cols {
                panic!("not a rentangle!");
            }
            cols = row_cols;
        }

        Self { grid, cols }
    }

    fn get(&self, i: usize, j: usize) -> Option<Xmas> {
        let idx = i * self.cols + j;
        return self.grid.get(idx).copied();
    }

    fn get_with_offset(&self, i: usize, j: usize, delta_i: i8, delta_j: i8) -> Option<Xmas> {
        let new_i = pos_with_offset(i, delta_i)?;
        let new_j = pos_with_offset(j, delta_j)?;
        self.get(new_i, new_j)
    }

    fn is_xmas_in_direction(&self, i: usize, delta_i: i8, j: usize, delta_j: i8) -> bool {
        let Some(here) = self.get(i, j) else {
            return false;
        };
        if (here != Xmas::x) || ((delta_i == 0) && (delta_j == 0)) {
            return false;
        }
        let mut char_buf = vec!(Xmas::x);
        for (needle, scalar) in [Xmas::m, Xmas::a, Xmas::s].into_iter().zip(1i8..) {
            if let Some(letter) = self.get_with_offset(i, j, delta_i * scalar, delta_j * scalar) {
                if letter != needle {
                    return false;
                }
                char_buf.push(letter);
            } else {
                return false;
            }
        }
        print!("{char_buf:?} ");

        return true;
    }

    fn find_xmas(&self, i: usize, j: usize) -> usize {
        let mut count: usize = 0;
        for delta_i in [-1, 0, 1] {
            for delta_j in [-1, 0, 1] {
                if (delta_i, delta_j) != (-1, 0){
                    continue;
                }
// 
                if self.is_xmas_in_direction(i, delta_i, j, delta_j) {
                    if count == 0 {
                        print!("({i}, {j}) ");
                    }
                    count += 1;

                    print!("{} ", direction_char(delta_i, delta_j));
                }
            }
        }

        if count > 0 {
            print!("\n");
        }

        return count;
    }

    fn count_xmas(&self) -> usize {
        let rows = self.grid.len() / self.cols;
        let mut total: usize = 0;

        for i in 0..rows {
            for j in 0..self.cols {
                total += self.find_xmas(i, j);
            }
        }

        return total;
    }
}

fn pos_with_offset(i: usize, delta_i: i8) -> Option<usize> {
    if delta_i == 0 {
        return Some(i);
    } else if delta_i < 0 {
        i.checked_sub((delta_i * -1).try_into().expect("delta i overflow"))
    } else {
        i.checked_add(delta_i.try_into().unwrap())
    }
}

fn direction_char(delta_i: i8, delta_j: i8) -> &'static str {
    if (delta_i > 0) && (delta_j < 0) {
        return "⬋";
    } else if (delta_i == 0) && (delta_j < 0) {
        return "⬅";
    } else if (delta_i < 0) && (delta_j < 0) {
        return "⬉";
    } else if (delta_i < 0) && (delta_j == 0) {
        return "⬆";
    } else if (delta_i < 0) && (delta_j > 0) {
        return "⬈";
    } else if (delta_i == 0) && (delta_j > 0) {
        return "⮕";
    } else if (delta_i > 0) && (delta_j > 0) {
        return "⬊";
    } else {
        return "⬇";
    }
}

fn main() -> anyhow::Result<()> {
    let data = std::fs::read_to_string("input")?;

    let grid = Grid::new(&data);

    let total = grid.count_xmas();

    println!("{total}");

    Ok(())
}
