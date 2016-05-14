use std::collections::BTreeMap;
use std::str::FromStr;

#[derive(PartialEq, Clone, Debug)]
pub enum Value {
	Array(Array),
	String(String),
	Table(Table),
}

impl Value {
	fn array(&self) -> &Array {
		match *self {
			Value::Array(ref array) => array,
			_ => panic!("Value is not an array"),
		}
	}
	fn string(&self) -> &String {
		match *self {
			Value::String(ref string) => string,
			_ => panic!("Value is not a string"),
		}
	}
	fn table(&self) -> &Table {
		match *self {
			Value::Table(ref table) => table,
			_ => panic!("Value is not a table"),
		}
	}
}

pub type Array = Vec<Value>;
pub type Table = BTreeMap<String, Value>;

#[derive(PartialEq, Clone, Debug)]
struct Acon {
	table: Table,
}

#[derive(PartialEq, Clone, Debug)]
enum AconError {
	ExcessiveClosingDelimiter(Option<usize>),
	InternalStringTop(Option<usize>),
	MissingStackTop(Option<usize>),
	TopNodeIsArray,
	OverwritingKey(Option<usize>),
	WrongClosingDelimiterExpectedArray(Option<usize>),
	WrongClosingDelimiterExpectedTable(Option<usize>),
}

#[allow(dead_code)]
impl AconError {
	fn reason(&self) -> String {
		use AconError::*;
		match *self {
			ExcessiveClosingDelimiter(line) => {
				let first = match line { Some(line) => format!("On line {}, t", line), None => "T".to_string() };
				format!("{}here's a closing delimiter that has no matching opening delimiter. Note that
all delimiters must be the first word on a line to count as such. The only delimiters are {}, {}, [, ], and $.",
				first, "{", "}")
			}
			InternalStringTop(line) => {
				let first = match line { Some(line) => format!("On line {}, t", line), None => "T".to_string() };
				format!("{}here's a string on the top of the internal parse stack. This is impossible unless there is a
bug in the parser. Please report this along with the input to the repository maintainer of ACON.", first)
			}
			MissingStackTop(line) => {
				let first = match line { Some(line) => format!("On line {}, t", line), None => "T".to_string() };
				format!("{}he top of the stack is missing. This indicates an internal error, as it's never supposed to
happen. Please contact the maintainer of the ACON repository.", first)
			}
			TopNodeIsArray => {
				format!("The top of the stack is an array. This indicates that there is an unterminated array all the way
until the end of the input. Try appending a ']' to the input to see if this solves the issue.")
			}
			OverwritingKey(line) => {
				let first = match line { Some(line) => format!("On line {}, t", line), None => "T".to_string() };
				format!("{}he key is already present in the table.", first)
			}
			WrongClosingDelimiterExpectedArray(line) => {
				let first = match line { Some(line) => format!("On line {}, t", line), None => "T".to_string() };
				format!("{}he closing delimiter did not match the array closing delimiter ]. Make sure all delimiters
match up in the input. Some editors can help you by jumping from/to each delimiter.", first)
			}
			WrongClosingDelimiterExpectedTable(line) => {
				let first = match line { Some(line) => format!("On line {}, t", line), None => "T".to_string() };
				format!("{}he closing delimiter did not match the table closing delimiter {}. Make sure all delimiters
until the end of the input. Try appending a ']' to the input to see if this solves the issue.", first, "}")
			}
		}
	}
}

impl FromStr for Acon {
	type Err = AconError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut stack = vec![];
		let lines = s.lines();
		let mut current_line = 0usize;
		push_base_table(&mut stack);

		for line in lines {
			current_line += 1;

			let mut words = line.split_whitespace();

			let mut first = None;
			if let Some(word) = words.next() {
				first = Some(word);
				match word {
					"{" => { push_table(&mut words, &mut stack); continue; }
					"[" => { push_array(&mut words, &mut stack); continue; }
					word @ "}" | word @ "]" => { try!(close_array_or_table(word, &mut stack, current_line)); continue; }
					_ => { }
				}
			}

			if let Some(top) = stack.last_mut() {
				match top.value {
					Value::Array(ref mut array)
						=> { append_line_to_top_array(array, &first, &mut words); }
					Value::String(_)
						=> return Err(AconError::InternalStringTop(Some(current_line))),
					Value::Table(ref mut table)
						=> { try!(append_entry_to_top_table(table, &first, &mut words, current_line)); }
				}
			} else {
				return Err(AconError::MissingStackTop(Some(current_line)));
			}
		}

		return {
			if let Some(node) = stack.pop() {
				match node.value {
					Value::Array(_) => Err(AconError::TopNodeIsArray),
					Value::String(_) => Err(AconError::InternalStringTop(Some(current_line))),
					Value::Table(table) => Ok(Acon { table: table }),
				}
			} else {
				Err(AconError::MissingStackTop(None))
			}
		};


		// BEGIN HELPER STRUCTURE ////////////////////////////////////////////
		use std::str::SplitWhitespace;
		struct Node {
			name: String,
			value: Value,
		}
		// END HELPER STRUCTURE //////////////////////////////////////////////

		// BEGIN HELPER FUNCTIONS ////////////////////////////////////////////
		fn push_base_table(stack: &mut Vec<Node>) {
			stack.push(Node {
				name: "".to_string(),
				value: Value::Table(Table::new()),
			});
		}

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

		fn close_array_or_table(word: &str, stack: &mut Vec<Node>, line: usize) -> Result<(), AconError> {
			if let Some(top) = stack.pop() {
				match top.value {
					Value::Array(_) if word != "]"
						=> return Err(AconError::WrongClosingDelimiterExpectedArray(Some(line))),
					Value::String(_) if word != "]"
						=> return Err(AconError::InternalStringTop(Some(line))),
					Value::Table(_) if word != "}"
						=> return Err(AconError::WrongClosingDelimiterExpectedTable(Some(line))),
					_ => {}
				}
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
						Value::String(_) => { return Err(AconError::InternalStringTop(Some(line))); }
						Value::Table(ref mut table) => {
							if table.contains_key(&top.name) {
								return Err(AconError::OverwritingKey(Some(line)));
							}
							table.insert(top.name, top.value);
						}
					}
					Ok(())
				} else {
					Err(AconError::ExcessiveClosingDelimiter(Some(line)))
				}
			} else {
				Err(AconError::MissingStackTop(Some(line)))
			}
		}

		fn append_line_to_top_array(array: &mut Array,
		                            first: &Option<&str>,
		                            words: &mut SplitWhitespace) {
			let first = first.unwrap_or("");
			let acc = words.fold(first.to_string(), |acc, x| acc + " " + x);
			let acc = acc.trim();
			array.push(Value::String(acc.to_string()));
		}

		fn append_entry_to_top_table(table: &mut Table,
		                             first: &Option<&str>,
		                             words: &mut SplitWhitespace,
		                             line: usize) -> Result<(), AconError> {
			match first {
				&Some(ref key) => {
					if table.contains_key(&key.to_string()) {
						return Err(AconError::OverwritingKey(Some(line)));
					}
					let acc = words.fold("".to_string(), |acc, x| acc + " " + x);
					let acc = acc.trim();
					table.insert(key.to_string(), Value::String(acc.to_string()));
				}
				&None => {}
			}
			Ok(())
		}
		// END HELPER FUNCTIONS //////////////////////////////////////////////

	}
}

#[cfg(test)]
mod tests {
	use {Acon, AconError};

