use nom::is_space;

#[inline]
pub fn is_quote(chr:u8) -> bool {
    chr == '`' as u8
}

pub fn not_quote(chr:u8) -> bool {
    !is_quote(chr)
}

pub fn not_space(chr:u8) -> bool {
    !is_space(chr)
}

pub fn is_keyword_symbol(chr:u8) -> bool {
    is_quote(chr) || is_space(chr) || chr == '\'' as u8 || chr == '"' as u8 || chr == '(' as u8 || chr == ')' as u8
}


pub fn not_keyword_symbol(chr:u8) -> bool {
    !is_keyword_symbol(chr)
}
