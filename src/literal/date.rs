//in parser,we cannt valid the date format,

use nom::multispace;
use super::string::StringType;
use super::string::string_type;

pub enum DateTime<'a>{
    Date(StringType<'a>),
    Time(StringType<'a>),
    Timestamp(StringType<'a>)
}

named!(date<&[u8],DateTime>,
       do_parse!(
           alt!(tag_no_case!("DATE") | tag_no_case!("d")) >>
           many0!(multispace) >>
           s:string_type >>
           (DateTime::Date(s))
       )
);

named!(time<&[u8],DateTime>,
       do_parse!(
           alt!(tag_no_case!("TIME") | tag_no_case!("t")) >>
               many0!(multispace) >>
               s:string_type >>
               (DateTime::Time(s))
       )
);

named!(timestamp<&[u8],DateTime>,
       do_parse!(
           alt!(tag_no_case!("TIMETIMESTAMP") | tag_no_case!("ts")) >>
               many0!(multispace) >>
               s:string_type >>
               (DateTime::Timestamp(s))
       )
);

named!(pub datetime<&[u8],DateTime>,
       alt!(date | time | timestamp)
);








