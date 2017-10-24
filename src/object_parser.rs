use std::collections::HashMap;

use value::JstpValue;
use string_parser::string;
use parser::value;

named!(
    pub object<&str, HashMap<String, JstpValue>>,
    map!(
        ws!(delimited!(
            tag_s!("{"),
            separated_list!(tag_s!(","), key_value),
            tag_s!("}")
        )),
        |pairs| {
            pairs.into_iter().collect()
        }
    )
);

named!(
    key_value<&str, (String, JstpValue)>,
    ws!(separated_pair!(
        string,
        tag_s!(":"),
        value
    ))
);
