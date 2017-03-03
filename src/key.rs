use nom::alphanumeric;
use nom::multispace;
use super::common::*;
use nom::is_space;
#[derive(PartialEq,Debug,Default)]
pub struct Key<'a> {

    val : &'a[u8],
    allow_reserverd : bool

}


impl<'a> Key<'a> {
    pub fn new(val : &'a[u8],allow_reserverd:bool) -> Self {
        Key{val:val,allow_reserverd:allow_reserverd}
    }

    pub fn val(&self) -> &'a [u8] {
        self.val
    }
    pub fn is_allow_reserverd(&self) -> bool {
        self.allow_reserverd
    }
}

named!(pub key<&[u8],Key>,
       alt!(
           map!(
               delimited!(
                   tag!("`"),
                   take_while!(not_quote),
                   tag!("`")
               ),
               |t| {
                   Key::new(t,true)
               }
           )
               |
           map!(
               take_while!(not_keyword_symbol),
               |t| {
                   Key::new(t,false)
               }
           )
       )
);
