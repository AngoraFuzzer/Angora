#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SearchMethod {
    Gd,
    Random,
    Cbh,
    Mb,
}

pub fn parse_search_method(m: &str) -> SearchMethod {
    match m {
        "gd" => SearchMethod::Gd,
        "random" => SearchMethod::Random,
        "cbh" => SearchMethod::Cbh,
        "mb" => SearchMethod::Mb,
        _ => SearchMethod::Gd,
    }
}
