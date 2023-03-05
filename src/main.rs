use std::collections::HashMap;

fn main(){

  let mut v = Vec::new();
  v.push(1);
  v.push(2);
  v.push(3);

  let v2 = vec![1, 2, 3];
  let first: &i32 = &v2[0];
  println!("first: {:?}", first);

  //pattern match

  let second: Option<&i32> = v2.get(1);
  match second {
    Some(second) => println!("second: {:?}", second),
    None => println!("there is no second element"),
  }

//Iterating over the Values in a Vector

  let vec = vec![1, 2, 3];
  for i in &vec {
    println!("{}", i);
  }

  let mut vec2 = vec![17, 2, 33];
  for i in &mut vec2 {
    *i += 10;
    println!("{}", i);
  }

  //Using an Enum to Store Multiple Types
  enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }

  let _row = vec![
    SpreadsheetCell::Int(2),
    SpreadsheetCell::Float(9.99),
    SpreadsheetCell::Text(String::from("lorem ipsum dolor")),
  ];

  // Updating a String
  let mut str = String::new(); //empty string
  str.push_str("hello");
  println!("{}", str);

  let value = 10;
  let value_to_string = value.to_string();
  println!("{}", value_to_string);

  let mut another_str = String::from("hello");
  another_str.make_ascii_uppercase();
  println!("{}", another_str);

  let mut laugh = String::from("lo");
  laugh.push('l');
  println!("{}", laugh);

  let hello1 = String::from("السلام عليكم");
  println!("{}", hello1);
  let hello2 = String::from("Dobrý den");
  println!("{}", hello2);
  let hello3 = String::from("Hello");
  println!("{}", hello3);
  let hello4 = String::from("שָׁלוֹם");
  println!("{}", hello4);
  let hello5 = String::from("नमस्ते");
  println!("{}", hello5);
  let hello6 = String::from("こんにちは");
  println!("{}", hello6);
  let hello7 = String::from("안녕하세요");
  println!("{}", hello7);
  let hello8 = String::from("你好");
  println!("{}", hello8);
  let hello9 = String::from("Olá");
  println!("{}", hello9);
  let hello10 = String::from("Здравствуйте");
  println!("{}", hello10);
  let hello11 = String::from("Hola");
  println!("{}", hello11);

  println!();

  // Concatenation with the + Operator or the format! Macro
  let s1 = String::from("Hola, "); //note s1 has been moved here and can no longer be used
  let s2 = String::from("Muchacho!");
  let s3 = s1 + &s2;
  println!("{}", s3);

  println!();

  let a1 = String::from("tic");
  let a2 = String::from("tac");
  let a3 = String::from("toe");

  let a = a1 + "-" + &a2 + "-" + &a3;
  println!("{}", a);

  println!();

  let b1 = String::from("tic");
  let b2 = String::from("tac");
  let b3 = String::from("toe");

  let b = format!("{b1}-{b2}-{b3}");
  println!("{}", b);

  println!();

  let hola = String::from("Hola");
  for c in hola.chars() {
    println!("{}", c);
  };

  //Storing Keys with Associated Values in Hash Maps
  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  let team_name = String::from("Blue");
  let score = scores.get(&team_name).copied().unwrap_or(0);
  println!("{}", score);
  println!("{:?}", scores);

  scores.entry(String::from("Yellow")).or_insert(50);
  scores.entry(String::from("Blue")).or_insert(50);

  scores.insert(String::from("Red"), 1);
  scores.entry(String::from("Red")).or_insert(7);

  println!("{:?}", scores);

  println!();

  for (key, value) in &scores {
    println!("{key}:{value}");
  }

  println!();
  let field_name = String::from("favorite colour");
  let field_value = String::from("Green");

  let mut map = HashMap::new();
  map.insert(field_name, field_value);
  for (key, value) in &map {
    println!("{key}:{value}");
  }

  let text = "lorem ipsum colour";
  let mut mapper = HashMap::new();
  for word in text.split_whitespace() {
    let count = mapper.entry(word).or_insert(0);
    *count += 1;
  }

  println!("{:?}", mapper);



}