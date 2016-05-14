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

		let mut root_table = Table::new();
		let mut stack = vec![];
		let mut lines = s.lines();

		struct Node {
			name: String,
			value: Value,
		}

		stack.push(Node {
			name: "".to_string(),
			value: Value::Table(root_table),
		});

		for line in lines {
			let mut words = line.split_whitespace();
			if let Some(top) = stack.last_mut() {
				match top.value {
					Value::Array(ref mut array) => {}
					Value::String(ref mut string) => {}
					Value::Table(ref mut table) => {}
				}
			}
		}

		if let Some(node) = stack.pop() {
			match node.value {
				Value::Array(array) => Err(AconError::Error),
				Value::String(string) => Err(AconError::Error),
				Value::Table(table) => Ok(Acon { table: table }),
			}
		} else {
			Err(AconError::Error)
		}
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
