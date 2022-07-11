fn main() {
    let mut g = Grid::new();
    
    g.print();
    
    let num = read_placement().unwrap();
    
    g.plot(num, 'X');
    
    g.print();
    // match g.plot(0, 0, 'X') {
    //     Ok(_) => (),
    //     Err(plot) => {
    //         println!("Error! {:?} is out of bounds", plot);
    //     }
    // };
    // println!("{}", g.at(0, 0));
}

pub fn prompt_usize(maybe_text: Option<&str>) -> Result<usize, String> {
    use std::str::FromStr;
    let input = prompt(maybe_text);
    let num = match usize::from_str(&input) {
        Ok(n) => n,
        Err(_) => {
            return Err(input);
        }
    };
    
    Ok(num)
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

pub fn read_placement() -> Result<usize, &'static str>{
    match prompt_usize(Some("Enter cell number: ")) {
        Ok(n) => {
            if Grid::valid_plot(n) {
                Ok(n)
            } else {
                Err("That number is not one of the valid cell numbers.")
            }
        },
        Err(_) => Err("That is not a number.")
    }
}

pub fn grid_mapping_to_indexes(mapping: usize) -> [usize; 2] {
    let m = mapping - 1;
    
    [m / 3, m % 3]
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
    
    fn valid_plot(num: usize) -> bool {
        match num {
            1..=9 => true,
            _ => false
        }
    }
    
    pub fn print(&self) {
        println!("{}", "---------");
        for g_r in self.grid {
            print!("{}", "| ");
            for c in g_r {
                print!("{} ", c);
            }
            println!("{}", "|");
        }
        println!("{}", "---------");
    }
    
    pub fn new() -> Self {
        Self {
            grid: [
                ['_', '_', '_'],
                ['_', '_', '_'],
                ['_', '_', '_']
            ]
        }
    }
}
