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