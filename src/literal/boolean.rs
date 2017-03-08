pub enum Boolean{
    True,
    False
}

named!(_true<&[u8],Boolean>,
       map!(
           tag_no_case!("true"),
           ||{Boolean::True}
       )
);

named!(_false<&[u8],Boolean>,
       map!(
           tag_no_case!("false"),
           ||{Boolean::False}

       )
);

named!(boolean<&[u8],Boolean>,
       alt!(_true|_false)
);
