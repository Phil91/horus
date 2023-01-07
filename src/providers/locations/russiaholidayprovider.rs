use crate::types::{countrycode::CountryCode, publicholiday::PublicHoliday, publicholidaytype::PublicHolidayType};

pub struct RussiaHolidayProvider {}

impl RussiaHolidayProvider {
	const COUNTRYCODE: CountryCode = CountryCode::RU;

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
				"Новый год",
				"New Year's Day",
				Self::COUNTRYCODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				1,
				2,
				"Новогодние каникулы",
				"New Year holiday",
				Self::COUNTRYCODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				1,
				3,
				"Новогодние каникулы",
				"New Year holiday",
				Self::COUNTRYCODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				1,
				4,
				"Новогодние каникулы",
				"New Year holiday",
				Self::COUNTRYCODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				1,
				5,
				"Новогодние каникулы",
				"New Year holiday",
				Self::COUNTRYCODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				1,
				6,
				"Новогодние каникулы",
				"New Year holiday",
				Self::COUNTRYCODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				1,
				7,
				"Рождество Христово",
				"Orthodox Christmas Day",
				Self::COUNTRYCODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				2,
				23,
				"День защитника Отечества",
				"Defender of the Fatherland Day",
				Self::COUNTRYCODE,
				Some(1918),
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				3,
				8,
				"Международный женский день",
				"International Women's Day",
				Self::COUNTRYCODE,
				Some(1913),
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				5,
				1,
				"День труда",
				"Labour Day",
				Self::COUNTRYCODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				5,
				9,
				"День Победы",
				"Victory Day",
				Self::COUNTRYCODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				6,
				12,
				"День России",
				"Russia Day",
				Self::COUNTRYCODE,
				Some(2002),
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				11,
				4,
				"День народного единства",
				"Unity Day",
				Self::COUNTRYCODE,
				Some(2005),
				None,
				PublicHolidayType::Public,
			),
		];

		holidays.sort_by(|a, b| a.date.cmp(&b.date));

		holidays
	}
}

#[cfg(test)]
mod tests {
	use super::RussiaHolidayProvider;

	#[test]
	fn test_get_holidays() {
		let result = RussiaHolidayProvider::get_holidays(2022);
		assert_eq!(result.len(), 13);
		assert_eq!(result.is_empty(), false);
		assert_eq!(result[5].local_name, "Новогодние каникулы");
	}
}
