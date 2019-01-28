# Desired API

```rust
/// Describes how to parse the Radiotap data, this should work for all namespace types
struct Radiotap;

impl Radiotap {
    // Parse all fields
    fn new() -> Self;
    // Parse no fields
    fn empty() -> Self;
    // Add fields
    fn include(self) -> &mut Self;
    // Exclude fields
    fn exclude(self) -> &mut Self;
    // Actually do the parsing
    fn parse(&self, data: &[u8]) -> Result<(RadiotapData, &[u8])>;
}


/// Contains only Radiotap namespace data
struct RadiotapData;

/// Can iterate over all namespace types
struct RadiotapIterator;





```
