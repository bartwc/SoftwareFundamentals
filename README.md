# Getting Started

1. Clone this repository
2. Open the project in an editor (CLion or VS Code)
3. Run with `cargo run < examples/[a dsmr file].dsmr`
4. Test with `cargo test`
5. View the output by opening `output/dsmr.html` in your browser. 
   * From your terminal you can run `firefox output/dsmr.html` or `chrome output/dsmr.html` or similar commands. `xdg-open output/dsmr.html` may also work on Linux.
6. Submit by pushing commits to your git repository

# Requirements for Telegram Content
A Telegram must have certain fields. Telegrams which do not follow the requirements are invalid and MUST be rejected.

Checked 1 | 2 | 5 | 6 | 7 | 8
Unchecked 3 | 4

1. Each Telegram MUST contain a start, an end, and date - Y
2. A telegram MUST contain up to 3 child telegrams (0 is possible) - Y
3. A telegram MUST only contain information from a single meter: Gas, Water or Electricity. Fields with information of different meters cannot be combined in a single telegram. 
4. A telegram MUST contain exactly one 4.1 information type field
5. The top level telegram MUST only be from a Water or Electricity meter. Gas meters MUST only be child meters and their information is contained in child telegrams. - Y
6. Empty lines and stray newlines MUST be ignored - Y
7. In a telegram, all field IDs MUST be unique. - Y
8. Every information type (Eventlog, Water, Gas and Electricity) MUST be complete. For example, the electricity data MUST contain voltage, current and power of all three phases and the total consumed and produced power. Another example, every event log MUST have a date, message and severity. - Y

# Multiple Telegrams
In a single .dsmr file, multiple telegrams MAY be stored. The telegrams come, concatenated, with zero or more newlines inbetween. When multiple telegrams are concatenated, only one header MUST be present. - Y

When multiple voltages are present of the same phase, you MUST only record the highest voltage per phase for purposes of the graphs. The same is true for current, that is to say, only record the highest. To repeat, all other quantities, for example power, should be summed. - N

# Learning Goals
1 - Gain confidence in your ability to write a small but useful Rust program, specifically:
      use control flow primitives and use match
      design data types to match data
      work with string data
      using iterators
      using recursion
      writing tests
      handle errors and validate input
2 - Learn how to read requirements and implement them
3 - Learn how to write robust programs by writing (unit) tests and applying proper error handling
4 - Gain some elementary knowledge about using the Git VCS

# Grading

Your grade starts at a 1. For each requirement you implement you get the specified amount of points below. Up to a maximum grade of a 10. You will see that in the rubric below, you can receive up to 9 points reflecting this.

You need to have a minimum of a 5.0 for this assignment to be allowed to follow the second part of this course (the group project) and thus be able to pass this course.

1 - Implement the complete parsing of v1 of the protocol (2pts).
      You have to use structs, enums and at least one match statement
      note: if unable to parse certain constructs, partial points may be awarded

2 - A sensible data format that represents the structure of telegrams. This item will be manually graded based to up to (1pt). With "sensible", we mean:
      You use enums to represent parts of the format which make sense to be enums.
      You use structs to represent parts of the format which make sense to be structs.
      Certain telegram structures which are never valid (hint: telgrams containing different kinds of information at the same time) cannot be represented by your format.

3 - Reject invalid telegrams (1pts)
      This requires you to reject invalid telegrams of all the protocol versions you implement
      On rejection: exit gracefully with exit code 42.
      note: partial points may be awarded

4 - Output of the aggregated data format as specified in the assignment above (1pts)
      Make sure to also do this for v1.2 of the protocol when you implement it.
      With this we mean the plotting of the data using the tudelft-dsmr-output-generator.

5 - Unit testing of components of your program (0.75pts)
      A few basic smoke tests (0.15pts)
      Substantial testing of at least 10 distinct unit tests covering 40%1 of your code (0.15pts)
      Thorough testing with unit and integration tests with in total least 25 distinct tests covering 80%1 of your code (0.3pts)
      Unit and smoke testing as defined in Lecture 7 (0.15 points)
      note: no further partial points can be obtained

6 - Error Handling (0.75pts)
      tip: use .expect(..) in your code instead of simply .unwrap() before attempting this so that it is easier to implement when you do get around to it.
      No panics given malformed data
      No panics on malformed unicode input
      On errors, the program outputs a message and nonzero exit status but does not panic.
      note: partial points may be awarded

7 - Integrated Error Handling with Testing (0.5pts)
      Different erroneous inputs return different kinds errors
      Have at least 10 different error cases across your program
      Tests verify that your parser rejects erroneous inputs for the right reason

8 - Implement version 1.2 of the protocol (parsing and aggregation) (2pts)
      Recursively parse sub-telegrams
      When graphing data, information should be accumulated (added together) from sub-telegrams (so if there is a toplevel electricity meter and a sub-electricity meter the total energy consumed and produced should be combined (usually by summing)). The Date of sub-telegrams does not matter, assume all data is produced at the same time.
      Parse Gas information when the g extension is found
      Make sure to parse and report the different gas models (G4, G5, G6) correctly
      note: partial points may be awarded

1-Verified using cargo tarpaulin