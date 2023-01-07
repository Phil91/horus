use chrono::*;

use crate::types::{countrycode::CountryCode, publicholiday::PublicHoliday, publicholidaytype::PublicHolidayType};

pub struct OrthodoxProvider {}

impl OrthodoxProvider {
	pub(crate) fn get_orthodox_easter(year: i32) -> DateTime<Utc> {
		// credits https://gist.github.com/georgekosmidis/7f2cbabbd57ef879e95d990f0c356106#file-getorthodoxeaster-cs
		let a: i32 = year % 19;
		let b: i32 = year % 7;
		let c: i32 = year % 4;

		let d: i32 = (19 * a + 16) % 30;
		let e: i32 = (2 * c + 4 * b + 6 * d) % 7;
		let f: i32 = (19 * a + 16) % 30;

		let key: i32 = f + e + 3;
		let month: i32 = if key > 30 { 5 } else { 4 };
		let day: i32 = if key > 30 { key - 30 } else { key };

		Utc.with_ymd_and_hms(year, month as u32, day as u32, 0, 0, 0).unwrap()
	}

	#[allow(dead_code)]
	pub(crate) fn maundy_thursday(localname: &str, year: i32, countrycode: CountryCode) -> PublicHoliday {
		PublicHoliday::new_non_fixed(
			Self::get_orthodox_easter(year) - Duration::days(3),
			localname,
			"Maundy Thursday",
			countrycode,
			None,
			None,
			PublicHolidayType::Public,
		)
	}

	pub(crate) fn good_friday(localname: &str, year: i32, countrycode: CountryCode) -> PublicHoliday {
		PublicHoliday::new_non_fixed(
			Self::get_orthodox_easter(year) - Duration::days(2),
			localname,
			"Good Friday",
			countrycode,
			None,
			None,
			PublicHolidayType::Public,
		)
	}

	pub(crate) fn easter_sunday(localname: &str, year: i32, countrycode: CountryCode) -> PublicHoliday {
		PublicHoliday::new_non_fixed(
			Self::get_orthodox_easter(year),
			localname,
			"Easter Sunday",
			countrycode,
			None,
			None,
			PublicHolidayType::Public,
		)
	}

	pub(crate) fn easter_monday(localname: &str, year: i32, countrycode: CountryCode) -> PublicHoliday {
		PublicHoliday::new_non_fixed(
			Self::get_orthodox_easter(year) + Duration::days(1),
			localname,
			"Easter Monday",
			countrycode,
			None,
			None,
			PublicHolidayType::Public,
		)
	}

	#[allow(dead_code)]
	pub(crate) fn ascension_day(localname: &str, year: i32, countrycode: CountryCode) -> PublicHoliday {
		PublicHoliday::new_non_fixed(
			Self::get_orthodox_easter(year) + Duration::days(39),
			localname,
			"Ascension Day",
			countrycode,
			None,
			None,
			PublicHolidayType::Public,
		)
	}

	pub(crate) fn pentecost(localname: &str, year: i32, countrycode: CountryCode) -> PublicHoliday {
		PublicHoliday::new_non_fixed(
			Self::get_orthodox_easter(year) + Duration::days(49),
			localname,
			"Pentecost",
			countrycode,
			None,
			None,
			PublicHolidayType::Public,
		)
	}

	pub(crate) fn whit_monday(localname: &str, year: i32, countrycode: CountryCode) -> PublicHoliday {
		PublicHoliday::new_non_fixed(
			Self::get_orthodox_easter(year) + Duration::days(50),
			localname,
			"Whit Monday",
			countrycode,
			None,
			None,
			PublicHolidayType::Public,
		)
	}

	#[allow(dead_code)]
	pub(crate) fn corpus_christi(localname: &str, year: i32, countrycode: CountryCode) -> PublicHoliday {
		PublicHoliday::new_non_fixed(
			Self::get_orthodox_easter(year) + Duration::days(60),
			localname,
			"Corpus Christi",
			countrycode,
			None,
			None,
			PublicHolidayType::Public,
		)
	}
}

#[cfg(test)]
mod tests {
	use chrono::Datelike;

	use crate::types::countrycode::CountryCode;

	use super::OrthodoxProvider;

	#[test]
	fn test_get_orthodox_easter() {
		let result = OrthodoxProvider::get_orthodox_easter(2022);
		assert_eq!(result.day(), 24);
		assert_eq!(result.month(), 4);
	}

	#[test]
	fn test_maundy_thursday() {
		let result = OrthodoxProvider::maundy_thursday("test", 2022, CountryCode::DE);
		assert_eq!(result.to_string(), "2022-04-21T00:00:00Z Maundy Thursday");
	}

	#[test]
	fn test_good_friday() {
		let result = OrthodoxProvider::good_friday("test", 2022, CountryCode::DE);
		assert_eq!(result.to_string(), "2022-04-22T00:00:00Z Good Friday");
	}

	#[test]
	fn test_easter_sunday() {
		let result = OrthodoxProvider::easter_sunday("test", 2022, CountryCode::DE);
		assert_eq!(result.to_string(), "2022-04-24T00:00:00Z Easter Sunday");
	}

	#[test]
	fn test_easter_monday() {
		let result = OrthodoxProvider::easter_monday("test", 2022, CountryCode::DE);
		assert_eq!(result.to_string(), "2022-04-25T00:00:00Z Easter Monday");
	}

	#[test]
	fn test_ascension_day() {
		let result = OrthodoxProvider::ascension_day("test", 2022, CountryCode::DE);
		assert_eq!(result.to_string(), "2022-06-02T00:00:00Z Ascension Day");
	}

	#[test]
	fn test_pentecost() {
		let result = OrthodoxProvider::pentecost("test", 2022, CountryCode::DE);
		assert_eq!(result.to_string(), "2022-06-12T00:00:00Z Pentecost");
	}

	#[test]
	fn test_whit_monday() {
		let result = OrthodoxProvider::whit_monday("test", 2022, CountryCode::DE);
		assert_eq!(result.to_string(), "2022-06-13T00:00:00Z Whit Monday");
	}

	#[test]
	fn test_corpus_christi() {
		let result = OrthodoxProvider::corpus_christi("test", 2022, CountryCode::DE);
		assert_eq!(result.to_string(), "2022-06-23T00:00:00Z Corpus Christi");
	}
}
