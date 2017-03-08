pub mod string;
pub mod number;
pub mod date;
pub mod boolean;
pub mod null;

pub enum Literal<'a> {
    String(string::String<'a>),
    DateTime(date::DateTime<'a>),
    Number( number::Number<'a>),
    Boolean(boolean::Boolean),
    NULL(NULL)
}

impl Literal {

    pub fn is_string(&self) -> bool{
        match self {
            String(_) => true,
            _ => false
        }
    }
    pub fn is_datetime(&self) -> bool{
        match self {
            DateTime(_) => true,
            _ => false
        }
    }
    pub fn is_number(&self) -> bool{
        match self {
            Number(_) => true,
            _ => false
        }
    }
    pub fn is_boolean(&self) -> bool{
        match self {
            Boolean(_) => true,
            _ => false
        }
    }
    pub fn is_null(&self) -> bool{
        match self {
            NULL(_) => true,
            _ => false
        }
    }

    pub fn is_binary() -> bool {

        match self {
            String(s) => {
                match s {
                    string::StringType::SinleQuoteBinary(_) | string::StringType::Binary => {true},
                    _ => false
                }
            },
            _ => false
        }

    }

    pub fn is_hex() -> bool {
        match self {
            String(s) => {
                match s {
                    string::StringType::SingleQuooteHex(_) | string::StringType::Hex => {true},
                    _ => false
                }
            },
            _ => false
        }
    }

    pub fn unwrap(&self) -> Option<&'a[u8]> {}
}
