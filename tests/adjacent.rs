use bpaf::*;

#[test]
fn posix_optional_option_argument() {
    // can't believe I'm doing this :)

    let a = short('o').req_flag(());
    let b = positional("FILE").optional();
    let oo = construct!(a, b).adjacent();
    let c = short('s').switch();
    let parser = construct!(oo, c).to_options();

    let r = parser.run_inner(Args::from(&["-ofile.rs", "-s"])).unwrap();

    todo!("{:?}", r);

    // this one must fail
    let r = parser
        .run_inner(Args::from(&["-o", "file.rs", "-s"]))
        .unwrap();
}
