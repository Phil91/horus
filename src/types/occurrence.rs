#[derive(Debug, PartialEq, Eq)]
pub enum Occurrence {
	// First
	First = 1,
	// Second
	Second = 2,
	// Third
	Third = 3,
	// Fourth
	Fourth = 4,
	// Sometimes a month has a weekday 5 days
	Fifth = 5,
}
