use chrono::Duration;

use crate::{
	providers::base::orthodoxprovider::OrthodoxProvider,
	types::{countrycode::CountryCode, publicholiday::PublicHoliday, publicholidaytype::PublicHolidayType},
};

pub struct GreeceHolidayProvider {}

impl GreeceHolidayProvider {
	const COUNTRY_CODE: CountryCode = CountryCode::GR;

	pub fn get_holidays(year: i32) -> Vec<PublicHoliday> {
		let easter_sunday = OrthodoxProvider::get_orthodox_easter(year);

		let mut holidays = vec![
			PublicHoliday::new_fixed(
				year,
				1,
				1,
				"Πρωτοχρονιά",
				"New Year's Day",
				Self::COUNTRY_CODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				1,
				6,
				"Θεοφάνεια",
				"Epiphany",
				CountryCode::GR,
				None,
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_non_fixed(
				easter_sunday - Duration::days(48),
				"Καθαρά Δευτέρα",
				"Clean Monday",
				Self::COUNTRY_CODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				3,
				25,
				"Ευαγγελισμός της Θεοτόκου",
				"Annunciation",
				Self::COUNTRY_CODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				3,
				25,
				"Εικοστή Πέμπτη Μαρτίου",
				"Independence Day",
				Self::COUNTRY_CODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			OrthodoxProvider::good_friday("Μεγάλη Παρασκευή", year, Self::COUNTRY_CODE),
			OrthodoxProvider::easter_sunday("Κυριακή του Πάσχα", year, Self::COUNTRY_CODE),
			OrthodoxProvider::easter_monday("Δευτέρα του Πάσχα", year, Self::COUNTRY_CODE),
			PublicHoliday::new_fixed(
				year,
				5,
				1,
				"Εργατική Πρωτομαγιά",
				"Labour Day",
				Self::COUNTRY_CODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			OrthodoxProvider::pentecost("Πεντηκοστή'", year, Self::COUNTRY_CODE),
			OrthodoxProvider::whit_monday("Δευτέρα Πεντηκοστής", year, Self::COUNTRY_CODE),
			PublicHoliday::new_fixed(
				year,
				8,
				15,
				"Κοίμηση της Θεοτόκου",
				"Assumption Day",
				Self::COUNTRY_CODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				10,
				28,
				"Το Όχι",
				"Ochi Day",
				Self::COUNTRY_CODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				12,
				25,
				"Χριστούγεννα",
				"Christmas Day",
				Self::COUNTRY_CODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				12,
				26,
				"Σύναξις Υπεραγίας Θεοτόκου Μαρίας",
				"St. Stephen's Day",
				Self::COUNTRY_CODE,
				None,
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
	use super::GreeceHolidayProvider;

	#[test]
	fn test_get_holidays() {
		let result = GreeceHolidayProvider::get_holidays(2022);
		assert_eq!(result.len(), 15);
		assert_eq!(result.is_empty(), false);
		assert_eq!(result[5].local_name, "Μεγάλη Παρασκευή");
	}
}
