# genbank-parser
- a greedy genbank parser
- can be modified easily for full information
- see [dev branch](https://github.com/acsweet/genbank-parser/tree/develop) for latest changes

# use
- install
```
cargo build --release
```
- call with `.seq` file
```
.genbank-parser gbvrl1.seq
```

# TODO
- [ ] Take .seq file or directory of .seq files as input and process all
- [ ] Unzip .gz files
- [ ] Handle multiple file reading and writing asynchronously

:warning: code could undergo dramatic changes, particularly around outputs
