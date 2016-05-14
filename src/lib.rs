#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::str::FromStr;

pub type Array = Vec<Acon>;
pub type Table = BTreeMap<String, Acon>;

#[derive(PartialEq, Clone, Debug)]
pub enum Acon {
	Array(Array),
	String(String),
	Table(Table),
}

impl Acon {
	pub fn array(&self) -> &Array {
		match *self {
			Acon::Array(ref array) => array,
			_ => panic!("Value is not an array"),
		}
	}

	pub fn string(&self) -> &String {
		match *self {
			Acon::String(ref string) => string,
			_ => panic!("Value is not a string"),
		}
	}

	pub fn table(&self) -> &Table {
		match *self {
			Acon::Table(ref table) => table,
			_ => panic!("Value is not a table"),
		}
	}

	pub fn path(&self, path: &str) -> Option<&Acon> {
		let paths = path.split(".");
		let mut current = self;
		for path in paths {
			let owned = current;
			current = match owned.get(path) {
				Some(ref acon) => acon,
				None => return None,
			}
		}
		Some(current)
	}

	pub fn path_mut(&mut self, path: &str) -> Option<&mut Acon> {
		let paths = path.split(".");
		let mut current = self;
		for path in paths {
			let owned = current;
			current = match owned.get_mut(path) {
				Some(acon) => acon,
				None => return None,
			}
		}
		Some(current)
	}

	pub fn get(&self, path: &str) -> Option<&Acon> {
		match *self {
			Acon::Array(ref array) => {
				match path.parse::<usize>() {
					Ok(value) => array.get(value),
					_ => None,
				}
			}
			Acon::String(_) => None,
			Acon::Table(ref table) => table.get(path),
		}
	}

	pub fn get_mut(&mut self, path: &str) -> Option<&mut Acon> {
		match *self {
			Acon::Array(ref mut array) => {
				match path.parse::<usize>() {
					Ok(value) => array.get_mut(value),
					_ => None,
				}
			}
			Acon::String(_) => None,
			Acon::Table(ref mut table) => table.get_mut(path),
		}
	}
}

#[derive(PartialEq, Clone, Debug)]
pub enum AconError {
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
					Acon::Array(ref mut array)
						=> { append_line_to_top_array(array, &first, &mut words); }
					Acon::String(_)
						=> return Err(AconError::InternalStringTop(Some(current_line))),
					Acon::Table(ref mut table)
						=> { try!(append_entry_to_top_table(table, &first, &mut words, current_line)); }
				}
			} else {
				return Err(AconError::MissingStackTop(Some(current_line)));
			}
		}

		return {
			if let Some(node) = stack.pop() {
				match node.value {
					Acon::Array(_) => Err(AconError::TopNodeIsArray),
					Acon::String(_) => Err(AconError::InternalStringTop(Some(current_line))),
					Acon::Table(table) => Ok(Acon::Table(table)),
				}
			} else {
				Err(AconError::MissingStackTop(None))
			}
		};


		// BEGIN HELPER STRUCTURE ////////////////////////////////////////////
		use std::str::SplitWhitespace;
		struct Node {
			name: String,
			value: Acon,
		}
		// END HELPER STRUCTURE //////////////////////////////////////////////

		// BEGIN HELPER FUNCTIONS ////////////////////////////////////////////
		fn push_base_table(stack: &mut Vec<Node>) {
			stack.push(Node {
				name: "".to_string(),
				value: Acon::Table(Table::new()),
			});
		}

		fn push_array(words: &mut SplitWhitespace, stack: &mut Vec<Node>) {
			let name = words.next().unwrap_or("");
			stack.push(Node {
				name: name.to_string(),
				value: Acon::Array(Array::new()),
			});
		}

		fn push_table(words: &mut SplitWhitespace, stack: &mut Vec<Node>) {
			let name = words.next().unwrap_or("");
			stack.push(Node {
				name: name.to_string(),
				value: Acon::Table(Table::new()),
			});
		}

		fn close_array_or_table(word: &str, stack: &mut Vec<Node>, line: usize) -> Result<(), AconError> {
			if let Some(top) = stack.pop() {
				match top.value {
					Acon::Array(_) if word != "]"
						=> return Err(AconError::WrongClosingDelimiterExpectedArray(Some(line))),
					Acon::String(_) if word != "]"
						=> return Err(AconError::InternalStringTop(Some(line))),
					Acon::Table(_) if word != "}"
						=> return Err(AconError::WrongClosingDelimiterExpectedTable(Some(line))),
					_ => {}
				}
				if let Some(node) = stack.last_mut() {
					match node.value {
						Acon::Array(ref mut array) => {
							if top.name == "" {
								array.push(top.value);
							} else {
								let mut new = Table::new();
								new.insert(top.name, top.value);
								array.push(Acon::Table(new));
							}
						}
						Acon::String(_) => { return Err(AconError::InternalStringTop(Some(line))); }
						Acon::Table(ref mut table) => {
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
			array.push(Acon::String(acc.to_string()));
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
					table.insert(key.to_string(), Acon::String(acc.to_string()));
				}
				&None => {}
			}
			Ok(())
		}
		// END HELPER FUNCTIONS //////////////////////////////////////////////

	}
}
