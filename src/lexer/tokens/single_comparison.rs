use std::fmt::{Error, Display, Formatter};

pub enum SingleComparison {
    Greater,
    Lesser,
}

impl SingleComparison {
    pub fn to_char(&self) -> char {
        match self {
            &SingleComparison::Greater => '>',
            &SingleComparison::Lesser => '<',
        }
    }

    pub fn from_char(s: char) -> Result<Self, ()> {
        match s {
            '>' => Ok(SingleComparison::Greater),
            '<' => Ok(SingleComparison::Lesser),
            _ => Err(()),
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            &SingleComparison::Greater => ">",
            &SingleComparison::Lesser => "<",
        }
    }
}

impl Display for SingleComparison {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &SingleComparison::Greater => write!(f, ">"),
            &SingleComparison::Lesser => write!(f, "<"),
        }

    }
}
