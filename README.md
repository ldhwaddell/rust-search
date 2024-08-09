## rust-search

A simple TF-IDF based search engine written in Rust.

Supported documents formats:

- `txt`

Supported stemming techniques:

- Porter
  - [Link](https://snowballstem.org/algorithms/porter/stemmer.html)
  - .sbl: `src/snowball/algorithms/sbl/porter.sbl`
  - Output: `src/snowball/algorithms/porter.sbl`
- Porter2
  - [Link](https://snowballstem.org/algorithms/english/stemmer.html)
  - .sbl: `src/snowball/algorithms/sbl/porter2.sbl`
  - Output: `src/snowball/algorithms/porter2.sbl`


All snowball files compiled using the [snowball compiler](https://github.com/snowballstem/snowball/tree/master)
