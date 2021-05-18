use regex::Match;
use std::sync::Arc;
use std::rc::Rc;
use either::Either;

/// Convert string in the match in to the object.
/// This should not fail, because the capture should match only if the object is convertible.
pub trait FromMatch<'t> {
    fn from_match(m: Option<Match<'t>>) -> Self;
}

pub struct Cap<'t>(Option<Match<'t>>);

impl<'t> Cap<'t> {
    pub fn new(m: Option<Match<'t>>) -> Self {
        Self(m)
    }
    pub fn convert<T: FromMatch<'t>>(self) -> T {
        T::from_match(self.0)
    }
}

pub trait RegexParser<'t>: Sized {
    fn parse_regex(txt: &'t str) -> Option<Self>;
}

pub trait RegexParserApply<'t>: Sized {
    type Apply;
    fn parse_apply(txt: &'t str) -> Option<Self::Apply>;
}

pub trait RegexParserChain<'t>: Sized {
    type Chain;
    fn create_chain() -> Self::Chain;
}

pub trait RegexParserChained<'t>: Sized {
    /// The final object this creates.
    type Chained;
    /// Parse a string. If string has been parsed, the first tuple item is true, otherwise false.
    /// The second tuple item is the next chain of the parsing. If the text has not been parsed, the same
    /// `self` is returned in the Left.
    /// If the `txt` has been parsed, the next in the chain can be the final result (Right), or just
    /// Self but the next in the chain of parsing.
    fn parse_chain(self, txt: &'t str) -> (bool, Either<Self, Self::Chained>);
}

impl<'t> FromMatch<'t> for String {
    fn from_match(m: Option<Match<'t>>) -> Self {
        m.unwrap().as_str().to_owned()
    }
}
impl<'t, T: FromMatch<'t>> FromMatch<'t> for Option<T> {
    fn from_match(m: Option<Match<'t>>) -> Self {
        if m.is_some() {
            Some(Cap::new(m).convert())
        } else {
            None
        }
    }
}
impl<'t, T: FromMatch<'t>> FromMatch<'t> for Arc<T> {
    fn from_match(m: Option<Match<'t>>) -> Self {
        Arc::new(Cap::new(m).convert())
    }
}

impl<'t, T: FromMatch<'t>> FromMatch<'t> for Rc<T> {
    fn from_match(m: Option<Match<'t>>) -> Self {
        Rc::new(Cap::new(m).convert())
    }
}

impl<'t, T: FromMatch<'t>> FromMatch<'t> for Box<T> {
    fn from_match(m: Option<Match<'t>>) -> Self {
        Box::new(Cap::new(m).convert())
    }
}
macro_rules! numbers {
    ($($ty: ty),+) => {
        $(
            impl<'t> FromMatch<'t> for $ty {
                fn from_match(m: Option<Match<'t>>) -> Self {
                    // println!("{:?}", m);
                    m.unwrap().as_str().parse().unwrap()
                }
            }
        )+
    }
}

numbers!{ i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, usize, isize, f32, f64}

impl<'t> FromMatch<'t> for &'t str {
    fn from_match(m: Option<Match<'t>>) -> Self {
        m.unwrap().as_str()
    }
}

impl<'t> FromMatch<'t> for bool {
    fn from_match(m: Option<Match<'t>>) -> Self {
        m.is_some()
    }
}

impl<'t> FromMatch<'t> for char {
    fn from_match(m: Option<Match<'t>>) -> Self {
        m.unwrap().as_str().chars().nth(0).unwrap()
    }
}
