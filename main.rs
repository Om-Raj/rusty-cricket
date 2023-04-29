use rand::{distributions::WeightedIndex, prelude::*};
use std::io::{self, Write};

fn main() {
    println!("###############\n\nRUSTY CRICKET\n\n###############\n\nHow to play :-\n1) Enter a number from 0 to 6\n2) If the ball matches your hit then you are OUT\n3) If the ball is different then you score the same runs as you hit\n4) If you hit 0 and the ball misses, you get the same runs as the bowled ball");

    let mut score = 0u16;
    let mut rng = thread_rng();

    /* Intial Weights
     * Assumption: Player is equally likely to hit any run.
     * You can customise the initial weight to punish greedy players.
     * Example:- [(0, 3), (1, 1), (2, 1), (3, 1), (4, 3), (5, 4), (6, 4)]
     */
    let mut weights = [(0, 1), (1, 1), (2, 1), (3, 1), (4, 1), (5, 1), (6, 1)];

    let mut dist = WeightedIndex::new(weights.iter().map(|weight| weight.1)).unwrap();

    loop {
        let ball: u8 = weights[dist.sample(&mut rng)].0;
        print!("\nHit : ");
        io::stdout().flush().unwrap();

        // Input & Parsing
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("\nError in reading input!");
        let hit = match input.trim().parse::<u8>() {
            Ok(hit) => {
                if hit > 6 {
                    println!("\nNumber too large! Enter a valid run.");
                    continue;
                }
                hit
            }
            Err(err) => {
                println!("\nError in parsing!\n{}", err);
                continue;
            }
        };

        // Logic
        if hit == ball {
            println!("\n\nOh no! You're OUT...\nFINAL SCORE : {}", score);
            break;
        } else if hit == 0 {
            score += ball as u16;
            println!("Well played! You copied my {}\nScore : {}", ball, score);
        } else {
            score += hit as u16;
            println!("Nice hit! I bowled a {}\nScore : {}", ball, score);
        }

        // Weight Increment & Distribution updation
        weights[hit as usize].1 += 1;
        dist.update_weights(&[(hit as usize, &weights[hit as usize].1)])
            .unwrap();
    }
}
