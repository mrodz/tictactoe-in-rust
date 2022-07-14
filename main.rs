/// Entrypoint to the program
fn main() {
    use tic_tac_toe::Game;
    let mut game: Game = Game::new();
    game.start();
}

mod tic_tac_toe {
    /// ### Abstraction for a Game of TicTacToe.
    /// ```no_run
    /// let mut g: Game = Game::new();
    /// g.start();
    /// ```
    /// to start a round.
    #[derive(Debug)]
    pub struct Game {
        grid: Grid,
        pub count: u32,
    }

    /// ### Maximum turns allowed in a game.
    /// Since a game's `count` starts at zero, this bound allows
    /// nine total moves for nine total slots.
    pub const MAX_TURNS: u32 = 8;

    impl Game {
        /// ### Start the game.
        /// Welcomes the player and ppens a game loop that will continue until either:
        /// - A player gets three tiles in a row
        /// - All tiles fill up
        /// ### This function cleans itself up, sanitizing the `Game` instance so it can be used again.
        pub fn start(&mut self) -> () {
            tutorial();
            loop {
                if self.step() {
                    break; // game is over
                }
            }
            self.clean_up();
        }

        /// Resets the grid
        fn clean_up(&mut self) -> () {
            self.grid = Grid::new(None);
            self.count = 0;
        }

        /// Individual step in the TicTacToe game.
        /// - Reads user input
        ///   - Handles errors accordingly
        /// - Plots the input
        /// - Applies end-of-game logic if applicable.
        ///
        /// Returns a boolean value: `true` if the game is over, otherwise `false`.
        fn step(&mut self) -> bool {
            let symbol: char = turn(self.count);
            self.grid.print();

            println!("It's {}'s turn!", symbol);

            let num: usize = match crate::inputs::read_placement(&self.grid) {
                Ok(n) => n,
                Err(msg) => {
                    println!("{}\nPlease try again...\n", msg);
                    return false; // void this step
                }
            };

            self.grid.plot(num, symbol);

            self.check_end()
        }

        /// Increments `count` with a ceiling of `MAX_TURNS`
        fn increment_count(&mut self) {
            self.count = std::cmp::min(MAX_TURNS, self.count + 1);
        }

        /// Returns false if the game is still underway; otherwise true.
        ///
        /// Will increment `count` if the game is not over.
        /// If the game has ended, will also print a win message.
        fn check_end(&mut self) -> bool {
            match self.grid.get_winner(self.count) {
                GameStates::StillPlaying => {
                    self.increment_count();
                    return false;
                }
                win_state => {
                    println!("~~~\n\nGAME OVER — {}", win_state.win_msg());
                    self.grid.print();
                    println!("\n")
                }
            }
            true
        }

        /// ### Constructor
        pub fn new() -> Self {
            Self {
                grid: Grid::new(None),
                count: 0,
            }
        }
    }

    /// ### Explains how to use the game + how input works.
    fn tutorial() -> () {
        let gen_fn = |i| char::from_digit((i + 1) as u32, 10).unwrap();

        let tutorial_grid: Grid = Grid::new(Some(gen_fn));

        println!("Welcome to TicTacToe!\n");
        println!("Take a look at the example grid before we get started.");
        println!("Each number corresponds to a slot you can play on.\n");

        tutorial_grid.print();

        println!("\nWhen prompted, type and enter one of these numbers to make a move!\nGood Luck :)\n\n~~~\n");
    }

    /// Shorthand for getting the current player.
    /// If count is even, player is 'X'; otherwise 'O'
    const fn turn(count: u32) -> char {
        if count & 1 == 0 {
            'X'
        } else {
            'O'
        }
    }

    /// Converts a plotting to an actual matrix index.
    ///
    /// ### Examples:
    /// ```no_run
    /// assert_eq!([2, 2], grid_mapping_to_indexes(5));
    /// assert_eq!([0, 0], grid_mapping_to_indexes(1));
    /// assert_eq!([1, 2], grid_mapping_to_indexes(6));
    /// ```
    const fn grid_mapping_to_indexes(mapping: usize) -> [usize; 2] {
        let m = mapping - 1;

        [m / 3, m % 3]
    }

    /// Used to represent the current state of the game.
    /// * `Win(char)` will contain the character of the winner.
    enum GameStates {
        Win(char),
        Tie,
        StillPlaying,
    }

    impl GameStates {
        /// Function to build an end-of-game message, given an end of game.
        ///
        /// Will panic if caller is `GameStates::StillPlaying`
        fn win_msg(&self) -> String {
            match self {
                GameStates::StillPlaying => panic!("active game is not fit for a win message"),
                GameStates::Tie => String::from("The game is a tie!"),
                GameStates::Win(c) => format!("{} wins!", c),
            }
        }
    }

    /// Abstraction for a character matrix.
    #[derive(Debug)]
    pub struct Grid {
        /// contents of the board
        grid: [[char; 3]; 3],
    }

