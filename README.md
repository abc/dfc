# dfc - duplicate filename counter
dfc is a command-line utility useful for counting the number of files 
within a directory structure which have the same name.

This is useful for finding potential duplicate files quickly within a 
structure without looking at file sizes or hashing the files, meaning 
it can run over vastly more files much faster and without as much RAM 
usage.

## Usage
```
dfc <file path> <depth>

<file path> The directory in which to start looking for duplicate filenames.
<depth>     The level of recursion, i.e. the depth of subdirectories to search.
```

## Output

Diagnostic and error messages will be printed to STDERR. Once processing
is fully completed, a report containing a line for each unique filename,
and a number representing the number of times that filename was found
within the directory structure will be printed to STDOUT. A summary is
also produced, with the count of unique, duplicate and total files.
This report is also printed to STDOUT.

## License

This project is licensed under either of

- <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or
- <a href="LICENSE-MIT">MIT license</a>

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in clarion by you, as defined in the Apache-2.0 license, shall be 
dual licensed as above, without any additional terms or conditions.