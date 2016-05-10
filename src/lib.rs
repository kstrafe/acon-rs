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

impl FromStr for Acon {
	type Err = u32;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut table = Table::new();
		for line in s.lines() {
			let line = line.trim();
			let mut line = line.split_whitespace();
			if let Some(word) = line.next() {
				if word == "{" {
				} else if word == "[" {
				} else {
					// Normal word
					let acc = String::new();
					let sum = line.fold("".to_string(), |acc, x| acc + " " + x);
					let sum = sum.trim();
					println!("Key: {}, value: '{}'", word, sum);
					table.insert(word.to_string(), Value::String(sum.to_string()));
				}
			}
		}
		Ok(Acon { table: table })
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
