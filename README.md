
This is a relatively small json parser,

While it does not directly use unsafe code,
is may be subject to denial of service attacks panics, through dynamic allocation, panics,
It is relatively small, somewhat conforming, and not the most efficient around.

The intent was to make a parser which was "obviously conforming", when comparing side by side to the
 grammar given by JSON, that has not quite work out as planned. But it isn't too terrible.
