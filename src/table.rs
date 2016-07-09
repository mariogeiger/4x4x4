use state::*;
use std::collections::BTreeMap;

enum Quality {
	Upperbound,
	Lowerbound,
	Exact
} 

struct TableEntry {
	value: i32,
	depth: i32,
	quality: Quality
}

struct Table {
	map: [BTreeMap<State, Vec<TableEntry>>; 2]
}

impl Table {
	pub fn new() -> Table {
		Table{ map: [BTreeMap::new(), BTreeMap::new()] }
	}
	
	pub fn get(&self, state: &State, player: i32, depth: i32, alpha: &mut i32, beta: &mut i32) -> Option<i32> {
		if let Some(vs) = self.map[player as usize].get(state) {
			for entry in vs.iter() {
				if entry.depth >= depth {
					match entry.quality {
						Quality::Exact => { return Some(entry.value); }
						Quality::Upperbound => {
							if entry.value < *beta { *beta = entry.value; }
						}
						Quality::Lowerbound => {
							if entry.value > *alpha { *alpha = entry.value; }
						}
					}
				
					if *alpha >= *beta { return Some(entry.value); }
				}
			}
		}
		None
	}
	
	pub fn insert(&mut self, state: State, player: i32, depth: i32, alpha: i32, beta: i32, score: i32) {	
		let entry = TableEntry{
			value: score,
			depth: depth,
			quality: 
				if score <= alpha { Quality::Upperbound }
				else if score >= beta { Quality::Lowerbound }
				else { Quality::Exact }
		};
		
		if let Some(vs) = self.map[player as usize].get_mut(&state) {
			vs.push(entry);
			return;
		}
		self.map[player as usize].insert(state, vec![entry]);
	}
	
	pub fn symmetrise(&mut self) {
		// Apply symmetries to populate the tables
	}
	
	pub fn clean(&mut self) {
		// remove useless entries
		
		// if a.depth >= b.depth {
		// 		if a.quality == Exact { remove b }
		//		if a.quality == Upperbound && b.quality == Upperbound { remove b }
		//		if a.quality == Lowerbound && b.quality == Lowerbound { remove b }
		// }
	}
}

/* Symmetry group

subgroup of S_{4*4}

il est possible que : SG_{16} = SG_{4} \otimes SG_{4}

=> subgroup of S_{4}

J'ai trouvé 8 éléments


id : identity
mv : verical mirror
mh : horizontal mirror
cw : rotation clockwise
cc : counter clockwise
iv : iversion
d1 : diagonal mirror
d2 : second diagonal mirror

*/
