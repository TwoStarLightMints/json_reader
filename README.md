# json_reader

This is a personal implementation of a JSON parser. There are many other crates that can be used for parsing JSON, but I wanted to strengthen my skills in rust.

Any feedback would be welcome!

# Use

main.rs
```
use json_reader::json_reader::*;

fn main() {
    let json_string = String::from(r#"{ "key1": "Hello, World!", "key2": [123, true, ["I am nested!"]], "key3": { "nested-obj": "Hello up there!"} }"#);

    println!("{}", &json_string);
    println!("{:?}", tokenize_json_string(&json_string));

    println!("{:?}", from_json_string(&json_string))
}
```
Output:
```
Compiling your_program_here v0.1.0 (C:\path\to\your_program_here)
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
     Running `target\debug\your_program_here.exe`
{ "key1": "Hello, World!", "key2": [123, true, ["I am nested!"]], "key3": { "nested-obj": "Hello up there!"} }[JsonObjBeg, JsonKey("key1"), JsonString("Hello, World!"), JsonKey("key2"), JsonArrBeg, JsonNum(123), JsonBool(true), JsonArrBeg, JsonString("I am nested!"), JsonArrEnd, JsonArrEnd, JsonKey("key3"), JsonObjBeg, JsonKey("nested-obj"), JsonString("Hello up there!"), JsonObjEnd, JsonObjEnd]
{"key2": JsonArr([JsonNum(123), JsonBool(true), JsonArr([JsonString("I am nested!")])]), "key3": JsonObj({"nested-obj": JsonString("Hello up there!")}), "key1": JsonString("Hello, World!")}
```

# Notes
There is what I would consider a redundant module declaration. I put this here because I was unable to import the crate into another rust project without getting a weird errror and adding the declaration somehow fixed it. If anyone knows how to fix this please let me know.
