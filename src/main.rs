use std::io;

struct HanoiGame {
    num_discs: u32,
    rods: [Vec<u32>; 3],
}

impl HanoiGame {
    fn new(num_discs: u32) -> Self {
        let mut rods = [Vec::new(), Vec::new(), Vec::new()];
        rods[0] = (1..=num_discs).rev().collect();
        HanoiGame { num_discs, rods }
    }
    fn print_game(&self) {
        for i in 1..=self.num_discs {
            for j in 0..3 {
                let disc = self.rods[j].iter().position(|&x| x == i);
                match disc {
                    Some(_) => print!("{} ", i),
                    None => print!("  "),
                }
            }
            println!("");
        }
        println!("---------");
    }
    fn is_complete(&self) -> bool {
        self.rods[2] == (1..=self.num_discs).rev().collect::<Vec<_>>()
    }
    fn move_disc(&mut self, from: usize, to: usize) -> Result<(), &'static str> {
        if from > 2 || to > 2 {
            return Err("Invalid rod index");
        }
    
        let disc = self.rods[from].pop();
        match disc {
            Some(disc) => {
                let top_disc = self.rods[to].last().cloned();
                match top_disc {
                    Some(top_disc) => {
                        if top_disc > disc {
                            self.rods[to].push(disc);
                            Ok(())
                        } else {
                            self.rods[from].push(disc);
                            Err("Cannot place a larger disc on top of a smaller one")
                        }
                    }
                    None => {
                        self.rods[to].push(disc);
                        Ok(())
                    }
                }
            }
            None => Err("No disc to move"),
        }
    }    
    fn move_discs(&mut self, num_discs: u32, from: usize, to: usize, aux: usize) -> Result<(), &'static str> {
        if num_discs == 0 {
            return Ok(());
        }
    
        self.move_discs(num_discs - 1, from, aux, to)?;
        self.move_disc(from, to)?;
        self.print_game();
        self.move_discs(num_discs - 1, aux, to, from)?;

        Ok(())
    }
    
}



fn main() {
    println!("Welcome to the Tower of Hanoi game!");
    println!("The objective of the game is to move all of the discs from the starting rod to the destination rod, following these rules:");
    println!("- Only one disc can be moved at a time.");
    println!("- Each disc must be placed on top of a larger disc, or on an empty rod.");

    println!("Enter the number of discs:");

    let mut num_discs_str = String::new();
    io::stdin()
        .read_line(&mut num_discs_str)
        .expect("Failed to read line");
    let num_discs: u32 = num_discs_str.trim().parse().expect("Invalid input");

    let mut game = HanoiGame::new(num_discs);
    game.move_discs(num_discs, 0, 2, 1).expect("Error solving game");

    
    if game.is_complete(){
        println!("Solution:");
        game.print_game();
    } else {
        println!("Could not solve the game:");
        game.print_game();
    }
}

#[cfg(test)]
mod tests;