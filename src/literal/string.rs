/// MYSQL String literal definition
/// see infomation [https://dev.mysql.com/doc/refman/5.7/en/string-literals.html]
/// there is full imlplation of mysql(5.7) string parser

use nom;
use nom::IResult;
use nom::IResult::Error;
use nom::IResult::Done;
use nom::ErrorKind;
use nom::Needed;
use nom::IResult::Incomplete;

/// [_charset] StringType [COLLATE collation_name];
/// first is charset,second is collate
pub type Charset<'a> = (&'a[u8], &'a[u8]);

/// mysql has some ways to definition a string
#[derive(Debug,PartialEq)]
pub enum StringType<'a> {
    /// [n|N]'naition'
    SingleQuoteNotion(&'a[u8]),
    /// b'0101010'
    SinleQuoteBinary(&'a[u8]),
    /// 'string'
    SingleQuoteString(&'a[u8]),
    // X'0F12D'
    SingleQuooteHex(&'a[u8]),
    /// "string"
    DoubleQuoteString(&'a[u8]),
    /// 0b1010101001
    Binary(&'a[u8]),
    /// `string`
    BacktickString(&'a[u8]),
    //abcd
    NoWrapString(&'a[u8]),
}

pub type String<'a> = (StringType<'a>, Option<Charset<'a>>);

fn string_type<'a>(input:&'a[u8]) -> IResult<&'a[u8],StringType<'a>> {

    let input_len = input.len();

    if input_len == 0 {
        return Incomplete(Needed::Unknown)
    }

    let first_byte = input[0];
    let second_byte = if input_len > 1 {
        Some(input[1] as char)
    }else{
        None
    };

    let mut start_idx = 0;
    let mut mode = 0;
    let mut end_char:u8 = first_byte;
    match (first_byte as char,second_byte) {

        ('\'',_) => {mode = 1;start_idx=1;},
        ('"',_) => {mode = 2;start_idx=1;},
        ('`',_) => {mode=3;start_idx=1;}
        ('x',Some('\'')) | ('X',Some('\''))  => {
            end_char = '\'' as u8;
            mode = 4;
            start_idx = 2;
        },
        ('n',Some('\'')) | ('N',Some('\'')) => {
            end_char = '\'' as u8;
            mode = 5;
            start_idx = 2;
        },
        ('b',Some('\'')) => {
            end_char = '\'' as u8;
            mode = 6;
            start_idx = 2;
        },
        ('0',Some('b')) => {
            mode = 7;
            start_idx = 2;
        },
        _ => {
            mode = 0;
            start_idx = 0;
        }
    }

    let mut skip_next:bool = false;
    for (idx,chr) in input[start_idx..].iter().enumerate() {
        let is_end = if idx == input_len - start_idx - 1 {
            true
        }else{
            false
        };
        if skip_next {
            skip_next = false;
            continue;
        }
        match mode {

            1 => {
                if *chr == end_char as u8 {
                    if is_end {
                        return Done(&input[idx+start_idx+1..],StringType::SingleQuoteString(&input[start_idx..idx+start_idx]))
                    }else if input[idx+1] == end_char as u8 {
                        skip_next = true;
                        continue;
                    }else{
                        return Done(&input[idx+start_idx+1..],StringType::SingleQuoteString(&input[start_idx..idx+start_idx]))
                    }
                }
        },

            2 => {
            if *chr == end_char as u8 {
                if is_end {
                    return Done(&input[idx+start_idx+1..],StringType::DoubleQuoteString(&input[start_idx..idx+start_idx]))
                }else if input[idx+1] == end_char as u8 {
                    skip_next = true;
                    continue;
                }else{
                    return Done(&input[idx+start_idx+1..],StringType::DoubleQuoteString(&input[start_idx..idx+start_idx]))
            }
        }
    },
            3 => {
         if *chr == end_char as u8 {
             return Done(&input[idx+start_idx+1..],StringType::BacktickString(&input[start_idx..idx+start_idx]))
         }
    },
            4 => {
                if *chr == end_char as u8 {
                    if is_end {
                        return Done(&input[idx+start_idx+1..],StringType::SingleQuooteHex(&input[start_idx..idx+start_idx]))
                    }else if input[idx+1] == end_char as u8 {
                        skip_next = true;
                        continue;
                    }else{
                        return Done(&input[idx+start_idx+1..],StringType::SingleQuooteHex(&input[start_idx..idx+start_idx]))
                    }
                }
                if !nom::is_hex_digit(*chr) {
                    return Error(error_position!(ErrorKind::Custom(0), input))
                }
            },

            5 => {
                if *chr == end_char as u8 {
                    if is_end {
                        return Done(&input[idx+start_idx+1..],StringType::SingleQuoteNotion(&input[start_idx..idx+start_idx]))
                    }else if input[idx+1] == end_char as u8 {
                        skip_next = true;
                        continue;
                    }else{
                        return Done(&input[idx+start_idx+1..],StringType::SingleQuoteNotion(&input[start_idx..idx+start_idx]))
                    }
                }
            }

            6 => {
                if *chr == end_char as u8 {
                    return Done(&input[idx+start_idx+1..],StringType::SinleQuoteBinary(&input[start_idx..idx+start_idx]))
                }

                if *chr != '0' as u8 && *chr != '1' as u8 {
                    return Error(error_position!(ErrorKind::Custom(0), input))
                }
                
            },

            7 => {
                if nom::is_space(*chr) {
                    return Done(&input[idx+start_idx..],StringType::Binary(&input[start_idx..idx+start_idx+1]))
                }
                if is_end {
                    return Done(&input[idx+start_idx+1..],StringType::Binary(&input[start_idx..idx+start_idx+1]))
                }
                if *chr != '0' as u8 && *chr != '1' as u8 {
                    return Error(error_position!(ErrorKind::Custom(0), input))
                }
            },

            _ => {
                if nom::is_space(*chr) {
                    return Done(&input[idx+start_idx..],StringType::NoWrapString(&input[start_idx..idx+start_idx+1]))
                }
                if is_end {
                    return Done(&input[idx+start_idx+1..],StringType::NoWrapString(&input[start_idx..idx+start_idx+1]))
                }
            }
    }
    }
    IResult::Done(&b""[..],StringType::Binary(&b""[..]))
}


#[test]
fn test_str() {
    assert_eq!(string_type(&b"abc"[..]),Done(&b""[..],StringType::NoWrapString(&b"abc"[..])));
    assert_eq!(string_type(&b"'abc'"[..]),Done(&b""[..],StringType::SingleQuoteString(&b"abc"[..])));
    assert_eq!(string_type(&b"\"abc\""[..]),Done(&b""[..],StringType::DoubleQuoteString(&b"abc"[..])));
    assert_eq!(string_type(&b"`abc`"[..]),Done(&b""[..],StringType::BacktickString(&b"abc"[..])));
    assert_eq!(string_type(&b"x'1a2b3c'"[..]),Done(&b""[..],StringType::SingleQuooteHex(&b"1a2b3c"[..])));
    assert_eq!(string_type(&b"N'abcd '"[..]),Done(&b""[..],StringType::SingleQuoteNotion(&b"abcd "[..])));

    assert_eq!(string_type(&b"b'01010101'"[..]),Done(&b""[..],StringType::SinleQuoteBinary(&b"01010101"[..])));
    assert_eq!(string_type(&b"0b01010101"[..]),Done(&b""[..],StringType::Binary(&b"01010101"[..])));
}
