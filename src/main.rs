
#[path = "checkers.rs"] mod checkers;

#[path = "bots/pick_first/checkers_bot.rs"] mod checkers_bot_v1;
#[path = "bots/minimax/checkers_bot.rs"] mod checkers_bot_v2;
#[path = "bots/alphabeta/checkers_bot.rs"] mod checkers_bot_v3;
#[path = "bots/attackdepth/checkers_bot.rs"] mod checkers_bot_v4;
#[path = "bots/checkbestfirst/checkers_bot.rs"] mod checkers_bot_v5;

fn bot_battle(mut board: checkers::Board, player_to_move: u8, bot_one: u32, bot_two: u32) -> u32 {
    board.print();

    let mut player = match player_to_move {
        checkers::BLACK => true,
        checkers::WHITE => false,
        _ => panic!("Invalid player"),
    };

    let mut current_bot = bot_one;

    let move_limit = 200;

    let mut move_count = 0;

    while move_count < move_limit {
        let color = match player {
            true => checkers::BLACK,
            false => checkers::WHITE,
        };

        let (move_to_make, board_score) = match current_bot {
            1 => checkers_bot_v1::pick_move(&mut board, color, 100),
            2 => checkers_bot_v2::pick_move(&mut board, color, 100),
            3 => checkers_bot_v3::pick_move(&mut board, color, 100),
            4 => checkers_bot_v4::pick_move(&mut board, color, 100),
            5 => checkers_bot_v5::pick_move(&mut board, color, 100),
            _ => panic!("Invalid bot number"),
        };

        board.make_move(&move_to_make);

        player = !player;

        if current_bot == bot_one {
            current_bot = bot_two;
        } else {
            current_bot = bot_one;
        }

        if board.search_black_all().len() == 0 {
            board.print();
            println!("White wins! {}", bot_two);
            return 2;
        }

        if board.search_white_all().len() == 0 {
            board.print();
            println!("Black wins! {}", bot_one);
            return 1;
        }

        move_count += 1;
    }

    if move_count == move_limit {
        board.print();
        println!("Draw!");
    }

    0
}

fn bot_battle_arena(games: u32, bot_one: u32, bot_two: u32) {
    let mut bot_one_wins = 0;
    let mut bot_two_wins = 0;
    let mut draws = 0;

    let mut index = 0;
    let mut boards = Vec::<(checkers::Board, u8)>::new();
    let mut board = checkers::Board::new();
    boards.push((board.clone(), checkers::BLACK));

    while (boards.len() as u32) < games {
        let (mut board, color) = boards[index].clone();

        let moves = match color {
            checkers::BLACK => board.search_black_all(),
            checkers::WHITE => board.search_white_all(),
            _ => panic!("Invalid color"),
        };

        for m in moves {
            let mut new_board = board.clone();
            new_board.make_move(&m);

            let new_color = match color {
                checkers::BLACK => checkers::WHITE,
                checkers::WHITE => checkers::BLACK,
                _ => panic!("Invalid color"),
            };

            boards.push((new_board, new_color));
        }

        index += 1;
    }

    let mut i = 0;
    for (b, c) in boards {
        let other_board = b.clone();

        let winner = bot_battle(b, c, bot_one, bot_two);

        match winner {
            0 => draws += 1,
            1 => bot_one_wins += 1,
            2 => bot_two_wins += 1,
            _ => panic!("Invalid winner"),
        }

        let winner = bot_battle(other_board, c, bot_two, bot_one);

        match winner {
            0 => draws += 1,
            1 => bot_two_wins += 1,
            2 => bot_one_wins += 1,
            _ => panic!("Invalid winner"),
        }

        println!("Game {} complete", i + 1);
        i += 1;
    }

    println!("Bot one wins: {}", bot_one_wins);
    println!("Bot two wins: {}", bot_two_wins);
    println!("Draws: {}", draws);
}

fn main() {
    println!("Hello, world!");

    let mut board = checkers::Board::new();

    let bot_battle_s = false;

    // get input from user to see if bot or player goes first, true for bot, false for player
    let mut bot = false;
    // get input from console to see if bot goes first
    let mut bot_input = String::new();
    std::io::stdin().read_line(&mut bot_input).unwrap();
    let bot_input = bot_input.trim();
    if bot_input == "true" {
        bot = true;
    }

    // get input from user to see if bot or player goes first, true for bot, false for player
    let mut player = false;
    let mut player_input = String::new();
    std::io::stdin().read_line(&mut player_input).unwrap();
    let player_input = player_input.trim();
    if player_input == "true" {
        player = true;
    }

    let bot_one = 2;
    let bot_two = 3;

    if bot_battle_s {
        bot_battle_arena(500, bot_one, bot_two);
        return;
    }

    while true {
        board.print();

        if bot {
            let (move_to_make, board_score) = match player {
                true => checkers_bot_v5::pick_move(&mut board, checkers::BLACK, 500),
                false => checkers_bot_v5::pick_move(&mut board, checkers::WHITE, 500),
            };

            println!("Bot move: ");
            move_to_make.print();

            println!("Board score: {}", board_score);

            board.make_move(&move_to_make);

            player = !player;
            bot = !bot;
            continue;
        }

        let moves = match player {
            true => board.search_black_all(),
            false => board.search_white_all(),
        };

        let mut i = 0;
        for m in moves.iter() {
            print!("{}: ", i);
            m.print();
            i += 1;
        }

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let index = input.parse::<usize>().unwrap();

        board.make_move(&moves[index]);

        player = !player;
        bot = !bot;
    }
}
