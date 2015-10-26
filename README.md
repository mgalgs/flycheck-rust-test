Test repo to iron out some issues with
[flycheck-rust](https://github.com/flycheck/flycheck-rust).


# Repro

    git clone https://github.com/mgalgs/flycheck-rust-test.git
    cd flycheck-rust-test.git
    cargo run
    cd frsub
    cargo build
    emacs lib.rs

Running `flycheck-buffer` in `frsub/lib.rs` results in:

```
    2   1 error           can't find crate for `env_logger`... (rust)
```
