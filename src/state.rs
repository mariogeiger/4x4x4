use std;
use std::cmp::Ordering;
use table;

pub struct State([i32; 4 * 4 * 4]);
/* 0---------------> x
   | 0  1  2  3
   | 4  5  6  7
   | 8  9  10 11
   | 12 13 14 15
   |
   v y               */

impl Clone for State {
    fn clone(&self) -> State {
        State(self.0)
    }
}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        for i in 0..4 * 4 * 4 {
            if self.0[i] != other.0[i] {
                return false;
            }
        }
        true
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        for i in 0..4 * 4 * 4 {
            if self.0[i] < other.0[i] {
                return Some(Ordering::Less);
            } else if self.0[i] > other.0[i] {
                return Some(Ordering::Greater);
            }
        }
        Some(Ordering::Equal)
    }
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        for i in 0..4 * 4 * 4 {
            if self.0[i] < other.0[i] {
                return Ordering::Less;
            } else if self.0[i] > other.0[i] {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    }
}

impl State {
    pub fn new() -> State {
        State([-1; 4 * 4 * 4])
    }
    // -1 : empty
    // 0 1 : players
    pub fn get(&self, x: usize, y: usize, z: usize) -> i32 {
        self.0[x + 4 * y + 16 * z]
    }
    pub fn add(&mut self, x: usize, y: usize, player: i32) -> bool {
        for z in 0..4 {
            if self.get(x, y, z) == -1 {
                self.0[x + 4 * y + 16 * z] = player;
                return true;
            }
        }
        false
    }

    pub fn possibilities(&self) -> Vec<(usize, usize)> {
        let mut r = Vec::new();
        for x in 0..4 {
            for y in 0..4 {
                if self.get(x, y, 3) == -1 {
                    r.push((x, y));
                }
            }
        }
        r
    }

    pub fn win(&self, player: i32) -> bool {
        'outer: for line in LINES.iter() {
            for i in 0..4 {
                if self.0[line[i]] != player {
                    continue 'outer;
                }
            }
            return true;
        }
        false
    }

    pub fn swap(&mut self) {
        for i in 0..4 * 4 * 4 {
            self.0[i] = if self.0[i] == 1 {
                0
            } else if self.0[i] == 0 {
                1
            } else {
                -1
            }
        }
    }

    pub fn symmetry(&self, id: usize) -> State {
        let mut x = State::new();

        for z in 0..4 {
            for i in 0..16 {
                x.0[16 * z + i] = self.0[16 * z + SYMMETRIES[id][i]];
            }
        }

        x
    }

    // compute the value in player 0 perspective
    pub fn value(&self) -> i32 {
        // 1        - 1 on a row
        // 76       - 2 on a row
        // 76*76    - 3 on a row
        // 76*76*76 - 4 on a row
        let mut v = 0;

        for line in LINES.iter() {
            let mut c0 = 0;
            let mut c1 = 0;
            for i in 0..4 {
                if self.0[line[i]] == 0 {
                    c0 += 1;
                }
                if self.0[line[i]] == 1 {
                    c1 += 1;
                }
            }
            if c1 == 0 {
                if c0 == 1 {
                    v += 1;
                }
                if c0 == 2 {
                    v += 76;
                }
                if c0 == 3 {
                    v += 76 * 76;
                }
                if c0 == 4 {
                    v += 76 * 76 * 76;
                }
            }
            if c0 == 0 {
                if c1 == 1 {
                    v -= 1;
                }
                if c1 == 2 {
                    v -= 76;
                }
                if c1 == 3 {
                    v -= 76 * 76;
                }
                if c1 == 4 {
                    v -= 76 * 76 * 76;
                }
            }
        }
        v
    }

    // returns a score in favor of player `player` in `player` perspective (higher is better)
    // do not look for score smaller than `alpha`
    // do not look for score bigger than `beta`
    pub fn negamax(&self, player: i32, depth: i32, mut alpha: i32, beta: i32) -> i32 {
        if depth == 0 || self.win(1 - player) {
            return (1 - 2 * player) * self.value();
        }

        let mut best_value = -std::i32::MAX;

        for mov in self.possibilities() {
            let mut child = self.clone();
            child.add(mov.0, mov.1, player);

            let v = -child.negamax(1 - player, depth - 1, -beta, -alpha);

            if v > best_value {
                best_value = v;
            }
            if v > alpha {
                alpha = v;
            }
            if alpha >= beta {
                break;
            }
        }
        best_value
    }

    pub fn negamax_table(
        &self,
        player: i32,
        depth: i32,
        mut alpha: i32,
        mut beta: i32,
        table: &mut table::Table,
    ) -> i32 {
        if depth == 0 || self.win(1 - player) {
            return (1 - 2 * player) * self.value();
        }

        if depth <= 2 {
            return self.negamax(player, depth, alpha, beta);
        }

        if let Some(s) = table.get(self, player, depth, &mut alpha, &mut beta) {
            return s;
        }

        let orig_alpha = alpha;
        let orig_beta = beta;

        let mut best_value = -std::i32::MAX;

        for mov in self.possibilities() {
            let mut child = self.clone();
            child.add(mov.0, mov.1, player);

            let v = -child.negamax_table(1 - player, depth - 1, -beta, -alpha, table);

            if v > best_value {
                best_value = v;
            }
            if v > alpha {
                alpha = v;
            }
            if alpha >= beta {
                break;
            }
        }

        table.insert(
            self.clone(),
            player,
            depth,
            orig_alpha,
            orig_beta,
            best_value,
        );
        best_value
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = String::new();
        for x in 0..4 {
            if x > 0 {
                s.push_str("  ");
            }

            s.push_str(&(x + 1).to_string());
            s.push('(');
            for y in 0..4 {
                if y > 0 {
                    s.push('|');
                }
                for z in 0..4 {
                    match self.get(x, y, z) {
                        -1 => {
                            s.push(' ');
                        }
                        0 => {
                            s.push('o');
                        }
                        1 => {
                            s.push('x');
                        }
                        _ => {
                            s.push('?');
                        }
                    }
                }
            }
            s.push(')');
        }
        write!(f, "{}", s)
    }
}

