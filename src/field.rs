use nom::alphanumeric;
use nom::multispace;

use super::key::key;
use super::key::Key;
use super::common::*;

#[derive(PartialEq,Debug,Default)]
pub struct Field<'a> {
    name : Key<'a>,
    func : Option<Key<'a>>
}

named!(field<&[u8],Field>,
       alt!(
           complete!(
           do_parse!(
               func:alphanumeric >>
               many0!(multispace) >>
               tag!("(") >>
               many0!(multispace) >>
               field:key >>
               many0!(multispace) >>
               tag!(")") >>
                   (
                  {
                   Field{name:field,func:Some(Key::new(func,false))}
                  })
             )
           ) |
           map!(
               key,
               |field| {
                   Field{name:field,func:None}
               }
           )
       )
);


#[test]
fn test_field() {

    use nom::IResult;
    assert_eq!(field(b"field"),IResult::Done(&b""[..],
                                             Field{
                                                          name:Key::new(&b"field"[..],false),
                                                          func:None,
                                                      })
    );

    assert_eq!(field(b"`field`"),IResult::Done(&b""[..],
                                             Field{
                                                 name:Key::new(&b"field"[..],true),
                                                 func:None,
                                             })
    );
  
    assert_eq!(field(b"`count(field)`"),IResult::Done(&b""[..],
                                               Field{
                                                   name:Key::new(&b"count(field)"[..],true),
                                                   func:None,
                                               })
    );
     
    
    assert_eq!(field(b"count(field)"),IResult::Done(&b""[..],
                                                      Field{
                                                          name:Key::new(&b"field"[..],false),
                                                          func:Some(Key::new(&b"count"[..],false)),
                                                      })
    );
    
    assert_eq!(field(b"count(`field`)"),IResult::Done(&b""[..],
                                                    Field{
                                                        name:Key::new(&b"field"[..],true),
                                                        func:Some(Key::new(&b"count"[..],false)),
                                                    })
    );
}

