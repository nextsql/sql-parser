pub enum NULL{}

name!(null<&[u8],NULL>,
      map!(
          tag_no_case!("null"),
          ||{NULL}
      )
);
