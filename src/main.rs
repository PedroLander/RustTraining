use std::io;

fn main () {
    let mut time:u32 = 0;

    let mut player1 : Player = Player {
        position: (0, 0, 0),
        health: 100,
        max_health: 100,
        status: PlayerStatus::Awake,
    };

    loop {
        show_time(&time);
        show_status(&player1);
        show_options();

        let action : Actions = select_action();
        player1.perform_action(action);

        time += 1;
    }
}

#[derive(Debug)]
struct Player {
    position: (i32, i32, i32),
    health: u32,
    max_health: u32,
    status: PlayerStatus,
}

#[derive(Debug)]
enum PlayerStatus {
    Awake,
}

#[derive(Debug)]
enum Directions {
    North,
    South,
    East,
    West,
}

enum Actions {
    Wait,
    Walk(Directions),
}

fn show_time(time: &u32) -> () {
    println!("Time: {}", time);
}

fn show_status(player: &Player) -> () {
    println!("{:?}", player)
}

fn show_options() -> () {
    let options = ["1. Wait", "2. Walk"];
    for option in options {println!("{option}")};
}

fn get_input() -> u32 {
    let mut input_text = String::new();

    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => return i,
        Err(i) => {
            println!("{}, please write a number", i);
            get_input()
        },
    }
}

fn show_move_options() -> () {
    let options = ["1. North", "2. South", "3. East", "4. West"];
    for option in options {println!("{option}")};
}


impl Player {
    fn perform_action(&mut self, action: Actions) -> () {
        match action {
            Actions::Wait => (),
            Actions::Walk(a) => self.walk(&a),
                }
            }

    fn walk(&mut self, direction: &Directions) -> () {
        match direction {
            Directions::North=> self.position.0 += 1,
            Directions::South=> self.position.0 -= 1,
            Directions::East=> self.position.1 += 1,
            Directions::West=> self.position.1 += -1,
        }
    }
}

fn select_action() -> Actions {
    match get_input() {
        1 => {
            Actions::Wait
        }
        2 => {
            show_move_options();
            let move_input = get_input();
            match move_input {
                1 => {
                    Actions::Walk(Directions::North)
                },
                2 => {
                    Actions::Walk(Directions::South)
                },
                3 => {
                    Actions::Walk(Directions::East)
                },
                4 => {
                    Actions::Walk(Directions::West)
                },
                _ => {
                    println!("No a valid direction");
                    show_options();
                    select_action()
                },
            }
        }
        _ => {
            println!("No action selected");
            show_options();
            select_action()
        },
    }
}