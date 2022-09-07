use bpaf::*;

#[test]
fn sequential_parser() {
    let a = short('a').req_flag(());
    let b = short('b').switch();
    let c = short('c').switch();
    let parser = construct!(a, b, c).adjacent().many().to_options();
    //let parser = construct!(a, b, c).many().to_options();

    let r = parser.run_inner(Args::from(&["-a", "-c", "-a", "-b"]));
    todo!("{:?}", r);
}

#[test]
fn experiment_paaa() {
    let a = short('a').req_flag(());
    let b = short('b').req_flag(());
    let c = short('c').req_flag(());
    let parser = construct!(a, b, c).adjacent().to_options();

    let r = parser.run_inner(Args::from(&["-x", "-x", "-a", "-x", "-x", "-x", "-x"]));
    todo!("{:?}", r);
}

#[test]
fn multi_argument() {
    let a = short('a').req_flag(());
    let b = positional("X").from_str::<u32>();
    let c = positional("Y").from_str::<u32>();
    let d = positional("Z").from_str::<u32>();
    let e = short('e').argument("XXX").from_str::<u32>().optional();
    let chunky = construct!(a, b, c, d).adjacent().optional();

    let f = short('f').switch();

    let parser = construct!(chunky, e, f).to_options();

    let r = parser.run_inner(Args::from(&["-e", "1"])).unwrap();
    assert_eq!(r, (None, Some(1), false));

    let r = parser
        .run_inner(Args::from(&["-e", "1", "-a", "2", "3", "4", "-f"]))
        .unwrap();
    assert_eq!(r, (Some(((), 2, 3, 4)), Some(1), true));

    let r = parser
        .run_inner(Args::from(&["-a", "a", "b", "c", "-e", "x"]))
        .unwrap();
    assert_eq!(r, (Some(((), 2, 3, 4)), Some(1), false));

    let r = parser
        .run_inner(Args::from(&["-a", "a", "b", "c", "-e", "x"]))
        .unwrap();
    assert_eq!(r, (Some(((), 2, 3, 4)), Some(1), false));
}
