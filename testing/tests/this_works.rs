use askama::Template;

#[derive(Template)]
#[template(path = "this_works.html")]
struct ThisWorksTemplate;

#[test]
fn test_this_works() {
    let s = ThisWorksTemplate {};
    println!("{}", s.render().unwrap());
    // See print out
    // Run with:
    // $ cargo test --test this_works -- --nocapture
    assert!(true)
}

#[derive(Template)]
#[template(path = "this_also_works.html")]
struct ThisAlsoWorksTemplate;

#[test]
fn test_this_also_works() {
    let s = ThisWorksTemplate {};
    println!("{}", s.render().unwrap());
    // See print out
    // Run with:
    // $ cargo test --test this_works -- --nocapture
    assert!(true)
}
