use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

const NEIGHBOURS:[(i32,i32);8] = [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];
#[derive(Clone)]
enum Status { Dead, Alive}
#[derive(Clone)]
struct Pos {
    h: i32,
    w: i32
}
impl Pos {
    fn diff(&self, diff_h: i32, diff_w: i32) -> Pos {
        Pos { h: self.h + diff_h, w: self.w + diff_w }
    }
}
struct Grid {
    fields: Vec<Status>,
    height: i32,
    width: i32,
}
struct Game {
    grid: Grid,
    alive: Vec<char>,
    dead: Vec<char>,
}
impl Game {
    fn new(height: i32, width: i32, alive: Vec<char>, dead: Vec<char>) -> Game {
        Game { grid: Grid::new(height,width), alive, dead }
    }
}

impl Grid {
    fn new(height: i32, width: i32) -> Grid {
        println!("Grid.new({}, {})", &height, &width);
        Grid { fields: vec![Status::Dead; (height * width) as usize], height, width }
    }
    fn print(&self) {
        for h in 0..self.height {
            let mut line = String::new();
            for w in 0..self.width {
                let pos = Pos{h,w};
                line = line + match self.get(&pos) {
                    Status::Dead => ".",
                    Status::Alive => "O"
                }
            }
            println!("{}",line);
        }
    }
    fn vector_position(&self, pos: &Pos) -> usize {
        ((*pos).h * self.width + (*pos).w) as usize
    }
    fn set(&mut self, pos: &Pos, value: Status) {
        let pos_in_1d = self.vector_position(pos);
        self.fields[pos_in_1d] = value;
    }
    fn get(&self, pos: &Pos) -> Status {
        let pos_in_1d = self.vector_position(pos);
        self.fields[pos_in_1d].clone()
    }
    fn is_out_of_range(&self, pos: &Pos) -> bool {
        println!("is_out_of_range");
        if (*pos).h >= self.height || (*pos).h < 0 || (*pos).w >= self.width || (*pos).w < 0 { true } else { false }
    }
    fn is_alive(&self, pos: &Pos) -> bool {
        println!("is_alive");
        if self.is_out_of_range(pos) { return false }
        match self.get(pos) {
            Status::Alive => { true }
            Status::Dead => { false }
        }
    }
    fn count_neighbours(&self, pos: &Pos) -> usize {
        NEIGHBOURS.iter().map(|(diff_h, diff_w)| self.is_alive(&(*pos).diff(*diff_h, *diff_w)))
            .map(|is_alive| if is_alive { 1 } else { 0 }).sum()
    }
}
impl Game {
    fn new_value(&self, pos:&Pos) -> Status {
        let current_status = self.grid.get(pos);
        let code = match current_status {
            Status::Dead => &self.dead,
            Status::Alive => &self.alive
        };
        let neighbour_count = self.grid.count_neighbours(pos);
        match code[neighbour_count] {
            '1' => Status::Alive,
            '0' => Status::Dead,
            _ => panic!()
        }
    }
    fn new_round(&mut self) {
        let mut new_grid = Grid::new(self.grid.height, self.grid.width);
        for h in 0..self.grid.height {
            for w in 0..self.grid.width {
                let pos = Pos {h,w};
                new_grid.set(&pos, self.new_value(&pos));
            }
        }
        self.grid = new_grid;
    }
}
/**

 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn get_line() -> String {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line.trim_matches('\n').to_string()
}
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let height = parse_input!(inputs[0], i32);
    let width = parse_input!(inputs[1], i32);
    let n = parse_input!(inputs[2], i32);
    let alive = get_line();
    let dead = get_line();
    let mut game = Game::new(height, width, alive.chars().collect(), dead.chars().collect());
    for h in 0..height {
        let line = get_line();
        println!("ORIGINAL: {}", &line);
        for w in 0..width {
            let pos = Pos {h,w};
            let value = match line.chars().nth(w as usize).unwrap() {
                '.' => Status::Dead,
                'O' => Status::Alive,
                _ => panic!()
            };
            game.grid.set(&pos, value);
        }
    }
    game.grid.print();
    for i in 0..n {
        game.new_round();
    }

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    game.grid.print();
}
