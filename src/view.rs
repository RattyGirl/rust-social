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
        assert_eq!("{x}\r\n{y}\r\n{z}", make_view!("testpage.html"));
        assert_eq!("oo\r\n{y}\r\n{z}", make_view!("testpage.html",, ("{x}", "oo")));
        assert_eq!("oo\r\naaa\r\n{z}", make_view!("testpage.html",, ("{x}", "oo"),("{y}", "aaa")));
        assert_eq!("oo\r\naaa\r\nuwu", make_view!("testpage.html",, ("{x}", "oo"),("{y}", "aaa"),("{z}", "uwu")));
        assert_eq!("{x}\r\naaa\r\nuwu", make_view!("testpage.html",, ("{y}", "aaa"),("{z}", "uwu")));
    }
}