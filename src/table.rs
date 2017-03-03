
use nom::alphanumeric;
use nom::multispace;
use nom::IResult;

use super::key::*;

#[derive(PartialEq,Debug,Default)]
pub struct Table<'a> {
    name : Key<'a>,
    alias :Option<Key<'a>>
}

named!(table_alias<&[u8],Option<Key> >,
       opt!(
           complete!(
               do_parse!(
                   tag_no_case!("AS") >>
                       multispace >>
                       alias :  key >>
                       (alias)
               )
           )
       )
);

named!(table<&[u8],Table>,
       do_parse!(
           name : key >>
           many0!(multispace) >>
           alias : table_alias >>
           (
               Table{name:name,alias:alias}
           )
       )
);

#[test]
fn test_simple_table_name() {
    assert_eq!(table(b"table"),IResult::Done(&b""[..],
                                             Table{
                                                 name:Key::new(&b"table"[..],false),
                                                 alias:None,
                                             })
    );

    assert_eq!(table(b"`table`"),IResult::Done(&b""[..],
                                             Table{
                                                 name:Key::new(&b"table"[..],true),
                                                 alias:None,
                                             })
    );

    assert_eq!(table(b"table as t"),IResult::Done(&b""[..],
                                             Table{
                                                 name:Key::new(&b"table"[..],false),
                                                 alias:Some(Key::new(&b"t"[..],false)),
                                             })
    );

    assert_eq!(table(b"`table` AS t"),IResult::Done(&b""[..],
                                             Table{
                                                 name:Key::new(&b"table"[..],true),
                                                 alias:Some(Key::new(&b"t"[..],false)),
                                             })
    );

    assert_eq!(table(b"`table` AS `t`"),IResult::Done(&b""[..],
                                                    Table{
                                                        name:Key::new(&b"table"[..],true),
                                                        alias:Some(Key::new(&b"t"[..],true)),
                                                    })
    );
}
