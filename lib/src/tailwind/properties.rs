use std::any::type_name;
use std::fmt;
use yew::Properties;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct DisplayProps {
    #[prop_or(false)]
    pub block: bool,
    #[prop_or(false)]
    pub hidden: bool,
    #[prop_or(false)]
    pub inline: bool,
    #[prop_or(false)]
    pub inline_block: bool,
    #[prop_or(false)]
    pub table: bool,
    #[prop_or(false)]
    pub table_cell: bool,
    #[prop_or(false)]
    pub table_row: bool,
}

impl From<&DisplayProps> for Vec<String> {
    fn from(dp: &DisplayProps) -> Vec<String> {
        let mut vec = Vec::new();

        if dp.block {
            vec.push("block".to_string());
        }
        if dp.hidden {
            vec.push("hidden".to_string());
        }
        if dp.inline {
            vec.push("inline".to_string());
        }
        if dp.inline_block {
            vec.push("inline-block".to_string());
        }
        if dp.table {
            vec.push("table".to_string());
        }
        if dp.table_cell {
            vec.push("table-cell".to_string());
        }
        if dp.table_row {
            vec.push("table-row".to_string());
        }

        vec
    }
}

impl fmt::Display for DisplayProps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = Vec::from(self);
        write!(f, "{}", vec.join(" "))
    }
}

#[derive(Clone, PartialEq)]
pub enum FloatOption {
    None,
    Right,
    Left,
}

impl fmt::Display for FloatOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FloatOption::None => write!(f, "none"),
            FloatOption::Right => write!(f, "right"),
            FloatOption::Left => write!(f, "left"),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct FloatProps {
    #[prop_or(false)]
    pub clearfix: bool,
    pub float: Option<FloatOption>,
}

// All of this is to try out mapping props to class names.

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

#[allow(dead_code)]
fn get_vec<T: Copy>(value: T) -> Vec<T> {
    vec![value]
    // match type_of(value).find("alloc::vec::Vec") {
    //     true => value,
    //     false => vec![value],
    // }
}

fn split_prop(_prop: yew::virtual_dom::AttrValue) -> (String, String) {
    ("".to_string(), "".to_string())
}

fn create_class_name(_utility: String, _variant: String, _prefix: String) -> String {
    "".to_string()
}

#[allow(dead_code)]
fn prop_to_class_name<T>(prop: yew::virtual_dom::AttrValue, values: T, prefix: String) -> String {
    let (utility, variant) = split_prop(prop);

    match type_of(values) {
        "bool" => create_class_name(utility, variant, prefix),
        // "alloc::vec::Vec<&str>" => {},
        _ => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_the_type_name() {
        struct Foo<'a> {
            #[allow(dead_code)]
            foo: &'a str,
        }

        assert_eq!(type_of(1_000), "i32");
        assert_eq!(type_of("foo"), "&str");
        assert_eq!(type_of(vec!["foo"]), "alloc::vec::Vec<&str>");
        assert_eq!(type_of(Foo { foo: "bar" }), "");
        assert_eq!(type_of(true), "bool");
    }

    #[test]
    fn it_gets_value_as_type() {
        assert_eq!(get_vec("foo"), (Some("foo"), None));
        // assert_eq!(get_vec(vec!["foo"]), (None, Some(vec!["foo"])));
    }
}
