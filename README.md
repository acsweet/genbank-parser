# genbank-parser
- a greedy genbank parser
- can be modified easily for full information

# use
- build and call e.g. (on mac) <code>.genbank-parser gbvrl1.seq</code>
- can specify a single .seq file or a directory, and it will process all .seq files in that directory
    - e.g. <code>.genbank-parser path/to/gbvrl1.seq</code> or <code>.genbank-parser path/to/files/</code>

# TODO
- [x] Take .seq file or directory of .seq files as input and process all
- [ ] Unzip .gz files
- [ ] Handle multiple file reading and writing asynchronously
:white_check_mark: Store animo acid and nucleic acid sequences as key value pairs with their accession ids
- [ ] Store viral taxanomies independently
:white_large_square: Store viral genomic and protein metadata independently

:warning: code could undergo dramatic changes, particularly around outputs
