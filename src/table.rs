use state::*;
use std::collections::BTreeMap;

#[derive(Clone, Copy, PartialEq)]
enum Quality {
    Upperbound,
    Lowerbound,
    Exact,
}

#[derive(Clone, Copy)]
struct TableEntry {
    value: i32,
    depth: i32,
    quality: Quality,
}

pub struct Table(BTreeMap<State, Vec<TableEntry>>);

impl Clone for Table {
    fn clone(&self) -> Table {
        Table(self.0.clone())
    }
}

impl Table {
    pub fn new() -> Table {
        Table(BTreeMap::new())
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        let mut x = 0;
        for (_, list) in self.0.iter() {
            x += list.len();
        }
        x
    }

    pub fn get(&self,
               state: &State,
               player: i32,
               depth: i32,
               alpha: &mut i32,
               beta: &mut i32)
               -> Option<i32> {
        if player == 1 {
            let mut cpy = state.clone();
            cpy.swap();

            return self.get(&cpy, 0, depth, alpha, beta);
        }

        if let Some(vs) = self.0.get(state) {
            for entry in vs.iter() {
                if entry.depth == depth {
                    match entry.quality {
                        Quality::Exact => {
                            return Some(entry.value);
                        }
                        Quality::Upperbound => {
                            if entry.value < *beta {
                                *beta = entry.value;
                            }
                        }
                        Quality::Lowerbound => {
                            if entry.value > *alpha {
                                *alpha = entry.value;
                            }
                        }
                    }

                    if *alpha >= *beta {
                        return Some(entry.value);
                    }
                }
            }
        }
        None
    }

    pub fn insert(&mut self,
                  mut state: State,
                  player: i32,
                  depth: i32,
                  alpha: i32,
                  beta: i32,
                  score: i32) {
        if player == 1 {
            // allways use the player 0 perspective
            state.swap();
        }

        let entry = TableEntry {
            value: score,
            depth: depth,
            quality: if score <= alpha {
                Quality::Upperbound // le score de `state` est de au maximum `score`
            } else if beta <= score {
                Quality::Lowerbound // le score de `state` est de au moins `score`
            } else {
                Quality::Exact
            },
        };

        for id in 0..8 {
            let symmetric = state.symmetry(id);
            if let Some(vs) = self.0.get_mut(&symmetric) {
                vs.push(entry);
                continue;
            }
            self.0.insert(symmetric, vec![entry]);
        }
    }

    #[allow(dead_code)]
    // Apply symmetries to populate the tables
    pub fn symmetrize(&mut self) {
        let mut buffer: Vec<(State, Vec<TableEntry>)> = Vec::new();

        for (state, list) in self.0.iter() {
            for id in 1..8 {
                let symmetric = state.symmetry(id);
                buffer.push((symmetric, list.clone()));
            }
        }

        for (state, mut list) in buffer {
            if let Some(vs) = self.0.get_mut(&state) {
                vs.append(&mut list);
                continue;
            }
            self.0.insert(state, list);
        }
    }

    // remove useless entries
    pub fn clean(&mut self) {
        for (_, list) in self.0.iter_mut() {
            let mut i = 0;
            'iloop: while i < list.len() {

                for j in 0..list.len() {
                    if i != j && list[j].depth >= list[i].depth &&
                       (list[j].quality == Quality::Exact || list[j].quality == list[i].quality) {
                        // `j` is better than `i`
                        list.swap_remove(i);
                        continue 'iloop;
                    }
                }

                // `i` is not that bad
                i += 1;
            }
        }
    }
}
