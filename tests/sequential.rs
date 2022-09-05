use bpaf::*;

#[test]
fn sequential_parser() {
    let a = short('a').req_flag(());
    let b = short('b').switch();
    let c = short('c').switch();
    let parser = construct!(a, b, c).sequential().many().to_options();
    //let parser = construct!(a, b, c).many().to_options();

    let r = parser.run_inner(Args::from(&["-a", "-c", "-a", "-b"]));
    todo!("{:?}", r);
}
