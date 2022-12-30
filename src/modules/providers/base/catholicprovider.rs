extern crate chrono;
pub use chrono::*;

pub struct CatholicProvider {}

use std::{
	collections::HashMap,
	sync::{Arc, Mutex},
};

use crate::modules::types::{
	countrycode::CountryCode, publicholiday::PublicHoliday, publicholidaytype::PublicHolidayType,
};

lazy_static! {
	static ref CATHOLIC_CACHE: Arc<Mutex<HashMap<i32, DateTime<Utc>>>> = Arc::new(Mutex::new(HashMap::new()));
}

impl CatholicProvider {
	fn get_easter_sunday(year: i32) -> DateTime<Utc> {
		//should be
		//Easter Monday  28 Mar 2005  17 Apr 2006  9 Apr 2007  24 Mar 2008

		//Oudin's Algorithm - http://www.smart.net/~mmontes/oudin.html
		let mut cache = CATHOLIC_CACHE.lock().unwrap();
		if !cache.contains_key(&year) {
			let g = year % 19;
			let c = year / 100;
			let h = (c - c / 4 - (8 * c + 13) / 25 + 19 * g + 15) % 30;
			let i = h - (h / 28) * (1 - (h / 28) * (29 / (h + 1)) * ((21 - g) / 11));

			let mut day: u32 = (i - ((year + (year / 4) + i + 2 - c + (c / 4)) % 7) + 28) as u32;
			let mut month = 3;

			if day > 31 {
				month += 1;
				day -= 31;
			}

			let datetime = Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap();
			cache.insert(year, datetime);
		}

		cache[&year]
	}

	pub(crate) fn advent_sunday(year: i32) -> DateTime<Utc> {
		let christmas_date = Utc.with_ymd_and_hms(year, 12, 24, 0, 0, 0).unwrap();
		let days_to_advent: i64 = (21 + christmas_date.weekday().num_days_from_sunday()).into();

		return christmas_date - Duration::days(days_to_advent);
	}

	pub(crate) fn maundy_thursday(local_name: &str, year: i32, country_code: CountryCode) -> PublicHoliday {
		return PublicHoliday::new_non_fixed(
			Self::get_easter_sunday(year) - Duration::days(3),
			local_name.to_string(),
			"Maundy Thursday".to_string(),
			country_code,
			None,
			None,
			PublicHolidayType::Public,
		);
	}

	pub(crate) fn good_friday(local_name: &str, year: i32, country_code: CountryCode) -> PublicHoliday {
		return PublicHoliday::new_non_fixed(
			Self::get_easter_sunday(year) - Duration::days(2),
			local_name.to_string(),
			"Good Friday".to_string(),
			country_code,
			None,
			None,
			PublicHolidayType::Public,
		);
	}

	pub(crate) fn easter_sunday(local_name: &str, year: i32, country_code: CountryCode) -> PublicHoliday {
		return PublicHoliday::new_non_fixed(
			Self::get_easter_sunday(year),
			local_name.to_string(),
			"Easter Sunday".to_string(),
			country_code,
			None,
			None,
			PublicHolidayType::Public,
		);
	}

	pub(crate) fn easter_monday(localname: &str, year: i32, countrycode: CountryCode) -> PublicHoliday {
		return PublicHoliday::new_non_fixed(
			Self::get_easter_sunday(year) + Duration::days(1),
			localname.to_string(),
			"Easter Monday".to_string(),
			countrycode,
			None,
			None,
			PublicHolidayType::Public,
		);
	}

	pub(crate) fn ascension_day(localname: &str, year: i32, countrycode: CountryCode) -> PublicHoliday {
		return PublicHoliday::new_non_fixed(
			Self::get_easter_sunday(year) + Duration::days(39),
			localname.to_string(),
			"Ascension Day".to_string(),
			countrycode,
			None,
			None,
			PublicHolidayType::Public,
		);
	}

	pub(crate) fn pentecost(localname: &str, year: i32, countrycode: CountryCode) -> PublicHoliday {
		return PublicHoliday::new_non_fixed(
			Self::get_easter_sunday(year) + Duration::days(49),
			localname.to_string(),
			"Pentecost".to_string(),
			countrycode,
			None,
			None,
			PublicHolidayType::Public,
		);
	}

	pub(crate) fn whit_monday(localname: &str, year: i32, countrycode: CountryCode) -> PublicHoliday {
		return PublicHoliday::new_non_fixed(
			Self::get_easter_sunday(year) + Duration::days(50),
			localname.to_string(),
			"Whit Monday".to_string(),
			countrycode,
			None,
			None,
			PublicHolidayType::Public,
		);
	}

	pub(crate) fn corpus_christi(localname: &str, year: i32, countrycode: CountryCode) -> PublicHoliday {
		return PublicHoliday::new_non_fixed(
			Self::get_easter_sunday(year) + Duration::days(60),
			localname.to_string(),
			"Corpus Christi".to_string(),
			countrycode,
			None,
			None,
			PublicHolidayType::Public,
		);
	}
}

#[cfg(test)]
mod tests {
	use chrono::Datelike;

	use crate::modules::types::countrycode::CountryCode;

	use super::CatholicProvider;

	#[test]
	fn test_get_easter_sunday() {
		let result = CatholicProvider::get_easter_sunday(2022);
		assert_eq!(result.day(), 17);
		assert_eq!(result.month(), 4);
	}

	#[test]
	fn test_advent_sunday() {
		let result = CatholicProvider::advent_sunday(2022);
		assert_eq!(result.day(), 27);
		assert_eq!(result.month(), 11);
	}

	#[test]
	fn test_maundy_thursday() {
		let result = CatholicProvider::maundy_thursday("test", 2022, CountryCode::DE);
		assert_eq!(result.to_string(), "2022-04-14T00:00:00Z Maundy Thursday");
	}

	#[test]
	fn test_good_friday() {
		let result = CatholicProvider::good_friday("test", 2022, CountryCode::DE);
		assert_eq!(result.to_string(), "2022-04-15T00:00:00Z Good Friday");
	}

	#[test]
	fn test_easter_sunday() {
		let result = CatholicProvider::easter_sunday("test", 2022, CountryCode::DE);
		assert_eq!(result.to_string(), "2022-04-17T00:00:00Z Easter Sunday");
	}

	#[test]
	fn test_easter_monday() {
		let result = CatholicProvider::easter_monday("test", 2022, CountryCode::DE);
		assert_eq!(result.to_string(), "2022-04-18T00:00:00Z Easter Monday");
	}

	#[test]
	fn test_ascension_day() {
		let result = CatholicProvider::ascension_day("test", 2022, CountryCode::DE);
		assert_eq!(result.to_string(), "2022-05-26T00:00:00Z Ascension Day");
	}

	#[test]
	fn test_pentecost() {
		let result = CatholicProvider::pentecost("test", 2022, CountryCode::DE);
		assert_eq!(result.to_string(), "2022-06-05T00:00:00Z Pentecost");
	}

	#[test]
	fn test_whit_monday() {
		let result = CatholicProvider::whit_monday("test", 2022, CountryCode::DE);
		assert_eq!(result.to_string(), "2022-06-06T00:00:00Z Whit Monday");
	}

	#[test]
	fn test_corpus_christi() {
		let result = CatholicProvider::corpus_christi("test", 2022, CountryCode::DE);
		assert_eq!(result.to_string(), "2022-06-16T00:00:00Z Corpus Christi");
	}
}
