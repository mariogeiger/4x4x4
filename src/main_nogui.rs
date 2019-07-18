extern crate negamax;
extern crate time;

mod state;

use negamax::GameState;

// +1 player
fn human(x: &mut state::State) -> bool {
    let mut mov = String::new();
    std::io::stdin()
        .read_line(&mut mov)
        .expect("Failed to read line");

    let mov: i32 = match mov.trim().parse() {
        Ok(num) => num,
        Err(_) => return false,
    };

    let mx = mov / 10 - 1;
    let my = mov % 10 - 1;

    mx >= 0 && mx < 4 && my >= 0 && my < 4 && x.add(mx as usize, my as usize, 1)
}

// -1 player
fn robot(x: &mut state::State, table: &mut negamax::Table<state::State>) -> bool {
    println!("...");

    let mut best_value = -std::i32::MAX;
    let mut alpha = -std::i32::MAX;
    let beta = std::i32::MAX;

    let t0 = time::precise_time_s();

    let possibilities = x.possibilities(-1);
    let n = possibilities.len();
    let mut i = 0;
    for y in possibilities {
        println!("{}/{}...", i, n);
        let v = -y.negamax_table(1, 5, -beta, -alpha, table);
        if v > best_value {
            best_value = v;
            *x = y;
        }
        if v > alpha {
            alpha = v;
        }
        i += 1;
    }

    table.clean();

    let t1 = time::precise_time_s();

    println!(
        "value={} {:.2} seconds {} values into table",
        best_value,
        t1 - t0,
        table.len()
    );

    true
}

fn main() {
    let mut x = state::State::new();
    let mut table = negamax::Table::new();

    let mut hist: Vec<state::State> = Vec::new();
    hist.push(x.clone());

    println!("human begin ?[y/n]");
    let mut yn = String::new();
    std::io::stdin()
        .read_line(&mut yn)
        .expect("Failed to read line");

    if yn.trim() == "n".to_string() {
        robot(&mut x, &mut table); // player -1
    }

    loop {
        println!("{} {}", x, x.value());

        if x.win(-1) {
            println!("machine won");
            break;
        }

        let ok = human(&mut x); // player +1

        if !ok {
            x = hist.pop().expect("empty history");
            println!("cancel last move");
            continue;
        }

        println!("{} {}", x, x.value());

        if x.win(1) {
            println!("the human won");
            break;
        }

        robot(&mut x, &mut table); // player -1

        hist.push(x.clone());
    }
}
