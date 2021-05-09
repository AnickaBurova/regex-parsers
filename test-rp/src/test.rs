
// use crate::regex_parsers;
use regex_procs::RegexParser;
use regex_parsers::*;
use regex::Regex;
use std::sync::Arc;
use std::rc::Rc;
macro_rules! convert {
    ($value: expr => $txt: literal => $rgx: literal : $ty: ty ) => {
         let rgx = Regex::new($rgx).unwrap();
         let caps = rgx.captures($txt).unwrap();
         let value: $ty = Cap::new(caps.get(1)).convert();
         println!("value = {:?}: {}", value, stringify!($ty));
         assert_eq!(value, $value);
    }
}
macro_rules! numbers {
    ($($ty: ty),+) => {
        $(convert!{ 33 => "33" => r"(\d+)": $ty})+
        $(convert!{ Some(33) => "33" => r"(\d+)?": Option<$ty>})+
        $(convert!{ None => "" => r"(\d+)?": Option<$ty>})+
    }
}
macro_rules! neg_numbers {
    ($($ty: ty),+) => {
        $(convert!{ -33 => "-33" => r"(-?\d+)": $ty})+
    }
}

#[test]
fn converts() {
    convert!{ "Jozko" => "Jozko" => r"(\w+)": String}
    convert!{ Some("Jozko".into()) => "Jozko" => r"(\w+)?": Option<String>}
    convert!{ None => "" => r"(\w+)?": Option<String>}
    convert!{ Arc::new("Jozko".into()) => "Jozko" => r"(\w+)": Arc<String>}
    numbers!{  i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, usize, isize }
    neg_numbers!{  i8, i16, i32, i64, i128, isize }
    convert!{ 3.14 => "3.14" => r"([+-]?\w+(?:\.\d+)?)": f32}
}


#[derive(Debug, RegexParser, PartialEq)]
enum Special {
    #[rgx = r"One\(\s*(\w+),\s+(\d+)\)"]
    One(String, u16),
    #[rgx = r"Two\((\w+)?,\s+(\d+)?\)"]
    Two(Option<String>, Option<u8>),
    #[rgx = r"Three\((?P<name>\w+)?,\s+(?P<age>\d+)?\)"]
    Three {
        name: Option<String>,
        age: Option<Arc<u32>>,
    },
}

#[test]
fn regex_parser() {
    let spec = Special::parse_regex("One(Jozo, 14)");
    println!("{:?}", spec);
    assert_eq!(Some(Special::One("Jozo".into(), 14)), spec);
    let spec = Special::parse_regex("Two(Jozo, )");
    println!("{:?}", spec);
    let spec = Special::parse_regex("Two(, )");
    println!("{:?}", spec);
    let spec = Special::parse_regex("Three(, 30)");
    println!("{:?}", spec);
    let spec = Special::parse_regex("Three(Fero, 30)");
    println!("{:?}", spec);
}
#[derive(Debug, RegexParser, PartialEq)]
enum Animal {
    #[rgx = r"^Wolf\s+(?P<name>[A-Z][a-z]+)(\s+(?P<enemy>.+))?"]
    Wolf {
        name: String,
        enemy: Option<Rc<Animal>>,
    },
    #[rgx = r"^Sheep\s+(?P<name>[A-Z][a-z]+)\s+(?P<colour>\d+)(\s+(?P<enemy>.+))?"]
    Sheep {
        name: String,
        colour: u8,
        enemy: Option<Rc<Animal>>
    },
    #[rgx = r"^(?P<name>[A-Z][a-z]+)(\s+(?P<likes>.+))?"]
    Human {
        name: String,
        likes: Option<Arc<Animal>>,
    }
}

#[test]
fn recursive() {
    let animals = Animal::parse_regex("Fero Sheep Mara 3 Wolf Martin Anca");
    println!("ANIMALS = {:?}", animals);
    assert_eq!(animals.unwrap(), Animal::Human{
        name: "Fero".into(),
        likes: Some(Arc::new(
            Animal::Sheep {
                name: "Mara".into(),
                colour: 3,
                enemy: Some(Rc::new(
                    Animal::Wolf {
                        name: "Martin".into(),
                        enemy: Some(Rc::new(
                            Animal::Human {
                                name: "Anca".into(),
                                likes: None,
                            }
                        )),
                    }
                )),
            }
        )),
    })
}

#[derive(Debug, RegexParser, PartialEq)]
#[rgx = r"^(?P<name>[A-Z][a-z]+)\s+(?P<surname>[A-Z][a-z]+)$"]
struct Person<'t> {
    name: Arc<String>,
    surname: &'t str,
}

#[test]
fn structs_named_one() {
    let p = Person::parse_regex("Franta  Bujak").unwrap();
    assert_eq!(p, Person {
        name: Arc::new("Franta".into()),
        surname: "Bujak".into(),
    });
}


#[derive(Debug, RegexParser)]
#[rgx = r"^(?P<name>[A-Z][a-z]+)\s+(?P<surname>[A-Z][a-z]+)$"]
#[rgx = r"^(?P<age>\d+)\s+(?P<weight>\d+)$"]
#[rgx = r"^(?P<id>.+)$"]
struct Employee {
    name: std::sync::Arc<String>,
    surname: String,
    age: u16,
    weight: u8,
    id: String,
}

#[test]
fn struct_named_mult() {
    let e = Employee::parse_regex("Anca Polepetka").unwrap();
    println!("{:?}", e);
    let e = e.parse_regex_chain("33 80").left().unwrap();
    println!("{:?}", e);
    let mut e = e.parse_regex_chain("JA38s92839").left().unwrap();
    println!("{:?}", e);
    let app = Employee::parse_apply("JA888888").unwrap();
    app.apply(&mut e);
    println!("{:?}", e);
    let app = Employee::parse_apply("Franto Dobrak").unwrap();
    app.apply(&mut e);
    println!("{:?}", e);
}

// #[derive(Debug, RegexParser, PartialEq)]
// #[rgx = r"Args\s+=\s+(\w+)\s*\|\s*(?:(\d+)\s*\|)?\s*(\w+)\s*\|\s*([+-]?\d+(?:\.\d+)?)"]
// struct Arguments(String, Option<Arc<usize>>, Arc<String>, Arc<f32>);
//
// #[test]
// fn struct_unamed_one() {
//     let args = Arguments::parse_regex("Args = Hello | World | -3.14").unwrap();
//     println!("args = {:?}", args);
//     assert_eq!(args, Arguments("Hello".into(), None, Arc::new("World".into()), Arc::new(-3.14)));
// }
