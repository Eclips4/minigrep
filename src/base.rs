pub struct Information<'a> {
    pub string: &'a str,
    pub line_number: u32,
}


pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
    pub ignore_case: bool,
}


pub type Search<'a> =  Option<Vec<Information<'a>>>;
