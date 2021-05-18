use regex::Match;
use std::sync::Arc;
use std::rc::Rc;
use either::Either;

/// Convert string in the match in to the object.
/// This should not fail, because the capture should match only if the object is convertible.
pub trait FromMatch {
    fn from_match<'t>(m: Option<Match<'t>>) -> Self;
}

/// Captured match
pub struct Cap<'t>(Option<Match<'t>>);

impl<'t> Cap<'t> {
    pub fn new(m: Option<Match<'t>>) -> Self {
        Self(m)
    }
    /// Convert the captured match in to an object
    pub fn convert<T: FromMatch>(self) -> T {
        T::from_match(self.0)
    }
}

/// Parse a partial structure from an input text.
/// This partial structure can be applied in to the final structure.
pub trait RegexParserApply: Sized {
    /// Type of partial structure.
    /// This object can be then applied to the final object
    type Apply;
    fn parse_apply<'t>(txt: &'t str) -> Option<Self::Apply>;
}

/// Parse an input text in to an object
pub trait RegexParser: Sized {
    fn parse_regex<'t>(txt: &'t str) -> Option<Self>;
}

/// Parse a partial structure from an input text
pub trait RegexParserChain: Sized {
    /// Type of the partial structure
    type Chain;
    fn create_chain() -> Self::Chain;
}

pub trait RegexParserChained: Sized {
    /// The final object this creates.
    type Chained;
    /// Parse a string. If string has been parsed, the first tuple item is true, otherwise false.
    /// The second tuple item is the next chain of the parsing. If the text has not been parsed, the same
    /// `self` is returned in the Left.
    /// If the `txt` has been parsed, the next in the chain can be the final result (Right), or just
    /// Self but the next in the chain of parsing.
    fn parse_chain<'t>(self, txt: &'t str) -> (bool, Either<Self, Self::Chained>);
}

impl FromMatch for String {
    fn from_match<'t>(m: Option<Match<'t>>) -> Self {
        m.unwrap().as_str().to_owned()
    }
}
impl<T: FromMatch> FromMatch for Option<T> {
    fn from_match<'t>(m: Option<Match<'t>>) -> Self {
        if m.is_some() {
            Some(Cap::new(m).convert())
        } else {
            None
        }
    }
}
impl<T: FromMatch> FromMatch for Arc<T> {
    fn from_match<'t>(m: Option<Match<'t>>) -> Self {
        Arc::new(Cap::new(m).convert())
    }
}

impl<T: FromMatch> FromMatch for Rc<T> {
    fn from_match<'t>(m: Option<Match<'t>>) -> Self {
        Rc::new(Cap::new(m).convert())
    }
}

impl<T: FromMatch> FromMatch for Box<T> {
    fn from_match<'t>(m: Option<Match<'t>>) -> Self {
        Box::new(Cap::new(m).convert())
    }
}
macro_rules! numbers {
    ($($ty: ty),+) => {
        $(
            impl FromMatch for $ty {
                fn from_match<'t>(m: Option<Match<'t>>) -> Self {
                    // println!("{:?}", m);
                    m.unwrap().as_str().parse().unwrap()
                }
            }
        )+
    }
}

numbers!{ i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, usize, isize, f32, f64}


impl FromMatch for bool {
    fn from_match<'t>(m: Option<Match<'t>>) -> Self {
        m.is_some()
    }
}

impl FromMatch for char {
    fn from_match<'t>(m: Option<Match<'t>>) -> Self {
        m.unwrap().as_str().chars().nth(0).unwrap()
    }
}