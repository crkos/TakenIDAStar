use rand::Rng;
use std::cmp::min;

const N: usize = 4;
const MAX_DEPTH: i32 = 100;
const INF: i32 = i32::MAX;


#[derive(Clone, PartialEq, Eq, Hash, Copy)]
struct State {
    board: [[i32; N]; N],
    x: usize,
    y: usize,
}

impl State {
    fn new() -> Self {
        // inicializa el tablero en orden
        let mut board = [[0; N]; N];
        let mut val = 1;
        for i in 0..N {
            for j in 0..N {
                board[i][j] = val;
                val += 1;
            }
        }
        board[N - 1][N - 1] = 0;
        Self {
            board,
            x: N - 1,
            y: N - 1,
        }
    }


    fn print(&self) {
        for i in 0..N {
            for j in 0..N {
                print!("{}   ", self.board[i][j]);
            }
            println!();
        }
        println!();
    }
}

fn distancia_manhattan(state: &State) -> i32 {
    let mut distance = 0;
    for i in 0..N {
        for j in 0..N {
            let value = state.board[i][j];
            if value != 0 {
                let target_x = (value - 1) as usize / N;
                let target_y = (value - 1) as usize % N;
                distance += (i as i32 - target_x as i32).abs() + (j as i32 - target_y as i32).abs();
            }
        }
    }
    distance
}

fn successors(state: &State) -> Vec<State> {
    let mut succ = Vec::new();
    let x = state.x;
    let y = state.y;
    let board = state.board.clone(); // create a new board vector
    if x > 0 {
        let mut s = State {
            board: board.clone(), // use the new board vector
            x: x - 1,
            y,
        };
        s.board[x][y] = board[x - 1][y];
        s.board[x - 1][y] = board[x][y];
        succ.push(s);
    }
    if x < N - 1 {
        let mut s = State {
            board: board.clone(), // use the new board vector
            x: x + 1,
            y,
        };
        s.board[x][y] = board[x + 1][y];
        s.board[x + 1][y] = board[x][y];
        succ.push(s);
    }
    if y > 0 {
        let mut s = State {
            board: board.clone(), // use the new board vector
            x,
            y: y - 1,
        };
        s.board[x][y] = board[x][y - 1];
        s.board[x][y - 1] = board[x][y];
        succ.push(s);
    }
    if y < N - 1 {
        let mut s = State {
            board: board.clone(), // use the new board vector
            x,
            y: y + 1,
        };
        s.board[x][y] = board[x][y + 1];
        s.board[x][y + 1] = board[x][y];
        succ.push(s);
    }
    succ
}

fn search(
    state: State,
    g: i32,
    bound: i32,
    heuristic: fn(&State) -> i32,
    solution: &mut Vec<State>,
    found_solution: &mut bool,
) -> i32 {
    let f = g + heuristic(&state);
    if f > bound {
        return f;
    }
    if heuristic(&state) == 0 {
        *found_solution = true;
        solution.push(state);
        return f;
    }
    let mut min_t = INF;
    let succ = successors(&state);
    for s in succ {
        if s == state {
            continue;
        }
        let t = search(s, g + 1, bound, heuristic, solution, found_solution);
        if *found_solution {
            solution.push(state);
            return t;
        }
        min_t = min(min_t, t);
    }
    min_t
}

fn ida_star_search(initial: &State, heuristic: fn(&State) -> i32) -> Vec<State> {
    let mut solution = Vec::new();
    let mut bound = heuristic(&initial);
    loop {
        let mut found_solution = false;
        let t = search(
            initial.clone(),
            0,
            bound,
            heuristic,
            &mut solution,
            &mut found_solution,
        );
        if found_solution {
            solution.reverse();
            return solution;
        } else if bound == INF {
            return solution;
        } else {
            bound = t;
        }
    }
}


fn main() {
    let mut initial = State::new();
    let mut rng = rand::thread_rng();
    for _ in 0..MAX_DEPTH {
        let succ = successors(&initial);
        let rand_index = rng.gen_range(0..succ.len());
        initial = succ[rand_index].clone();
    }
    println!("Tablero inicial:");
    initial.print();
    let solution = ida_star_search(&initial, distancia_manhattan);
    if solution.is_empty() {
        println!("No se encontró solución");
    } else {
        println!("Solucion encontrada con costo {}: ", solution.len() - 1);
        for s in solution {
            s.print();
        }
    }
}
