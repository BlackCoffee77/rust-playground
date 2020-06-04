enum Movement {
	// variants
	Up,
	Down,
	Left,
	Right,
}

fn move_player(m: Movement) {
	match m {
		Movement::Up => println!("Up"),
		Movement::Down => println!("Down"),
		Movement::Left => println!("Left"),
		Movement::Right => println!("Right"),

	}
}

pub fn run() {
	let player1 = Movement::Left;
	move_player(player1);
	let player2 = Movement::Right;
	move_player(player2);
}
