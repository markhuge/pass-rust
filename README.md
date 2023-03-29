# pass

## Pass
`pass` contains primitives for decoding pass <https://passwordstore.org>
entries into structured data.

`pass` password entries utilize an informal schema. By convention, many
consumers of `pass` data use the `url` and `login` directives.

## Examples

### `Entry::from_utf8` - Decode a password store entry from utf8 input.

 This is handy for piping the return from stdout.

 ```rust
 use std::process::{Command, Stdio};

 let name = "myEmail";
 let output = Command::new("pass")
		.arg(&name)
		.stdout(Stdio::piped())
		.output()
		.expect("command failed");

 let entry = pass::Entry::from_utf8(name, &output.stdout).unwrap();
 ```

License: LGPL-2.1
