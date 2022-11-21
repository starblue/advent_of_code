# advent_of_code

Solutions to Advent of Code problems in Rust

## Notes

This is my almost complete collection of solutions starting from 2015
(2021 50\*, 2020 48\*, 2019 35\*, 2018 46\*, 2017 47\*, 2016 50\*, 2015 50\*).

This is not production code, rather it is relatively quick
and hopefully not too dirty code to get the correct result.
In particular:

* There is no error handling, so it may panic or otherwise misbehave
  on mal-formed input.
* Parsing of the input is not precise, as long as the correct data
  can be extracted from the actual input that is fine.

The focus is more on algorithms and using appropriate data structures
to get both reasonably clean code and reasonable efficiency.

Most solutions use `nom` for parsing. The resulting internal data structures
usually implement `std::fmt::Display` such that the output is identical
to the parsed input; I use that for validating the parser.

2015 day 20 and 2020 day 25 use local crates which are not yet published,
for factorizing integers and for computing modulo.
I apologize and hope you can still make sense of them.
