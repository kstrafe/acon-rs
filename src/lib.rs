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
		let mut stack = vec![];
		let mut lines = s.lines();

		stack.push(Node {
			name: "".to_string(),
			value: Value::Table(Table::new()),
		});

		for line in lines {
			let mut words = line.split_whitespace();

			let mut first = None;
			if let Some(word) = words.next() {
				first = Some(word);
				match word {
					"{" => { push_table(&mut words, &mut stack); continue; }
					"[" => { push_array(&mut words, &mut stack); continue; }
					"}" | "]" => { close_array_or_table(&mut stack); continue; }
					_ => { println!("Unrecognized first item, control flow to stacker"); }
				}
			}

			// Handle members, array elems etc. This
			if let Some(top) = stack.last_mut() {
				match top.value {
					Value::Array(ref mut array) => { append_line_to_top_array(array,
					                                                          &first,
					                                                          &mut words); }
					Value::String(ref mut string) => {
						println!("The stack can not hold a string, internal error!");
					}
					Value::Table(ref mut table) => { append_entry_to_top_table(table,
					                                                           &first,
					                                                           &mut words); }
				}
			} else {
				println!("Somehow there's no last_mut on {}", line!());
			}
		}

		return {
			if let Some(node) = stack.pop() {
				match node.value {
					Value::Array(array) => Err(AconError::Error),
					Value::String(string) => Err(AconError::Error),
					Value::Table(table) => Ok(Acon { table: table }),
				}
			} else {
				println!("Somehow there's no last_mut on {}", line!());
				Err(AconError::Error)
			}
		};


		// BEGIN HELPER STRUCTURE ////////////////////////////////////////////
		use std::str::{Lines, SplitWhitespace};
		struct Node {
			name: String,
			value: Value,
		}
		// END HELPER STRUCTURE //////////////////////////////////////////////

		// BEGIN HELPER FUNCTIONS ////////////////////////////////////////////
		fn push_array(words: &mut SplitWhitespace, stack: &mut Vec<Node>) {
			let name = words.next().unwrap_or("");
			stack.push(Node {
				name: name.to_string(),
				value: Value::Array(Array::new()),
			});
		}

		fn push_table(words: &mut SplitWhitespace, stack: &mut Vec<Node>) {
			let name = words.next().unwrap_or("");
			stack.push(Node {
				name: name.to_string(),
				value: Value::Table(Table::new()),
			});
		}

		fn close_array_or_table(stack: &mut Vec<Node>) {
			if let Some(top) = stack.pop() {
				if let Some(node) = stack.last_mut() {
					match node.value {
						Value::Array(ref mut array) => {
							if top.name == "" {
								array.push(top.value);
							} else {
								let mut new = Table::new();
								new.insert(top.name, top.value);
								array.push(Value::Table(new));
							}
						}
						Value::String(ref mut string) => {}
						Value::Table(ref mut table) => {
							table.insert(top.name, top.value);
						}
					}
				} else {
					println!("{} found without anything on the stack!", "}");
				}
			} else {
				println!("{} found without anything on the stack!", "}");
			}
		}

		fn append_line_to_top_array(array: &mut Array,
		                            first: &Option<&str>,
		                            words: &mut SplitWhitespace) {
			let first = first.unwrap_or("");
			let acc = String::new();
			let acc = words.fold(first.to_string(), |acc, x| acc + " " + x);
			let acc = acc.trim();
			array.push(Value::String(acc.to_string()));
		}

		fn append_entry_to_top_table(table: &mut Table, first: &Option<&str>,
		                             words: &mut SplitWhitespace) {
			match first {
				&Some(ref key) => {
					let acc = String::new();
					let acc = words.fold("".to_string(), |acc, x| acc + " " + x);
					let acc = acc.trim();
					table.insert(key.to_string(), Value::String(acc.to_string()));
				}
				&None => {}
			}
		}
		// END HELPER FUNCTIONS //////////////////////////////////////////////

	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		use Acon;
		let value = r#"
		[ array
		]
		"#;
		let acon = value.parse::<Acon>().unwrap();
		println!("{:?}", acon.table);
	}
}
