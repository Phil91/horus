use crate::{
	providers::base::catholicprovider::CatholicProvider,
	types::{countrycode::CountryCode, publicholiday::PublicHoliday, publicholidaytype::PublicHolidayType},
};

pub struct PolandHolidayProvider {}

impl PolandHolidayProvider {
	const COUNTRYCODE: CountryCode = CountryCode::PL;

	/// .
	///
	/// # Panics
	///
	/// Panics if .
	pub fn get_holidays(year: i32) -> Vec<PublicHoliday> {
		let mut holidays = vec![
			PublicHoliday::new_fixed(
				year,
				1,
				1,
				"Nowy Rok",
				"New Year's Day",
				Self::COUNTRYCODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				1,
				6,
				"Święto Trzech Króli",
				"Epiphany",
				Self::COUNTRYCODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			CatholicProvider::easter_sunday("Wielkanoc", year, Self::COUNTRYCODE),
			CatholicProvider::easter_monday("Drugi Dzień Wielkanocy", year, Self::COUNTRYCODE),
			PublicHoliday::new_fixed(
				year,
				5,
				1,
				"Święto Pracy",
				"May Day",
				Self::COUNTRYCODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				5,
				3,
				"Święto Narodowe Trzeciego Maja",
				"Constitution Day",
				Self::COUNTRYCODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			CatholicProvider::pentecost("Zielone Świątki", year, Self::COUNTRYCODE),
			CatholicProvider::corpus_christi("Boże Ciało", year, Self::COUNTRYCODE),
			PublicHoliday::new_fixed(
				year,
				8,
				15,
				"Wniebowzięcie Najświętszej Maryi Panny",
				"Assumption Day",
				Self::COUNTRYCODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				11,
				1,
				"Wszystkich Świętych",
				"All Saints' Day",
				Self::COUNTRYCODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				11,
				11,
				"Narodowe Święto Niepodległości",
				"Independence Day",
				Self::COUNTRYCODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				12,
				25,
				"Boże Narodzenie",
				"Christmas Day",
				Self::COUNTRYCODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				12,
				26,
				"Drugi Dzień Bożego Narodzenia",
				"St. Stephen's Day",
				Self::COUNTRYCODE,
				None,
				None,
				PublicHolidayType::Public,
			),
		];

		if year == 2018 {
			//100th anniversary
			holidays.push(PublicHoliday::new_fixed(
				year,
				11,
				12,
				"Narodowe Święto Niepodległości",
				"Independence Day",
				Self::COUNTRYCODE,
				None,
				None,
				PublicHolidayType::Public,
			));
		}
		holidays.sort_by(|a, b| a.date.cmp(&b.date));

		holidays
	}
}

#[cfg(test)]
mod tests {
	use super::PolandHolidayProvider;

	#[test]
	fn test_get_holidays() {
		let result = PolandHolidayProvider::get_holidays(2022);
		assert_eq!(result.len(), 13);
		assert_eq!(result.is_empty(), false);
		assert_eq!(result[5].local_name, "Święto Narodowe Trzeciego Maja");
	}
}