// x + 4*y + 16*z
static LINES: [[usize; 4]; 76] = [
    // 3*2 faces
    // xy
    [0, 16, 32, 48],
    [4, 20, 36, 52],
    [8, 24, 40, 56],
    [12, 28, 44, 60],
    [1, 17, 33, 49],
    [5, 21, 37, 53],
    [9, 25, 41, 57],
    [13, 29, 45, 61],
    [2, 18, 34, 50],
    [6, 22, 38, 54],
    [10, 26, 42, 58],
    [14, 30, 46, 62],
    [3, 19, 35, 51],
    [7, 23, 39, 55],
    [11, 27, 43, 59],
    [15, 31, 47, 63],
    // xz
    [0, 4, 8, 12],
    [16, 20, 24, 28],
    [32, 36, 40, 44],
    [48, 52, 56, 60],
    [1, 5, 9, 13],
    [17, 21, 25, 29],
    [33, 37, 41, 45],
    [49, 53, 57, 61],
    [2, 6, 10, 14],
    [18, 22, 26, 30],
    [34, 38, 42, 46],
    [50, 54, 58, 62],
    [3, 7, 11, 15],
    [19, 23, 27, 31],
    [35, 39, 43, 47],
    [51, 55, 59, 63],
    // yz
    [0, 1, 2, 3],
    [16, 17, 18, 19],
    [32, 33, 34, 35],
    [48, 49, 50, 51],
    [4, 5, 6, 7],
    [20, 21, 22, 23],
    [36, 37, 38, 39],
    [52, 53, 54, 55],
    [8, 9, 10, 11],
    [24, 25, 26, 27],
    [40, 41, 42, 43],
    [56, 57, 58, 59],
    [12, 13, 14, 15],
    [28, 29, 30, 31],
    [44, 45, 46, 47],
    [60, 61, 62, 63],
    // 6*2 segments
    // diag+
    [0, 20, 40, 60],
    [1, 21, 41, 61],
    [2, 22, 42, 62],
    [3, 23, 43, 63],
    [0, 17, 34, 51],
    [4, 21, 38, 55],
    [8, 25, 42, 59],
    [12, 29, 46, 63],
    [0, 5, 10, 15],
    [16, 21, 26, 31],
    [32, 37, 42, 47],
    [48, 53, 58, 63],
    // diag-
    [12, 24, 36, 48],
    [13, 25, 37, 49],
    [14, 26, 38, 50],
    [15, 27, 39, 51],
    [12, 9, 6, 3],
    [28, 25, 22, 19],
    [44, 41, 38, 35],
    [60, 57, 54, 51],
    [3, 18, 33, 48],
    [7, 22, 37, 52],
    [11, 26, 41, 56],
    [15, 30, 45, 60],
    // 4*2 edges
    [0, 21, 42, 63],
    [3, 22, 41, 60],
    [12, 25, 38, 51],
    [15, 26, 37, 48],
];

/* Symmetry group
id : identity
mv : verical mirror
mh : horizontal mirror
cw : rotation clockwise
cc : counter clockwise
iv : iversion
d1 : diagonal mirror
d2 : second diagonal mirror
*/

static SYMMETRIES: [[usize; 16]; 8] = [
    // identity
    [
        00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15,
    ],
    // h-mirror
    [
        12, 13, 14, 15, 08, 09, 10, 11, 04, 05, 06, 07, 00, 01, 02, 03,
    ],
    // v-mirror
    [
        03, 02, 01, 00, 07, 06, 05, 04, 11, 10, 09, 08, 15, 14, 13, 12,
    ],
    // c-clockwise
    [
        03, 07, 11, 15, 02, 06, 10, 14, 01, 05, 09, 13, 00, 04, 08, 12,
    ],
    // clockwise
    [
        12, 08, 04, 00, 13, 09, 05, 01, 14, 10, 06, 02, 15, 11, 07, 03,
    ],
    // inversion
    [
        15, 14, 13, 12, 11, 10, 09, 08, 07, 06, 05, 04, 03, 02, 01, 00,
    ],
    // diagonal mirror
    [
        00, 04, 08, 12, 01, 05, 09, 13, 02, 06, 10, 14, 03, 07, 11, 15,
    ],
    // diagonal mirror
    [
        15, 11, 07, 03, 14, 10, 06, 02, 13, 09, 05, 01, 12, 08, 04, 00,
    ],
];
