use bpaf::*;

#[test]
fn posix_optional_option_argument() {
    let a0 = short('o').req_flag(());
    let a1 = positional("FILE");
    let a = construct!(a0, a1).adjacent().map(|x| Some(x.1));
    //    let a = short('o').argument("FILE").adjacent().map(Some);
    let b = short('o').req_flag(None);
    let oo = construct!([a, b]);

    //    let a = short('o').req_flag(());
    //    let b = positional("FILE").optional();
    //   let oo = construct!(a, b).adjacent();
    let c = short('s').switch();
    let parser = construct!(oo, c).to_options();

    let r = parser.run_inner(Args::from(&["-ofile.rs", "-s"])).unwrap();

    todo!("{:?}", r);

    // this one must fail
    let r = parser
        .run_inner(Args::from(&["-o", "file.rs", "-s"]))
        .unwrap();
}
