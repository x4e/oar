# Contributing

Anyone is welcome to contribute to Oar.
Here is how to do so:

Fork the repo, then clone it:

```shell
git clone git@github.com:yourusernamehere/oar.git && cd oar
git checkout -b my-new-feature-branch
cargo build
```

Now apply any changes to the source code.
Make sure to abide by the following rules:

* Use tabs for indentation and alignment
* Keep indentation on blank lines
* Keep code and documentation professional: avoid swear words for example
* Always leave the campsite cleaner than you found it

When you have made your changes check that the code successfully builds and unit tests pass:

```shell
cargo test
```

Manually check it works:

```shell
oartmp=$(mktemp)
cat /usr/share/dict/words > $oartmp
cargo run $oartmp
rm $oartmp
```

Lint it:

```shell
cargo clippy -- -D warnings
```

Note: running clippy can sometimes fail with "`caused by: Compiler not supported`".
To fix run like: `RUSTC_WRAPPER="" cargo clippy -- -D warnings`.

Commit it:

```shell
git add src/
git commit -m "Description of the feature"
git push --set-upstream origin my-new-feature-branch
```

Create a pull request:

```
https://github.com/yourusernamehere/oar/pull/new/my-new-feature-branch
```
