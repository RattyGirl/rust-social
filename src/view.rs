#[macro_export]
macro_rules! make_view {
    ($filename: expr) => {
        std::fs::read_to_string(std::format!("www/{}",$filename)).unwrap_or($filename.to_string()).as_str()
    };
    ($filename: expr,, $($replacers: expr),*) => {{
        let mut out = std::fs::read_to_string(std::format!("www/{}",$filename)).unwrap_or($filename.to_string());
        $(
            out = str::replace(out.as_str(), $replacers.0, $replacers.1);
        )*
        out
    }};
}



#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!("{x}\n{y}\n{z}", make_view!("testpage.social"));
        assert_eq!("oo\n{y}\n{z}", make_view!("testpage.social",, ("{x}", "oo")));
        assert_eq!("oo\naaa\n{z}", make_view!("testpage.social",, ("{x}", "oo"),("{y}", "aaa")));
        assert_eq!("oo\naaa\nuwu", make_view!("testpage.social",, ("{x}", "oo"),("{y}", "aaa"),("{z}", "uwu")));
        assert_eq!("{x}\naaa\nuwu", make_view!("testpage.social",, ("{y}", "aaa"),("{z}", "uwu")));
    }
}