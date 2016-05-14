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
	let acon = value.parse::<Acon>().unwrap();
	assert_eq!(acon.get("").unwrap().array().get(1).unwrap().table()
	           .get("message").unwrap().table().get("recipient").unwrap().string(), "you");
	assert_eq!(acon.path(".1.message.recipient"), Some(&Acon::String("you".to_string())));
}
