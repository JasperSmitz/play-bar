use mpris::{self, PlayerFinder};

fn main() {
    
    let player_finder = match PlayerFinder::new() {
        Ok(f) => f,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let current_player = match player_finder.find_active() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    println!("player: {:#?}", current_player);


}
