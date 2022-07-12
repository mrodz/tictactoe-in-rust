fn main() {
    Game::new().start();
}

#[derive(Debug)]
struct Game {
    grid: Grid,
    count: u32
}

const MAX_TURNS: u32 = 8;

impl Game {
    fn turn(&self) -> char {
        if self.count % 2 == 0 { 'X' } else { 'O' }
    }

    pub fn start(&mut self) -> () {
        tutorial();    
        loop {
            if self.step() {
                break;
            }
        }
    }

    pub fn step(&mut self) -> bool {
        let symbol: char = self.turn();
        self.grid.print();

        println!("It's {}'s turn!", symbol);

        let num: usize = match read_placement(&self.grid) {
            Ok(n) => n,
            Err(msg) => {
                println!("{}\nPlease try again...\n", msg);
                return false;
            }
        };
    
        self.grid.plot(num, symbol);
    
        self.check_end()
    }

    fn increment_count(&mut self) {
        self.count = std::cmp::min(MAX_TURNS, self.count + 1);
    }

    pub fn check_end(&mut self) -> bool {
        match self.grid.get_winner(self.count) {
            GameStates::StillPlaying => {
                self.increment_count();
                return false;
            },
            win_state => {
                println!("{}", GameStates::win_msg(win_state));
            }
        }
        self.count = 0;
        true
    }

    pub fn new() -> Self {
        Self {
            grid: Grid::new(None),
            count: 0
        }
    }
}

fn tutorial() -> () {
    let gen_fn = |i| char::from_digit((i + 1) as u32, 10).unwrap();

    let tutorial_grid: Grid = Grid::new(Some(gen_fn));
    
    println!("Welcome to TicTacToe!\n");
    println!("Take a look at the example grid before we get started.");
    println!("Each number corresponds to a slot you can play on.\n");

    tutorial_grid.print();

    println!("\nWhen prompted, type and enter one of these numbers to make a move!\nGood Luck :)\n\n~~~\n");

}

pub fn prompt_usize(maybe_text: Option<&str>) -> Result<usize, String> {
    use std::str::FromStr;
    let input = prompt(maybe_text);

    match usize::from_str(&input) {
        Ok(n) => Ok(n),
        Err(_) => Err(input)
    }
}

pub fn prompt(maybe_text: Option<&str>) -> String {
    use std::io::{stdin,stdout,Write};
    
    let mut s: String = String::new();
    if let Some(text) = maybe_text {
        print!("{}", text);
    }
    
    let _ = stdout().flush();
    
    stdin().read_line(&mut s).expect("You did not enter a valid string");
    
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    
    println!();
    
    s
}

fn read_placement(target: &Grid) -> Result<usize, &str>{
    match prompt_usize(Some("Enter cell number: ")) {
        Ok(n) => target.valid_plot(n),
        Err(_) => Err("That is not a number.")
    }
}

pub fn grid_mapping_to_indexes(mapping: usize) -> [usize; 2] {
    let m = mapping - 1;
    
    [m / 3, m % 3]
}

enum GameStates {
    Win(char),
    Tie,
    StillPlaying
}

impl GameStates {
    fn win_msg(state: GameStates) -> String {
        match state {
            GameStates::StillPlaying => panic!("active game is not fit for a win message"),
            GameStates::Tie => String::from("The game is a tie!"),
            GameStates::Win(c) => format!("{} wins!", c)
        }
    }
}

#[derive(Debug)]
struct Grid {
    grid: [[char; 3]; 3]
}

impl Grid {
    fn _at_indexes(&self, x: usize, y: usize) -> char {
        self.grid[x][y]
    }
    
    pub fn at(&self, plotting: usize) -> char {
        let mapped: [usize; 2] = grid_mapping_to_indexes(plotting);
        self._at_indexes(mapped[0], mapped[1])
    }
    
    fn _plot_at_indexes(&mut self, x: usize, y: usize, c: char) -> char {
        let old = self.grid[x][y];
        self.grid[x][y] = c;
        old
    }
    
    pub fn plot(&mut self, plotting: usize, c: char) {
        let mapped: [usize; 2] = grid_mapping_to_indexes(plotting);

        self._plot_at_indexes(mapped[0], mapped[1], c);
    }
    
    pub fn valid_plot(&self, num: usize) -> Result<usize, &str> {
        if let 1..=9 = num {
            if self.at(num) == '_' { Ok(num) } else { Err("The slot is full!") }
        } else {
            Err("That number is not one of the valid cell numbers.")
        }
    }
    
    pub fn print(&self) {
        println!("---------");
        for g_r in self.grid {
            print!("| ");
            for c in g_r {
                print!("{} ", c);
            }
            println!("|");
        }
        println!("---------");
    }

    fn get_winner(&self, turns: u32) -> GameStates {
        //Straights
        for i in 0..3 {
            if self.grid[i][0] == self.grid[i][1] && self.grid[i][1] == self.grid[i][2] && self.grid[i][1] != '_' {
                return GameStates::Win(self.grid[i][1]);
            }
    
            if self.grid[0][i] == self.grid[1][i] && self.grid[1][i] == self.grid[2][i] && self.grid[1][i] != '_' {
                return GameStates::Win(self.grid[1][i]);
            }
        }
    
        //Diagonals
        if self.grid[1][1] != '_' {
            if self.grid[0][0] == self.grid[1][1] && self.grid[1][1] == self.grid[2][2] {
                return GameStates::Win(self.grid[1][1]);
            }
    
            if self.grid[2][0] == self.grid[1][1] && self.grid[1][1] == self.grid[0][2] {
                return GameStates::Win(self.grid[1][1]);
            }
        }
    
        // Is the Game still active?
        if turns < MAX_TURNS {
            return GameStates::StillPlaying;
        }
    
        return GameStates::Tie;
    }
    
    
    pub fn new(gen: Option<fn(usize) -> char>) -> Self {
        let mut grid: [[char; 3]; 3] = [['_'; 3]; 3];
    
        if let Some(gf) = gen {
            for g_r in 0..3 {
                for c in 0..3 {
                    grid[g_r][c] = gf(3 * g_r + c);
                }
            }
        }
        
        Self {
            grid: grid
        }
    }
}
