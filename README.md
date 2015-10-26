Test repo to iron out some issues with
[flycheck-rust](https://github.com/flycheck/flycheck-rust).


Currently, running `flycheck-buffer` in `frsub/lib.rs` results in:

```
    1  14 error           multiple matching crates for `log`... (rust)
    1  14 error           can't find crate for `log`... (rust)
```

Running it in `src/main.rs` results in roughly the same thing:

```
    3  14 error           multiple matching crates for `log`... (rust)
    3  14 error           can't find crate for `log`... (rust)
```
