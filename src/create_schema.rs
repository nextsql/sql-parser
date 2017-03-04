use nom::multispace;
use nom::alphanumeric;

use super::key::*;

#[derive(PartialEq,Debug,Default)]
pub struct CreateSchema<'a> {
    name : Key<'a>,
    charset:Option<&'a[u8]>,
    collation:Option<&'a[u8]>,
    if_not_exists:bool
}

named!(create_schema<&[u8],CreateSchema>,
       do_parse!(
           tag_no_case!(b"CREATE") >>
           multispace >>
           alt!(tag_no_case!(b"DATABASE") | tag_no_case!("SCHEMA")) >>
           multispace >>
           if_not_exists:opt!(tag_no_case!("IF NOT EXISTS")) >>
           many0!(multispace) >>
           name:key >>
           many0!(multispace) >>
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

             ({

                 let mut schema = CreateSchema{
                     name:name,
                     charset:charset,
                     collation:collation,
                     if_not_exists:false
                 };
                 if let Some(_) = if_not_exists {
                     schema.if_not_exists = true;
                 }
                 schema
             })
       )
);


#[test]
fn test_create_schema() {

    use nom::IResult;

    {
        let sql = b"create database test";
        assert_eq!(create_schema(sql),IResult::Done(&b""[..],
                                                    CreateSchema{
                                                        name : Key::new(b"test",false),
                                                        charset:None,
                                                        collation:None,
                                                        if_not_exists:false
                                                    }
        ));
    }

    {
        let sql = b"create database `test`";
        assert_eq!(create_schema(sql),IResult::Done(&b""[..],
                                                    CreateSchema{
                                                        name : Key::new(b"test",true),
                                                        charset:None,
                                                        collation:None,
                                                        if_not_exists:false
                                                    }
        ));
    }

    {
        let sql = b"create database if not exists test";
        assert_eq!(create_schema(sql),IResult::Done(&b""[..],
                                                    CreateSchema{
                                                        name : Key::new(b"test",false),
                                                        charset:None,
                                                        collation:None,
                                                        if_not_exists:true
                                                    }
        ));
    }

    {
        let sql = b"create database if not exists test CHARACTER SET = utf8";
        assert_eq!(create_schema(sql),IResult::Done(&b""[..],
                                                    CreateSchema{
                                                        name : Key::new(&b"test"[..],false),
                                                        charset:Some(&b"utf8"[..]),
                                                        collation:None,
                                                        if_not_exists:true
                                                    }
        ));
    }


    {
        let sql = b"create database if not exists test CHARACTER SET = utf8 COLLATE = none";
        assert_eq!(create_schema(sql),IResult::Done(&b""[..],
                                                    CreateSchema{
                                                        name : Key::new(&b"test"[..],false),
                                                        charset:Some(&b"utf8"[..]),
                                                        collation:Some(&b"none"[..]),
                                                        if_not_exists:true
                                                    }
        ));
    }

}
