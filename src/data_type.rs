pub enum Integer {
    Bit(usize),
    TinyInt((usize,bool,bool)),
    SmallInt((usize,bool,bool)),
    MediumInt((usize,bool,bool)),
    Int((usize,bool,bool)),
    Integer((usize,bool,bool)),
    BigInt((usize,bool,bool)),
}

pub enum Float {
    Real((usize,usize,bool.bool)),
    Double((usize,usize,bool.bool)),
    Float((usize,usize,bool.bool)),
    Decimal((usize,usize,bool.bool)),
    Numberic((usize,usize,bool.bool)),
}

pub enum DateTime {

}

pub enum String {

}

pub enum Blob {

}
