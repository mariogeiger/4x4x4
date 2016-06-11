mod state;

fn human(x:&mut state::State) -> bool {
	let mut mov = String::new();
	std::io::stdin().read_line(&mut mov)
		.expect("Failed to read line");

	let mov:i32 = match mov.trim().parse() {
		Ok(num) => num,
		Err(_)  => return false
	};

	let mx = mov / 10 - 1;
	let my = mov % 10 - 1;

	mx >= 0 && mx < 4 && my >= 0 && my < 4 && x.add(mx, my, 0)
}

fn robot(x:&mut state::State) -> bool {
	println!("...");

	let mut best_value = -std::i32::MAX;
	let mut alpha = -std::i32::MAX;
	let beta = std::i32::MAX;
	let mut best_mov = (0,0);

	for mov in x.possibilities() {
		let mut y = x.clone();
		y.add(mov.0, mov.1, 1);
		let v = -y.negamax(0, 7, -beta, -alpha);
		if v > best_value {
			best_value = v;
			best_mov = mov;
		}
		if v > alpha { alpha = v; }
	}
	println!("{}{} value = {}", best_mov.0 + 1, best_mov.1 + 1, best_value);

	x.add(best_mov.0, best_mov.1, 1);
	true
}

fn main() {
	let mut x = state::State::new();
	let mut hist : Vec<state::State> = Vec::new();
	hist.push(x.clone());

	println!("human begin ?[y/n]");
	let mut yn = String::new();
	std::io::stdin().read_line(&mut yn)
		.expect("Failed to read line");

	if yn.trim() == "n".to_string() {
		robot(&mut x); // player 1
	}

	loop {
		println!("{} {}", x, x.value());

		if x.win(1) {
			println!("player 1 win");
			break;
		}

		let ok = human(&mut x); // player 0

		if !ok {
			x = hist.pop().expect("empty history");
			println!("cancel last move");
			continue;
		}

		println!("{} {}", x, x.value());

		if x.win(0) {
			println!("player 0 win");
			break;
		}

		robot(&mut x); // player 1

		hist.push(x.clone());
	}
}
