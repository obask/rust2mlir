use rustc_parse;
use rustc_session::parse::ParseSess;
use rustc_span;
use rustc_span::source_map::FilePathMapping;

fn main() {
    let file = rustc_span::FileName::Custom("some_file.rs".to_owned());
    let input = String::from("fn main() { }");
    rustc_span::create_default_session_if_not_set_then(|_| {
        let parse_sess = ParseSess::new(FilePathMapping::empty());
        let x = rustc_parse::parse_crate_from_source_str(file.clone(), input.clone(), &parse_sess)
            .expect("can't parse");
        println!("{:?}", x);
    });
}
