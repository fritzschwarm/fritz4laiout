extern crate rasciigraph;
extern crate rand;

use rasciigraph::plot;
use rasciigraph::Config;
use rand::Rng;
use std::cmp;

struct Node {
	
	x: f64,
	y: f64,
}

fn random_nodes(n: usize) -> Vec<Node> {
	
	let mut nodes = vec![];
	
	let mut x: f64;
	let mut y: f64;
	
	for _i in 0..n {
		
		x = rand::random();
		y = rand::random();
		
		nodes.push(Node { x: x, y: y });
		
//		println!("Node {} : ( {:.2} , {:.2} )", _i, x, y);
	}
	
	return nodes;
}

fn measure_distance(nodes: &Vec<Node>, order: &Vec<usize>) -> f64 {

	let mut distance: f64 = 0.0;
	let mut next: usize;
	
	for i in 0..order.len() {
		
		if i < order.len() - 1 {
			
			next = i + 1; 
		} else {
			next = 0;
		}
		
		distance += ( (nodes[order[next]].x - nodes[order[i]].x).powf(2.0) +
		              (nodes[order[next]].y - nodes[order[i]].y).powf(2.0)   ).sqrt();
	}
	
	return distance;
}

fn tsp_serial(nodes: &Vec<Node>) -> Vec<f64> {
	
	let mut history = Vec::<f64>::default();
	let order: Vec<usize> = (0..nodes.len()).collect();
	
	history.push(measure_distance(nodes, &order));
	
	return history;
}

fn tsp_hill_climb(nodes: &Vec<Node>) -> Vec<f64> {
	
	const CONVERGENCE: f64 = 0.005;
	let min_iter: usize = cmp::max(nodes.len() / 10, 10);
	let max_iter: usize = cmp::min(1000 * nodes.len(), 100);
	
	let mut history = Vec::<f64>::default();
	let mut order: Vec<usize> = (0..nodes.len()).collect();
	let mut converged = false;
	let mut iteration: usize = 0;
	let mut distance: f64;
	let mut best: f64 = f64::MAX;
	let mut _changed: bool;
	let mut idx1: usize;
	let mut idx2: usize;
	let mut shift: usize;
	
	while !converged && iteration < max_iter {
		
		shift = rand::thread_rng().gen_range(1..nodes.len());
		
		idx1 = iteration % (nodes.len());
		idx2 = (idx1 + shift) % (nodes.len());
		
		if iteration > 0 {
			
			order.swap(idx1, idx2);
		}
		
		distance = measure_distance(nodes, &order);
		
		history.push(distance);

		if (iteration >= max_iter - 1) || ((iteration > min_iter) && (((distance - best)/distance).abs() < CONVERGENCE)) {
			
			converged = true;
			
			history.push(best);
		}

		if best <= distance {
			
			// Swap back if fit is worse
			order.swap(idx1, idx2);
			
			_changed = false;
			
		} else {
			
			_changed = true;
			
			best = distance;
			
//			if iteration > 0 {
//				println!("Swapped {} and {}", idx1, idx2);
//			}
		}
		
//		println!("{:4} {:?} : {:.6} (better: {:?})", iteration, order, distance, _changed);
		
		iteration += 1;
	}
	
	return history;
}

//fn tsp_simulated_annealing(nodes: &Vec<Node>) -> Vec<f64> {
//    todo!();
//}

fn main() {
	
	let n = 10;
	
	let nodes = random_nodes(n);
	
	let se_history = tsp_serial(&nodes);
	let hc_history = tsp_hill_climb(&nodes);
//	let sa_history = tsp_simulated_annealing(&nodes);
	
	println!("   Serial: {:.6}", se_history[0]);
	println!("Hillclimb: {:.6}", hc_history.last().unwrap());
	println!("{}", plot(hc_history,
	                    Config::default().with_offset(10).with_height(10).
	                    with_caption("".to_string())));
}
