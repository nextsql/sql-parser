use nom::multispace;
use nom::alphanumeric;

use super::key::*;

#[derive(PartialEq,Debug,Default)]
pub struct DropSchema<'a> {
    name : Key<'a>,
    if_exists:bool
}

named!(drop_schema<&[u8],DropSchema>,
       do_parse!(
           tag_no_case!("DROP") >>
           multispace >>
           alt!(tag_no_case!("DATABASE") | tag_no_case!("SCHEMA")) >>
           multispace >>
           if_exists:opt!(tag_no_case!("IF EXISTS")) >>
           many0!(multispace) >>
           name : key >>
           (
             {
               let mut s = DropSchema{name:name,if_exists:false};
               if let Some(_) = if_exists {
                   s.if_exists = true;
               }
             s
            }
           )
       ));


#[test]
fn test_drop_schema() {
    use nom::IResult;

    {
        let sql = b"drop database test";
        assert_eq!(drop_schema(sql),IResult::Done(&b""[..],
                                                    DropSchema{
                                                        name : Key::new(b"test",false),
                                                        if_exists:false
                                                    }
        ));
    }

    {
        let sql = b"drop database if exists test";
        assert_eq!(drop_schema(sql),IResult::Done(&b""[..],
                                                  DropSchema{
                                                      name : Key::new(b"test",false),
                                                      if_exists:true
                                                  }
        ));
    }


}




