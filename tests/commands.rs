use bpaf::*;

#[test]
fn chain_commands() {
    #[derive(Debug, PartialEq, Eq)]
    enum Cmd {
        A(bool),
        B(bool),
    }

    let a = short('a')
        .switch()
        .to_options()
        .command("a")
        .adjacent()
        .map(Cmd::A);
    let b = short('b')
        .switch()
        .to_options()
        .command("b")
        .adjacent()
        .map(Cmd::B);
    let parser = construct!([a, b]).many().to_options();

    let r = parser
        .run_inner(Args::from(&["a", "-a", "b", "a", "b", "-b"]))
        .unwrap();

    use Cmd::*;
    assert_eq!(&r, &[A(true), B(false), A(false), B(true)]);
}
