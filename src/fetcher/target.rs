// This is free and unencumbered software released into the public domain.

#[derive(Debug, PartialEq, Eq)]
pub enum FetchTarget {
    Discover,
    Category(String),
    Calendar(String),
    Place(String),
    Event(String),
    Unknown(String),
}

pub fn parse_fetch_url<S>(url: S) -> Option<FetchTarget>
where
    S: AsRef<str>,
{
    let url = url::Url::parse(url.as_ref()).ok()?;

    if !matches!(url.scheme(), "https" | "http") {
        return None;
    }

    if url.host_str() != Some("lu.ma") {
        return None;
    }

    match url.path() {
        "/discover" => Some(FetchTarget::Discover),
        _ => {
            let path_segments = url.path_segments()?;

            if path_segments.count() != 1 {
                return None;
            }

            let path = url.path()[1..].to_string();
            if path.is_empty() {
                return None;
            }

            let mut query = url.query_pairs();
            let k = query.find(|(name, _)| name == "k");

            if let Some((_, v)) = k {
                match v.as_ref() {
                    "t" => return Some(FetchTarget::Category(path)),
                    "c" => return Some(FetchTarget::Calendar(path)),
                    "p" => return Some(FetchTarget::Place(path)),
                    _ => (),
                }
            }

            Some(FetchTarget::Unknown(path))
        },
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_parse_fetch_url() {
        assert_eq!(parse_fetch_url("http://lu.ma"), None);
        assert_eq!(parse_fetch_url("http://lu.ma/"), None);
        assert_eq!(
            parse_fetch_url("http://lu.ma/discover"),
            Some(FetchTarget::Discover)
        );
        assert_eq!(
            parse_fetch_url("https://lu.ma/discover"),
            Some(FetchTarget::Discover)
        );
        assert_eq!(
            parse_fetch_url("https://lu.ma/ai?k=t"),
            Some(FetchTarget::Category(String::from("ai")))
        );
        assert_eq!(
            parse_fetch_url("https://lu.ma/southparkcommons-events?k=c"),
            Some(FetchTarget::Calendar(String::from(
                "southparkcommons-events"
            )))
        );
        assert_eq!(
            parse_fetch_url("https://lu.ma/sf?k=p"),
            Some(FetchTarget::Place(String::from("sf")))
        );
        assert_eq!(
            parse_fetch_url("http://lu.ma/a"),
            Some(FetchTarget::Unknown(String::from("a")))
        );
        assert_eq!(
            parse_fetch_url("https://lu.ma/g73p0wlk"),
            Some(FetchTarget::Unknown(String::from("g73p0wlk")))
        );
    }
}
