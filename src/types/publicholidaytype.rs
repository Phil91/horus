#[derive(Debug, PartialEq, Eq)]
pub enum PublicHolidayType {
	// Public holiday
	Public = 1,
	// Bank holiday, banks and offices are closed
	Bank = 2,
	// School holiday, schools are closed
	School = 4,
	// Authorities are closed
	Authorities = 8,
	// Majority of people take a day off
	Optional = 16,
	// Optional festivity, no paid day off
	Observance = 32,
}
