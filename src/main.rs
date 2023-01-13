use rand;
use std::{io::{Write, Stdout}};
use terminal::{Clear, Action, Attribute, Value, Retrieved, KeyEvent, Event, KeyCode, Color};


fn clear_screen(term : &mut terminal::Terminal<Stdout>) 
{
	term.act(Action::ClearTerminal(Clear::All)).unwrap();
	term.act(Action::MoveCursorTo(0,0)).unwrap();
}

fn write_welcome(term : &mut terminal::Terminal<Stdout>) 
{
	term.write_all(b"Welcome to the Multiplication Trainer\n").unwrap();
	term.write_all(b"Press").unwrap();
	term.act(Action::SetAttribute(Attribute::Bold)).unwrap();
	term.write_all(b" ESC ").unwrap();
	term.act(Action::SetAttribute(Attribute::Reset)).unwrap();
	term.flush_batch().unwrap();
	term.write_all(b"to exit the application\n").unwrap();
}

enum Input {
	ESC,
	INTRO,
	CHAR(char),
}
#[allow(unreachable_code)]
fn read_input_key(term : &mut terminal::Terminal<Stdout>) -> Input
{
	loop {
		let key = term.get(Value::Event(None)).unwrap();
		match key{
			Retrieved::Event(Some(Event::Key(KeyEvent{code: KeyCode::Esc, ..}))) =>
				return Input::ESC,
			Retrieved::Event(Some(Event::Key(KeyEvent{code: KeyCode::Enter, ..}))) =>
				return Input::INTRO,
			Retrieved::Event(Some(Event::Key(KeyEvent{code: KeyCode::Char(c), ..}))) => {
				match c {
					'0'..='9' => return Input::CHAR(c),
					_ => {}
				}
			}
			_ => {}
		}
	}
	unreachable!("Should never reach this point");
}

struct Problem{
	a : u16,
	b : u16,
}

fn generate_problem() -> Problem {
	Problem{
		a : rand::random::<u16>() % 10,
		b : rand::random::<u16>() % 10,
	}
}

fn start_problem(term: &mut terminal::Terminal<Stdout>) -> Problem {
	let problem = generate_problem();
	term.write_all(format!("{} x {} = ", problem.a, problem.b).as_bytes()).unwrap();
	term.flush_batch().unwrap();
	return problem;
}

enum UserInput
{
	STOP,
	VALUE(u16)
}

fn get_user_input(term: &mut terminal::Terminal<Stdout>) -> UserInput{
	let mut input_str = String::new();
	loop {
		match read_input_key(term) {
			Input::ESC => return UserInput::STOP,
			Input::CHAR(c) => {
				term.write_all(&[c as u8]).unwrap();
				term.flush_batch().unwrap();
				input_str.push(c);
			}
			Input::INTRO => {
				let user_input : u16 = input_str.parse().unwrap();
				return UserInput::VALUE(user_input);
			},
		}
	}
}

fn check_user_input(term: &mut terminal::Terminal<Stdout>, user_input: u16, problem: &Problem) -> bool {
	let correct_answer = problem.a * problem.b;

	if user_input == correct_answer {
		term.batch(Action::SetForegroundColor(Color::Green)).unwrap();
		term.write_all(b"\tCorrect!\n").unwrap();
		term.batch(Action::SetForegroundColor(Color::Reset)).unwrap();
		term.flush_batch().unwrap();
		return true;
	} else {
		term.batch(Action::SetForegroundColor(Color::Red)).unwrap();
		term.write_all(b"\tIncorrect! The correct asnwer was ").unwrap();
		term.act(Action::SetAttribute(Attribute::Bold)).unwrap();
		term.write_all(format!("{}\n", correct_answer).as_bytes()).unwrap();
		term.batch(Action::SetForegroundColor(Color::Reset)).unwrap();
		term.batch(Action::SetAttribute(Attribute::Reset)).unwrap();
		term.flush_batch().unwrap();
		return false;
	}
}

fn main() {
	let mut term = terminal::stdout();

	term.act(Action::EnableRawMode).unwrap();

	clear_screen(&mut term);

	write_welcome(&mut term);

	let mut problem = start_problem(&mut term);
	
	loop {
		match get_user_input(&mut term) {
			UserInput::STOP => break,
			UserInput::VALUE(user_value) => {
				check_user_input(&mut term, user_value, &problem);
				problem = start_problem(&mut term);
			},
		}
	}
	
	
	term.act(Action::SetAttribute(Attribute::Reset)).unwrap();
	term.act(Action::DisableRawMode).unwrap();
	term.write_all(b"\nBye bye!\n").unwrap();
	term.flush_batch().unwrap();
}
