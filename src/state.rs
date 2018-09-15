use negamax;
use std;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct State([i32; 4 * 4 * 4]);
/* 0---------------> x
   | 0  1  2  3
   | 4  5  6  7
   | 8  9  10 11
   | 12 13 14 15
   |
   v y               */

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
        State([0; 4 * 4 * 4])
    }
    // 0 : empty
    // +1 -1 : players

    pub fn get(&self, x: usize, y: usize, z: usize) -> i32 {
        self.0[x + 4 * y + 16 * z]
    }

    pub fn add(&mut self, x: usize, y: usize, player: i32) -> bool {
        for z in 0..4 {
            if self.get(x, y, z) == 0 {
                self.0[x + 4 * y + 16 * z] = player;
                return true;
            }
        }
        false
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
}

impl<'a> negamax::GameState<'a> for State {
    type It = Vec<State>;

    // compute the value in player +1 perspective
    fn value(&self) -> i32 {
        // 1        - 1 on a row
        // 76       - 2 on a row
        // 76*76    - 3 on a row
        // 76*76*76 - 4 on a row
        let mut v = 0;

        for line in LINES.iter() {
            let mut me = 0;
            let mut op = 0;
            for i in 0..4 {
                if self.0[line[i]] == 1 {
                    me += 1;
                }
                if self.0[line[i]] == -1 {
                    op += 1;
                }
            }
            if op == 0 {
                if me == 1 {
                    v += 1;
                }
                if me == 2 {
                    v += 76;
                }
                if me == 3 {
                    v += 76 * 76;
                }
                if me == 4 {
                    v += 76 * 76 * 76;
                }
            }
            if me == 0 {
                if op == 1 {
                    v -= 1;
                }
                if op == 2 {
                    v -= 76;
                }
                if op == 3 {
                    v -= 76 * 76;
                }
                if op == 4 {
                    v -= 76 * 76 * 76;
                }
            }
        }
        v
    }

    fn possibilities(&self, player: i32) -> Vec<State> {
        let mut r = Vec::new();
        for x in 0..4 {
            for y in 0..4 {
                if self.get(x, y, 3) == 0 {
                    let mut copy = self.clone();
                    copy.add(x, y, player);
                    r.push(copy);
                }
            }
        }
        r
    }

    fn win(&self, player: i32) -> bool {
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

    fn swap(&mut self) {
        for i in 0..4 * 4 * 4 {
            self.0[i] = -self.0[i];
        }
    }

    fn symmetries(&self) -> Vec<State> {
        let mut r = Vec::new();
        for i in 0..8 {
            r.push(self.symmetry(i));
        }
        r
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
                        0 => {
                            s.push(' ');
                        }
                        1 => {
                            s.push('+');
                        }
                        -1 => {
                            s.push('-');
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
