use std::io;

fn main() {
    let mut board = vec![' '; 9]; // Tabuleiro vazio
    let mut player_turn = 'X';

    loop {
        print_board(&board);

        println!("Jogador {}, é sua vez. Escolha um número de 1 a 9:", player_turn);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erro ao ler a linha");

        let position: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrada inválida. Por favor, escolha um número de 1 a 9.");
                continue;
            }
        };

        if position < 1 || position > 9 || board[position - 1] != ' ' {
            println!("Posição inválida. Tente novamente.");
            continue;
        }

        board[position - 1] = player_turn;

        if check_winner(&board, player_turn) {
            print_board(&board);
            println!("Parabéns! Jogador {} venceu!", player_turn);
            break;
        }

        if board.iter().all(|&cell| cell != ' ') {
            print_board(&board);
            println!("Empate! O jogo acabou.");
            break;
        }

        player_turn = if player_turn == 'X' { 'O' } else { 'X' };
    }
}

fn print_board(board: &Vec<char>) {
    println!(" 1 | 2 | 3 ");
    println!("-----------");
    println!(" 4 | 5 | 6 ");
    println!("-----------");
    println!(" 7 | 8 | 9 ");
    println!("");

    for (i, cell) in board.iter().enumerate() {
        print!(" {} ", cell);

        if (i + 1) % 3 == 0 {
            println!("");
            if i < 8 {
                println!("-----------");
            }
        } else {
            print!("|");
        }
    }

    println!("");
}

fn check_winner(board: &Vec<char>, player: char) -> bool {
    // Verifica as linhas, colunas e diagonais
    for i in 0..3 {
        if (board[i] == player && board[i + 3] == player && board[i + 6] == player)
            || (board[i * 3] == player && board[i * 3 + 1] == player && board[i * 3 + 2] == player)
        {
            return true;
        }
    }

    // Verifica as diagonais
    if (board[0] == player && board[4] == player && board[8] == player)
        || (board[2] == player && board[4] == player && board[6] == player)
    {
        return true;
    }

    false
}
