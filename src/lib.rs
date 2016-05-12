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
		enum State { Table, Array }

		stack.push((State::Table, table));

		for line in lines {

			let (state, table);
			if let Some(last) = stack.last_mut() {
				state = &mut last.0;
				table = &mut last.1;
			} else {
				break;
			}

			let mut words = line.split_whitespace();

			match *state {
				State::Table => {
					if let Some(word) = words.next() {
						match word {
							"{" => {}
							"}" => {}
							"[" => {
								if let Some(word) = words.next() {
								} else {
								}
							}
							"]" => {}
							name @ _ => {
								let mut acc;
								acc = words.fold("".to_string(), |acc, x| acc + " " + x);
								let acc = acc.trim().to_string();
								table.insert(name.to_string(), Value::String(acc));
							}
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