	#[test]
	fn neg_duplicate_keys() {
		let value = r#"
			key value1
			key2 value2
			key value3
			key2 value4
		"#;
		let acon = value.parse::<Acon>();
		assert_eq!(acon, Err(AconError::OverwritingKey(Some(4))));
	}

	#[test]
	fn neg_duplicate_keys_table() {
		let value = r#"
			key value1
			key2 value2
			{ key
			}
			key2 value4
		"#;
		let acon = value.parse::<Acon>();
		assert_eq!(acon, Err(AconError::OverwritingKey(Some(5))));
	}

	#[test]
	fn neg_duplicate_keys_array() {
		let value = r#"
			key value1
			key2 value2
			[ key
			]
			key2 value4
		"#;
		let acon = value.parse::<Acon>();
		assert_eq!(acon, Err(AconError::OverwritingKey(Some(5))));
	}

	#[test]
	fn neg_duplicate_keys_nested() {
		let value = r#"
			{ key
				{ key
					key value
					[
					]
					key value
				}
			}
		"#;
		let acon = value.parse::<Acon>();
		assert_eq!(acon, Err(AconError::OverwritingKey(Some(7))));
	}


	#[test]
	fn inspect_message() {
		let value = r#"
			[
				{ message
					recipient me
					sender you
					[ content
						Hey what is this ACON thingy all about?
						I mean, we've got TOML, JSON, XML, and SGML.
						Why do we need this data serilization language?
					]
				}
				{ message
					sender me
					recipient you
					[ content
						ACON means Awk-Compatible Object Notation.
						TOML, JSON, etc are great serialization languages, but they're quite complex.
						We need tools and languages that are easily
						parsable and friendly for bash scripting.
						ACON allows just that!
					]
				}
			]
		"#;
		let acon = value.parse::<Acon>();
		assert_eq!(acon.unwrap().table.get("").unwrap().array().get(1).unwrap().table().get("message").unwrap().table().get("recipient").unwrap().string(), "you");
	}

}
