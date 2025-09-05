use colored::Colorize;
use std::io;
use std::io::Write;
use rand::Rng;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    
    if args[1] == "-h" || args[1] == "--help" {
        println!("{}", "\nOne Dimensional Cellular Automaton".bold());
        println!("1 - Black, 0 - White");
        println!("Use ca1d -help for help");
        println!("Usage: ca1d <cols> <steps> <init> <rule>");
        println!("Cols: Number of columns; length of the 1d grid");
        println!("Steps: Number of steps to simulate");
        println!("Init: Initial State of the Grid. Ex: 01001001. number of characters(0/1) must be equal to columns or 0 (for empty grid) and 1 (for full grid).");
        println!("Rule: Cellular Rule. Must be an integer between 1 to 127.");
        println!("Example of a Rule: 84 --- into binary --> 01010100");
        println!("left center right   result");
        println!(" 1     1     1         0");
        println!(" 0     1     1         1");
        println!(" 1     0     1         0");
        println!(" 0     0     1         1");
        println!(" 1     1     0         0");
        println!(" 0     1     0         1");
        println!(" 1     0     0         0");
        println!(" 0     0     0         0");
        return;
    }
    if args[1] == "makerule" {
        rule_maker();
        return;
    }
    if args.len() < 5 {
        println!("Invalid arguments. Run ca1d -h or ca1d --help for help.");
    }

    let cols: i32 = args[1].parse::<i32>().unwrap();
    let steps: i32 = args[2].parse::<i32>().unwrap();
    let initial_state = &args[3];
    let rule = dec_to_bin(args[4].parse::<i32>().unwrap());

    let mut grid: Vec<i32> =  Vec::new();
    if initial_state.chars().count() != cols as usize {
        if initial_state == "0" {
            for _ in 0..cols
            {
                grid.push(0);
            }
        }
        else if initial_state == "1"{
            for _ in 0..cols
            {
                grid.push(1);
            }
        }
        else if initial_state == "m"{

            for _ in 0..cols
            {
                grid.push(0);
            }
            grid[cols as usize / 2] = 1;
        }
        else if initial_state == "!m"{

            for _ in 0..cols
            {
                grid.push(1);
            }
            grid[cols as usize / 2] = 0;
        }
        else if initial_state == "r"{

            let mut rng = rand::rng();
            for _ in 0..cols
            {
                grid.push(rng.random_range(0..=1));
            }
        }
        else {
            println!("Initial State must be {} bits long", cols);
            return;
        }
    }
    else {
        for i in 1..cols as usize + 1
        {
            grid.push(initial_state[i - 1..i].parse::<i32>().unwrap());
        }
    }

    display_grid(&grid);

    for i in 0..steps
    {
        let mut new_grid = vec![0; cols as usize];
        for c in 0..cols as usize
        {
            let left = {
                if c > 0 {grid[c-1]} else {grid.last().unwrap().clone()}
            };
            let right = {
                if c < cols as usize - 1 {grid[c+1]} else {grid.first().unwrap().clone()}
            };
            let clr = format!("{}{}{}", left, grid[c], right);
            new_grid[c] = apply_rule(clr.as_str(), &rule);
        }
        grid = new_grid;
        display_grid(&grid);
    }
}

fn rule_maker()
{
    let combs = ["111", "011", "101", "001", "110", "010", "100", "000"];
    let mut rule = String::new();
    println!("left center right   result");
    for i in combs
    {
        print!(" {}     {}     {}         ", i.chars().nth(0).unwrap(), i.chars().nth(1).unwrap(), i.chars().nth(2).unwrap());
        io::stdout().flush().unwrap(); // Make sure the prompt prints before input

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        rule.push_str(&input.trim());
    }
    println!("Binary:- {}", rule);
    println!("Rule:- {}", bin_to_dec(rule.as_str()).unwrap_or(0));
}

fn dec_to_bin(n: i32) -> String
{
    let bin = String::from(format!("{n:b}"));
    let zeroes = "0".repeat(8 - bin.len());
    zeroes + bin.as_str()
}
fn bin_to_dec(bin: &str) -> Option<i32>
{
    match u32::from_str_radix(bin, 2) {
        Ok(decimal) => Some(decimal as i32),
        Err(e) => None,
    }
}

fn apply_rule(clr: &str, rule: &str) -> i32
{
    let res: &str =
        match clr {
            "111" => &rule[0..1],
            "011" => &rule[1..2],
            "101" => &rule[2..3],
            "001" => &rule[3..4],
            "110" => &rule[4..5],
            "010" => &rule[5..6],
            "100" => &rule[6..7],
            "000" => &rule[7..8],
            _ => ""
        };
    res.parse::<i32>().unwrap()
}


fn display_grid(grid: &Vec<i32>)
{
    let mut text = String::new();
    for i  in 0..grid.len()
    {
        if grid[i] == 0
        {
            text.push_str("██".white().to_string().as_str());
        }
        else if grid[i] == 1
        {
            text.push_str("██".black().to_string().as_str());
        }
    }
    println!("{}", text);
}