    impl Grid {
        /// Accessor function
        /// ### Arguments
        /// * `x` - grid **ROW**
        /// * `y` - grid **COLUMN**
        fn at_indexes(&self, x: usize, y: usize) -> char {
            self.grid[x][y]
        }

        /// Abstraction for `at_indexes()`
        /// ### Arguments
        /// * `plotting` - number in range of (1..=9) mapped to a slot on the grid.
        pub fn at(&self, plotting: usize) -> char {
            let mapped: [usize; 2] = grid_mapping_to_indexes(plotting);
            self.at_indexes(mapped[0], mapped[1])
        }

        /// Setter function
        /// ### Arguments
        /// * `x` - grid **ROW**
        /// * `y` - grid **COLUMN**
        /// * `c` - new character
        fn plot_at_indexes(&mut self, x: usize, y: usize, c: char) -> char {
            let old = self.grid[x][y];
            self.grid[x][y] = c;
            old
        }

        /// Abstraction for `plot_at_indexes()`
        /// ### Arguments
        /// * `plotting` - number in range of (1..=9) mapped to a slot on the grid.
        /// * `c` - new character
        pub fn plot(&mut self, plotting: usize, c: char) -> char {
            let mapped: [usize; 2] = grid_mapping_to_indexes(plotting);
            self.plot_at_indexes(mapped[0], mapped[1], c)
        }

        /// Returns whether or not a plot is valid.
        ///
        /// A plot is valid if `num` fits the range of 1-9 (inclusive)
        /// and the requested slot is empty.
        pub fn valid_plot(&self, num: usize) -> Result<usize, &str> {
            if let 1..=9 = num {
                if self.at(num) == '_' {
                    Ok(num)
                } else {
                    Err("The slot is full!")
                }
            } else {
                Err("That number is not one of the valid cell numbers.")
            }
        }

        /// Decorative way to print the grid
        pub fn print(&self) -> () {
            println!("---------");
            for g_r in self.grid {
                print!("| ");
                for c in g_r {
                    print!("{} ", c);
                }
                println!("|");
            }
            println!("---------");
            // let _ = std::io::stdout().flush();
        }

        fn get_winner(&self, turns: u32) -> GameStates {
            //Straights
            for i in 0..3 {
                if self.grid[i][0] == self.grid[i][1]
                    && self.grid[i][1] == self.grid[i][2]
                    && self.grid[i][1] != '_'
                {
                    return GameStates::Win(self.grid[i][1]);
                }

                if self.grid[0][i] == self.grid[1][i]
                    && self.grid[1][i] == self.grid[2][i]
                    && self.grid[1][i] != '_'
                {
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
                GameStates::StillPlaying
            } else {
                GameStates::Tie
            }
        }

        /// ### Constructor
        /// Arguments:
        /// * `gen` - optional parameter; a function that outputs a character when given an index.
        ///
        /// Usage:
        /// ```no_run
        /// // grid filled with empty slots ('_')
        /// let grid_1 = Grid::new(None);
        ///
        /// // grid filled with numbers
        /// let grid_2 = Grid::new(Some(|i| char::from_digit(i as u32, 10).unwrap()))
        /// ```
        pub fn new(gen: Option<fn(usize) -> char>) -> Self {
            let mut grid: [[char; 3]; 3] = [['_'; 3]; 3];

            if let Some(gf) = gen {
                for g_r in 0..3 {
                    for c in 0..3 {
                        grid[g_r][c] = gf(3 * g_r + c);
                    }
                }
            }

            Self { grid: grid }
        }
    }
}

mod inputs {
    /// ### `prompt()`, except it parses for a `usize`
    pub fn prompt_usize(maybe_text: Option<&str>) -> Result<usize, String> {
        use std::str::FromStr;
        let input: String = prompt(maybe_text)
            .chars()
            .filter(|&c| char::is_numeric(c))
            .collect();

        match usize::from_str(&input) {
            Ok(n) => Ok(n),
            Err(_) => Err(input),
        }
    }

    /// ### Read from stdio.
    ///
    /// Resembles python's `input()` function.
    /// Usage:
    /// ```no_run
    /// let name = prompt(Some("Enter your name: "));
    /// println!("Hello {}!", name);
    /// ```
    pub fn prompt(maybe_text: Option<&str>) -> String {
        use std::io::{stdin, stdout, Write};

        let mut s: String = String::new();
        if let Some(text) = maybe_text {
            print!("{}", text);
        }

        let _ = stdout().flush();

        stdin()
            .read_line(&mut s)
            .expect("You did not enter a valid string");

        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }

        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }

        println!();

        s
    }

    /// ### Abstraction to get user input.
    ///
    /// Returns either an `Ok(usize)` with the placement or an `Err(str)` with the error message.
    ///
    /// Will only return `Ok(usize)` if the user input is a number and a valid placement.
    use crate::tic_tac_toe::Grid;
    pub fn read_placement(target: &Grid) -> Result<usize, &str> {
        match prompt_usize(Some("Enter cell number: ")) {
            Ok(n) => target.valid_plot(n),
            Err(_) => Err("That is not a number."),
        }
    }
}
