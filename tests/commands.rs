use bpaf::*;

#[test]
fn chain_commands() {
    #[derive(Debug, PartialEq, Eq)]
    enum Cmd {
        A(bool),
        B(bool),
    }

    let a = short('a').switch().to_options().command("a").map(Cmd::A);
    let b = short('b').switch().to_options().command("b").map(Cmd::B);
    let parser = construct!([a, b]).many().to_options();

    let r = parser
        .run_inner(Args::from(&["a", "-a", "b", "a", "b", "-b"]))
        .unwrap();

    todo!("{:?}", r)
}
