<details>
<summary>Combinatoric usage</summary>

```no_run
# use bpaf::*;
#[derive(Debug, Clone)]
# #[allow(dead_code)]
pub struct Options {
    version: Option<usize>,
    feature: Option<String>,
}
pub fn options() -> OptionParser<Options> {
    let version = positional("VERS").from_str::<usize>().optional().catch();
    let feature = positional("FEAT").optional();
    construct!(Options { version, feature }).to_options()
}
```

</details>
<details>
<summary>Derive usage</summary>

```no_run
# use bpaf::*;
#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
# #[allow(dead_code)]
pub struct Options {
    #[bpaf(positional("VERS"), catch)]
    version: Option<usize>,
    #[bpaf(positional("FEAT"), catch)]
    feature: Option<String>,
}
```

</details>
<details>
<summary>Examples</summary>


No argument, both parsers succeed due to [`option`]
```console
% app 
Options { version: None, feature: None }
```

Decimal value - version parser succeeds
```console
% app 10
Options { version: Some(10), feature: None }
```

String value - version parser fails, catch handles that.
```console
% app feature
Options { version: None, feature: Some("feature") }
```

</details>
