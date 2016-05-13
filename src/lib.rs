use std::collections::BTreeMap;
use std::str::FromStr;

#[derive(PartialEq, Clone, Debug)]
pub enum Value {
	Array(Array),
	String(String),
	Table(Table),
}

pub type Array = Vec<Value>;
pub type Table = BTreeMap<String, Value>;

struct Acon {
	table: Table,
}

#[derive(PartialEq, Clone, Debug)]
enum AconError {
	Error
}

impl FromStr for Acon {
	type Err = AconError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		use std::str::{Lines, SplitWhitespace};

		let mut table = Table::new();
		let mut stack = vec![];
		let mut lines = s.lines();
		#[derive(Clone, Copy)]
		enum State { Array, Table }
		struct Node {
			name: String,
			state: State,
			table: Table,
		}

		stack.push(Node {
			name: "".to_string(),
			state: State::Table,
			table: table
		});

		fn get_state(stack: &Vec<Node>) -> Option<State> {
			if let Some(last) = stack.last() {
				Some(last.state)
			} else {
				None
			}
		};

		fn push_state(stack: &mut Vec<Node>, name: String, state: State) {

		}

		for line in lines {

			if let Some(state) = get_state(&stack) {

				let mut words = line.split_whitespace();

				match state {
					State::Table => {
						if let Some(word) = words.next() {
							match word {
								"{" => {}
								"}" => {}
								"[" => {}
								"]" => {}
								name @ _ => { }
							}
						}
					}
					State::Array => {
						if let Some(word) = words.next() {
							match word {
								"{" => {}
								"}" => {}
								"[" => {}
								"]" => {}
								name @ _ => {
								}
							}
						}
					}
				}

			} else {
				panic!("{}", "There is no state!");
			}
		}

		Err(AconError::Error)
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		use Acon;
		let value = r#"
		{ table dude
			element this is my element
			value this is my value
			[ array
				0 1 2 3
				4 5 6 7

			]
		}"#;
		let acon = value.parse::<Acon>().unwrap();
		println!("{:?}", acon.table);
	}
}
