
This is a relatively small json parser,

The main parser is in src/*.rs and contains few frills.

In addition to that theres an extras/ module which contains pretty error handling,
testsuite stuff

While it does not directly use unsafe code,
is may be subject to denial of service attacks through dynamic allocation, panics, etc.
It is relatively small, somewhat conforming, and not the most efficient around.

The intent was to make a parser which was "obviously conforming", when comparing side by side to the grammar given by JSON, that has not quite work out as planned. But it isn't too terrible.
