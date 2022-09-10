mod combine;

#[test]
fn combine_works() {
    use bpaf::*;
    let options = combine::options();

    let args = Args::from(&[]);
    let r = options.run_inner(args).unwrap();
    let r = format!("{:?}", r);
    assert_eq!(r, "Options { version: None, feature: None }");

    let args = Args::from(&["10"]);
    let r = options.run_inner(args).unwrap();
    let r = format!("{:?}", r);
    assert_eq!(r, "Options { version: Some(10), feature: None }");

    let args = Args::from(&["feature"]);
    let r = options.run_inner(args).unwrap();
    let r = format!("{:?}", r);
    assert_eq!(r, "Options { version: None, feature: Some(\"feature\") }");
}
mod derive;

#[test]
fn derive_works() {
    use bpaf::*;
    let options = derive::options();

    let args = Args::from(&[]);
    let r = options.run_inner(args).unwrap();
    let r = format!("{:?}", r);
    assert_eq!(r, "Options { version: None, feature: None }");

    let args = Args::from(&["10"]);
    let r = options.run_inner(args).unwrap();
    let r = format!("{:?}", r);
    assert_eq!(r, "Options { version: Some(10), feature: None }");

    let args = Args::from(&["feature"]);
    let r = options.run_inner(args).unwrap();
    let r = format!("{:?}", r);
    assert_eq!(r, "Options { version: None, feature: Some(\"feature\") }");
}
