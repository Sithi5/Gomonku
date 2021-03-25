use crate::state::create_child;
use crate::state::State;
use std::cmp::Reverse;
pub fn negamax(mut state: &mut State, depth: i32, mut alpha: i32, beta: i32, color: i8) -> i32 {
    if depth != 0 {
        state.available_move = create_child(&mut state);
        state.available_move.sort_by_key(|d| Reverse(d.heuristic));
    }
    // println!("current state: {:?} current heuristic {} depth {}", state.current_move, state.heuristic, depth);
    if depth == 0 || state.available_move.len() == 0 {
        return state.heuristic * color as i32;
    }
    let mut value: i32 = -1000;
    let len = state.available_move.len();
    for child in 0..len {
        value = std::cmp::max(
            value,
            -negamax(
                &mut state.available_move[child],
                depth - 1,
                -beta,
                -alpha,
                -color,
            ),
        );
        alpha = std::cmp::max(alpha, value);
        if alpha >= beta {
            // println!("pruning");
            break;
        }
    }
    // println!("alpha {}  beta {}", alpha, beta);
    return value;
}

pub fn return_move(state: &State, heuristic: i32) -> ((isize, isize), i32) {
    // println!("heuristic ret = {:?}", heuristic);
    let len = state.available_move.len();
	print_heuristic_table(state);
    for child in 0..len {
        // println!(
        //     "child value = {:?}",
        //     (
        //         (state.available_move[child].current_move),
        //         state.available_move[child].heuristic
        //     )
        // );
        if state.available_move[child].heuristic == heuristic {
            return ((state.available_move[child].current_move), state.heuristic);
        }
    }
    return (
        (state.available_move[0].current_move),
        state.available_move[0].heuristic,
    );
}

pub fn print_heuristic_table(state: &State) {
	let len = state.available_move.len();
	let mut table: Vec<Vec<i32>> = vec![];
	let mut line : Vec<i32> = vec![];
	let mut xmax = 0;
	let mut xmin = 18;
	let mut ymax = 0;
	let mut ymin = 18;
	let mut trigger = 0;
	for idx in 0..len {
		println!("x {} y {}", state.available_move[idx].current_move.0, state.available_move[idx].current_move.1);
		xmax = std::cmp::max(state.available_move[idx].current_move.0,xmax);
		xmin = std::cmp::min(xmin,state.available_move[idx].current_move.0);
		ymax = std::cmp::max(ymax,state.available_move[idx].current_move.1);
		ymin = std::cmp::min(ymin,state.available_move[idx].current_move.1);
	}
	println!("xmax: {}, xmin: {}, ymax: {},ymin: {}", xmax,xmin,ymax,ymin);
	for x in xmin..xmax {
		for y in ymin..ymax {
			for idx in 0..len {
				if state.available_move[idx].current_move.0 == x && state.available_move[idx].current_move.1 == y {
					line.push(state.available_move[idx].heuristic);
					trigger = 1;
				}
			}
			if trigger == 0 {
				line.push(0);
			}
			else {
				trigger = 0;
			}
		}
	table.push(line);
	line = vec![];
	}
	println!("heuristic table:");
	for x in 0..table.len() {
		println!("{:?}", table[x]);
	}
}
//example of negamax alpha beta pruning
// function negamax(node, depth, α, β, color)
// 	is if depth = 0 or node is a terminal node then
// 		return color × the heuristic value of node

// 	childNodes := generateMoves(node)
// 	childNodes := orderMoves(childNodes)
// 	value := −∞
// 	foreach child in childNodes do
// 		value := max(value, −negamax(child, depth − 1, −β, −α, −color))
// 		α := max(α, value)
// 		if α ≥ β then
// 			break (* cut-off *)
// 	return value Negamax

// first call
// negamax(rootNode, depth, −∞, +∞, 1)

//negascout

// function pvs(node, depth, α, β, color) is
//     if depth = 0 or node is a terminal node then
//         return color × the heuristic value of node
//     for each child of node do
//         if child is first child then
//             score := −pvs(child, depth − 1, −β, −α, −color)
//         else
//             score := −pvs(child, depth − 1, −α − 1, −α, −color) (* search with a null window *)
//             if α < score < β then
//                 score := −pvs(child, depth − 1, −β, −score, −color) (* if it failed high, do a full re-search *)
//         α := max(α, score)
//         if α ≥ β then
//             break (* beta cut-off *)
//     return α
