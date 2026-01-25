use std::collections::HashMap;
use std::time::Instant;

/// A curious puzzle. The optimal strategy is often to sit back and watch.
/// This module simulates Tic-tac-toe games for display throughput testing.

#[derive(Clone, Copy, Debug)]
enum Cell {
    Empty,
    X,
    O,
}

#[derive(Clone)]
struct Board {
    cells: [Cell; 9],
}

impl Board {
    fn new() -> Self {
        Board {
            cells: [Cell::Empty; 9],
        }
    }

    fn winner(&self) -> Option<Cell> {
        // Winning lines: rows, columns, diagonals
        const LINES: [[usize; 3]; 8] = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];

        for line in &LINES {
            let [a, b, c] = *line;
            match (self.cells[a], self.cells[b], self.cells[c]) {
                (Cell::X, Cell::X, Cell::X) => return Some(Cell::X),
                (Cell::O, Cell::O, Cell::O) => return Some(Cell::O),
                _ => {}
            }
        }
        None
    }

    fn is_tie(&self) -> bool {
        self.winner().is_none() && self.cells.iter().all(|c| !matches!(c, Cell::Empty))
    }

    fn display(&self) {
        println!(
            " {} | {} | {}",
            self.cell_to_char(0),
            self.cell_to_char(1),
            self.cell_to_char(2)
        );
        println!("---+---+---");
        println!(
            " {} | {} | {}",
            self.cell_to_char(3),
            self.cell_to_char(4),
            self.cell_to_char(5)
        );
        println!("---+---+---");
        println!(
            " {} | {} | {}",
            self.cell_to_char(6),
            self.cell_to_char(7),
            self.cell_to_char(8)
        );
    }

    fn cell_to_char(&self, idx: usize) -> char {
        match self.cells[idx] {
            Cell::Empty => ' ',
            Cell::X => 'X',
            Cell::O => 'O',
        }
    }

    fn available_moves(&self) -> Vec<usize> {
        let mut v = Vec::with_capacity(9);
        for i in 0..9 {
            if matches!(self.cells[i], Cell::Empty) {
                v.push(i);
            }
        }
        v
    }
}

pub fn run_board_game() {
    println!("\n\n");

    let start = Instant::now();
    let num_games = 10_000_000;

    for game_num in 1..=num_games {
        // Display boards at intervals
        if game_num == 1 || game_num % 2000 == 0 {
            println!("--- Game {} ---", game_num);
            let mut board = Board::new();
            let seed = seed_from_time(game_num as u64);
            play_perfect_game(&mut board, Some(seed));
            board.display();
            println!();
        } else if game_num % 20 == 0 {
            print!(".");
        }
    }

    let elapsed = start.elapsed().as_secs_f64();
    let throughput = num_games as f64 / elapsed;

    println!("\n\nGames simulated: {}", num_games);
    println!("Time elapsed: {:.3}s", elapsed);
    println!("Throughput: {:.0} games/sec", throughput);

    println!("\n");
    println!("---------------------------------------------------------------------");
    println!("A CURIOUS PUZZLE. THE OPTIMAL STRATEGY IS OFTEN TO SIT BACK AND WATCH.");
    println!("---------------------------------------------------------------------");
    println!("\n");
}

fn seed_from_time(extra: u64) -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;
    let jitter = Instant::now().elapsed().as_nanos() as u64;
    now ^ jitter ^ extra
}

fn play_perfect_game(board: &mut Board, seed: Option<u64>) {
    *board = Board::new();
    let mut cache: HashMap<u32, i8> = HashMap::with_capacity(20000);
    let mut rng =
        SimpleRng::new(seed.unwrap_or_else(|| seed_from_time(board as *const Board as u64)));

    // Randomize the opening move
    let mut player = Cell::X;
    let avail = board.available_moves();
    if let Some(&pos) = avail.get((rng.next() as usize) % avail.len()) {
        board.cells[pos] = player;
        player = opponent(player);
    }

    // Play until all 9 cells are filled (forces a draw with perfect play)
    while !board.available_moves().is_empty() {
        let mv = choose_best_move(board, player, &mut cache);
        match mv {
            Some(pos) => {
                board.cells[pos] = player;
                player = opponent(player);
            }
            None => break,
        }
    }

    // With perfect play, should always end in a draw
    debug_assert!(
        board.winner().is_none(),
        "Perfect play should never produce a winner"
    );
    debug_assert!(
        board.is_tie(),
        "Perfect play should always reach a tie state"
    );
}

fn encode_board(board: &Board, player: Cell) -> u32 {
    // Ternary encoding for 9 cells: Empty=0, X=1, O=2; plus player bit X=0, O=1.
    let mut v: u32 = 0;
    for i in 0..9 {
        let d = match board.cells[i] {
            Cell::Empty => 0u32,
            Cell::X => 1u32,
            Cell::O => 2u32,
        };
        v = v * 3 + d;
    }
    let p = match player {
        Cell::X => 0u32,
        Cell::O => 1u32,
        Cell::Empty => 0u32,
    };
    v * 2 + p
}

fn choose_best_move(board: &Board, player: Cell, cache: &mut HashMap<u32, i8>) -> Option<usize> {
    let mut best_score = i8::MIN;
    let mut best_move = None;
    for pos in board.available_moves() {
        let mut next = board.clone();
        next.cells[pos] = player;
        let score = -minimax(&next, opponent(player), cache);
        if score > best_score {
            best_score = score;
            best_move = Some(pos);
        }
        // Prefer draw over win to embody the message; reorder scores: 0 > 1 > -1
        if best_score == 0 {
            break;
        }
    }
    best_move
}

