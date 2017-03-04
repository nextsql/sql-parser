use nom::multispace;
use nom::alphanumeric;

use super::key::*;

#[derive(PartialEq,Debug,Default)]
pub struct AlertSchema<'a> {
    name : Key<'a>,
    charset:Option<&'a[u8]>,
    collation:Option<&'a[u8]>,
}

named!(alert_schema<&[u8],AlertSchema>,
       do_parse!(
           tag_no_case!(b"ALERT") >>
           multispace >>
           alt!(tag_no_case!(b"DATABASE") | tag_no_case!("SCHEMA")) >>
           multispace >>
           name:key >>
           multispace >>
           charset:opt!(
                   complete!(
                       do_parse!(
                           opt!(tag_no_case!("DEFAULT")) >>
                           multispace >>
                           tag_no_case!("CHARACTER SET") >>
                           many0!(multispace) >>
                           opt!(tag_no_case!("=")) >>
                           many0!(multispace) >>
                           charset: alphanumeric >>
                           (charset)
                       )
                   )
           ) >>
               collation: opt!(
                   complete!(
                       do_parse!(
                           opt!(tag_no_case!("DEFAULT")) >>
                           multispace >>
                           tag_no_case!("COLLATE") >>
                           many0!(multispace) >>
                           opt!(tag_no_case!("=")) >>
                           many0!(multispace) >>
                           collation: alphanumeric >>
                           (collation)
                       )
                   )
            ) >>

               (AlertSchema{
                   name:name,
                   charset:charset,
                   collation:collation
               })
       )
);

#[test]
fn test_alert_schema() {

    use nom::IResult;

    {
        let sql = b"alert database test";
        assert_eq!(alert_schema(sql),IResult::Done(&b""[..],
                                                    AlertSchema{
                                                        name : Key::new(b"test",false),
                                                        charset:None,
                                                        collation:None,
                                                    }
        ));
    }

    {
        let sql = b"alert database `test`";
        assert_eq!(alert_schema(sql),IResult::Done(&b""[..],
                                                    AlertSchema{
                                                        name : Key::new(b"test",true),
                                                        charset:None,
                                                        collation:None,
                                                    }
        ));
    }

   

    {
        let sql = b"alert database if not exists test CHARACTER SET = utf8 COLLATE = none";
        assert_eq!(alert_schema(sql),IResult::Done(&b""[..],
                                                    AlertSchema{
                                                        name : Key::new(&b"test"[..],false),
                                                        charset:Some(&b"utf8"[..]),
                                                        collation:Some(&b"none"[..]),
                                                    }
        ));
    }

}
