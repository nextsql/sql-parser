extern crate nom;

use nom::IResult;
use nom::IResult::Done;
use nom::IResult::Error;
use nom::IResult::Incomplete;
use nom::Needed;
use nom::ErrorKind;

pub type Number<'a> = &'a[u8];

//[+/-]digit.digit
//[+/-]digit.digite-12
//
pub fn number<'a>(input:&'a[u8]) -> IResult<&'a[u8],Number> {

    let input_len = input.len();

    if input_len == 0 {
        return Incomplete(Needed::Unknown)
    }

    let mut once_checked:u8 = 0;

    for (idx,chr) in input.iter().enumerate() {

        match *chr as char {

            'e' | 'E' | '.' => {
                if idx < 1 || !nom::is_digit(input[idx-1]) {
                    return Error(error_position!(ErrorKind::Custom(0), input))
                }

                if false  == match *chr as char {
                    'e' => {
                        if once_checked & 1 << 3 == 1 << 3 {
                            false
                        }else{
                            once_checked += 1 << 3;
                            true
                        }
                    },
                    'E' => {
                        if once_checked & 1 << 2 == 1 << 2 {
                            false
                        }else{
                            once_checked += 1 << 2;
                            true
                        }
                    },
                    '.' => {
                        if once_checked & 1 << 1 == 1 << 1 {
                            false
                        }else{
                            once_checked += 1 << 1;
                            true
                        }
                    },
                    _ => {
                        false
                    }
                }{
                    return Error(error_position!(ErrorKind::Custom(0), input))
                }
            },
            '+' | '-'  => {
                if !(idx == 0 || (input[idx-1] == 'e' as u8 || input[idx-1] == 'E' as u8) ) {
                    return Error(error_position!(ErrorKind::Custom(0), input))
                }
            },

            _ => {
                if nom::is_digit(*chr) {
                    continue
                }
                if nom::is_space(*chr) {
                    if idx < 1  || !nom::is_digit(input[idx-1]) {
                        return Error(error_position!(ErrorKind::Custom(0), input))
                    }
                    return Done(&input[idx..],&input[..idx])
                }
                return Error(error_position!(ErrorKind::Custom(0), input))
            }
        }
    }
    if !nom::is_digit(input[input_len-1]) {
        return Error(error_position!(ErrorKind::Custom(0), input))
    }
    return Done(&input[input_len..],&input[..input_len])
}

#[test]
fn test_number() {
    assert_eq!(number(&b"123"[..]),IResult::Done(&b""[..],&b"123"[..]));
    assert_eq!(number(&b"+123"[..]),IResult::Done(&b""[..],&b"+123"[..]));
    assert_eq!(number(&b"-123"[..]),IResult::Done(&b""[..],&b"-123"[..]));
    assert_eq!(number(&b"123.123"[..]),IResult::Done(&b""[..],&b"123.123"[..]));
    assert_eq!(number(&b"123.123e12"[..]),IResult::Done(&b""[..],&b"123.123e12"[..]));
    assert_eq!(number(&b"123.123e-12"[..]),IResult::Done(&b""[..],&b"123.123e-12"[..]));
    assert_eq!(number(&b"abc"[..]),IResult::Error(error_position!(ErrorKind::Custom(0), &b"abc"[..])));
    assert_eq!(number(&b"12abc"[..]),IResult::Error(error_position!(ErrorKind::Custom(0), &b"12abc"[..])));
    assert_eq!(number(&b".1"[..]),IResult::Error(error_position!(ErrorKind::Custom(0), &b".1"[..])));
    assert_eq!(number(&b"1."[..]),IResult::Error(error_position!(ErrorKind::Custom(0), &b".1"[..])));
}