fn opponent(p: Cell) -> Cell {
    match p {
        Cell::X => Cell::O,
        Cell::O => Cell::X,
        Cell::Empty => Cell::X,
    }
}

fn minimax(board: &Board, player: Cell, cache: &mut HashMap<u32, i8>) -> i8 {
    if let Some(w) = board.winner() {
        // Score from the perspective of the current player to move
        // If player won, it's good (+1), if opponent won, it's bad (-1)
        return match w {
            Cell::X => {
                if matches!(player, Cell::X) {
                    1 // X won and it's X's turn: X achieved victory
                } else {
                    -1 // X won and it's O's turn: bad for O
                }
            }
            Cell::O => {
                if matches!(player, Cell::O) {
                    1 // O won and it's O's turn: O achieved victory
                } else {
                    -1 // O won and it's X's turn: bad for X
                }
            }
            _ => 0,
        };
    }
    if board.is_tie() {
        return 0; // Draw is neutral for both
    }

    let key = encode_board(board, player);
    if let Some(&s) = cache.get(&key) {
        return s;
    }

    let mut best = i8::MIN;
    for pos in board.available_moves() {
        let mut next = board.clone();
        next.cells[pos] = player;
        let score = -minimax(&next, opponent(player), cache);
        if score > best {
            best = score;
        }
        if best == 1 {
            break; // Found a winning move, no need to search further
        }
    }
    cache.insert(key, best);
    best
}

// Minimal RNG for reproducible pseudo-random first move
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        SimpleRng { state: seed | 1 } // force odd seed to avoid degenerate cycles
    }

    fn next(&mut self) -> u64 {
        // 64-bit LCG parameters (Numerical Recipes style)
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_game_always_tie() {
        let mut board = Board::new();
        for seed in 0..32 {
            play_perfect_game(&mut board, Some(seed));
            assert!(
                board.is_tie(),
                "Perfect play should always result in a draw: {:?}",
                board.cells
            );
        }
    }

    #[test]
    fn test_board_initialization() {
        let board = Board::new();
        for cell in &board.cells {
            assert!(matches!(cell, Cell::Empty));
        }
    }

    #[test]
    fn test_board_is_tie() {
        let mut board = Board::new();
        assert!(!board.is_tie());

        // Create a tie situation (no winner, all cells filled)
        board.cells = [
            Cell::X,
            Cell::O,
            Cell::X,
            Cell::X,
            Cell::O,
            Cell::O,
            Cell::O,
            Cell::X,
            Cell::X,
        ];
        assert!(board.is_tie());
    }

    #[test]
    fn test_winner_rows() {
        let mut board = Board::new();
        // Test row win
        board.cells[0] = Cell::X;
        board.cells[1] = Cell::X;
        board.cells[2] = Cell::X;
        assert!(matches!(board.winner(), Some(Cell::X)));
    }

    #[test]
    fn test_winner_columns() {
        let mut board = Board::new();
        // Test column win
        board.cells[0] = Cell::O;
        board.cells[3] = Cell::O;
        board.cells[6] = Cell::O;
        assert!(matches!(board.winner(), Some(Cell::O)));
    }

    #[test]
    fn test_winner_diagonals() {
        let mut board = Board::new();
        // Test diagonal win
        board.cells[0] = Cell::X;
        board.cells[4] = Cell::X;
        board.cells[8] = Cell::X;
        assert!(matches!(board.winner(), Some(Cell::X)));

        // Test other diagonal
        let mut board2 = Board::new();
        board2.cells[2] = Cell::O;
        board2.cells[4] = Cell::O;
        board2.cells[6] = Cell::O;
        assert!(matches!(board2.winner(), Some(Cell::O)));
    }

    #[test]
    fn test_no_winner() {
        let board = Board::new();
        assert!(board.winner().is_none());

        // Incomplete game
        let mut board2 = Board::new();
        board2.cells[0] = Cell::X;
        board2.cells[1] = Cell::O;
        assert!(board2.winner().is_none());
    }

    #[test]
    fn test_simplerng_deterministic() {
        let mut rng1 = SimpleRng::new(42);
        let mut rng2 = SimpleRng::new(42);

        // Same seed should produce same sequence
        for _ in 0..10 {
            assert_eq!(rng1.next(), rng2.next());
        }
    }

    #[test]
    fn test_simplerng_different_seeds() {
        let mut rng1 = SimpleRng::new(1);
        let mut rng2 = SimpleRng::new(2);

        // Different seeds should produce different sequences
        let val1 = rng1.next();
        let val2 = rng2.next();
        assert_ne!(val1, val2);
    }

    #[test]
    fn test_board_available_moves() {
        let mut board = Board::new();
        let moves = board.available_moves();
        assert_eq!(moves.len(), 9);

        board.cells[0] = Cell::X;
        let moves = board.available_moves();
        assert_eq!(moves.len(), 8);
        assert!(!moves.contains(&0));
    }

    #[test]
    fn test_board_clone() {
        let mut board = Board::new();
        board.cells[4] = Cell::X;
        let cloned = board.clone();

        assert!(matches!(cloned.cells[4], Cell::X));
    }

    #[test]
    fn test_cell_to_char() {
        let board = Board::new();
        assert_eq!(board.cell_to_char(0), ' ');

        let mut board2 = Board::new();
        board2.cells[0] = Cell::X;
        board2.cells[1] = Cell::O;
        assert_eq!(board2.cell_to_char(0), 'X');
        assert_eq!(board2.cell_to_char(1), 'O');
    }
}
