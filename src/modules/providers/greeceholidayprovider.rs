use chrono::Duration;

use crate::modules::types::{
	countrycode::CountryCode, publicholiday::PublicHoliday, publicholidaytype::PublicHolidayType,
};

use super::base::orthodoxprovider::OrthodoxProvider;

pub struct GreeceHolidayProvider {}

impl GreeceHolidayProvider {
	const COUNTRY_CODE: CountryCode = CountryCode::GR;

	pub fn get_holidays(year: i32) -> Vec<PublicHoliday> {
		let easter_sunday = OrthodoxProvider::get_orthodox_easter(year);

		let mut holidays = Vec::new();
		holidays.push(PublicHoliday::new_fixed(
			year,
			1,
			1,
			"Πρωτοχρονιά".to_string(),
			"New Year's Day".to_string(),
			Self::COUNTRY_CODE,
			None,
			None,
			PublicHolidayType::Public,
		));
		holidays.push(PublicHoliday::new_fixed(
			year,
			1,
			6,
			"Θεοφάνεια".to_string(),
			"Epiphany".to_string(),
			CountryCode::GR,
			None,
			None,
			PublicHolidayType::Public,
		));
		holidays.push(PublicHoliday::new_non_fixed(
			easter_sunday - Duration::days(48),
			"Καθαρά Δευτέρα".to_string(),
			"Clean Monday".to_string(),
			Self::COUNTRY_CODE,
			None,
			None,
			PublicHolidayType::Public,
		));
		holidays.push(PublicHoliday::new_fixed(
			year,
			3,
			25,
			"Ευαγγελισμός της Θεοτόκου".to_string(),
			"Annunciation".to_string(),
			Self::COUNTRY_CODE,
			None,
			None,
			PublicHolidayType::Public,
		));
		holidays.push(PublicHoliday::new_fixed(
			year,
			3,
			25,
			"Εικοστή Πέμπτη Μαρτίου".to_string(),
			"Independence Day".to_string(),
			Self::COUNTRY_CODE,
			None,
			None,
			PublicHolidayType::Public,
		));
		holidays.push(OrthodoxProvider::good_friday(
			"Μεγάλη Παρασκευή",
			year,
			Self::COUNTRY_CODE,
		));
		holidays.push(OrthodoxProvider::easter_sunday(
			"Κυριακή του Πάσχα",
			year,
			Self::COUNTRY_CODE,
		));
		holidays.push(OrthodoxProvider::easter_monday(
			"Δευτέρα του Πάσχα",
			year,
			Self::COUNTRY_CODE,
		));
		holidays.push(PublicHoliday::new_fixed(
			year,
			5,
			1,
			"Εργατική Πρωτομαγιά".to_string(),
			"Labour Day".to_string(),
			Self::COUNTRY_CODE,
			None,
			None,
			PublicHolidayType::Public,
		));
		holidays.push(OrthodoxProvider::pentecost("Πεντηκοστή'", year, Self::COUNTRY_CODE));
		holidays.push(OrthodoxProvider::whit_monday(
			"Δευτέρα Πεντηκοστής",
			year,
			Self::COUNTRY_CODE,
		));
		holidays.push(PublicHoliday::new_fixed(
			year,
			8,
			15,
			"Κοίμηση της Θεοτόκου".to_string(),
			"Assumption Day".to_string(),
			Self::COUNTRY_CODE,
			None,
			None,
			PublicHolidayType::Public,
		));
		holidays.push(PublicHoliday::new_fixed(
			year,
			10,
			28,
			"Το Όχι".to_string(),
			"Ochi Day".to_string(),
			Self::COUNTRY_CODE,
			None,
			None,
			PublicHolidayType::Public,
		));
		holidays.push(PublicHoliday::new_fixed(
			year,
			12,
			25,
			"Χριστούγεννα".to_string(),
			"Christmas Day".to_string(),
			Self::COUNTRY_CODE,
			None,
			None,
			PublicHolidayType::Public,
		));
		holidays.push(PublicHoliday::new_fixed(
			year,
			12,
			26,
			"Σύναξις Υπεραγίας Θεοτόκου Μαρίας".to_string(),
			"St. Stephen's Day".to_string(),
			Self::COUNTRY_CODE,
			None,
			None,
			PublicHolidayType::Public,
		));

		holidays.sort_by(|a, b| a.date.cmp(&b.date));

		return holidays;
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
		assert_eq!(result[5].local_name, "Μεγάλη Παρασκευή".to_string());
	}
}
