use std::collections::HashMap;

use chrono::{Duration, TimeZone, Utc};

use crate::modules::types::{
	countrycode::CountryCode, publicholiday::PublicHoliday, publicholidaytype::PublicHolidayType,
};

use super::base::catholicprovider::CatholicProvider;

pub struct GermanHolidayProvider {}

impl GermanHolidayProvider {
	const COUNTRYCODE: CountryCode = CountryCode::DE;

	/// Gets a hashmap of the available counties of germany
	pub fn get_counties() -> HashMap<String, String> {
		return HashMap::from([
			("DE-BW".to_string(), "Baden-Württemberg".to_string()),
			("DE-BY".to_string(), "Bayern".to_string()),
			("DE-BE".to_string(), "Berlin".to_string()),
			("DE-BB".to_string(), "Brandenburg".to_string()),
			("DE-HB".to_string(), "Bremen".to_string()),
			("DE-HH".to_string(), "Hamburg".to_string()),
			("DE-HE".to_string(), "Hessen".to_string()),
			("DE-MV".to_string(), "Mecklenburg-Vorpommern".to_string()),
			("DE-NI".to_string(), "Niedersachsen".to_string()),
			("DE-NW".to_string(), "Nordrhein-Westfalen".to_string()),
			("DE-RP".to_string(), "Rheinland-Pfalz".to_string()),
			("DE-SL".to_string(), "Saarland".to_string()),
			("DE-SN".to_string(), "Sachsen".to_string()),
			("DE-ST".to_string(), "Sachsen-Anhalt".to_string()),
			("DE-SH".to_string(), "Schleswig-Holstein".to_string()),
			("DE-TH".to_string(), "Thüringen".to_string()),
		]);
	}

	/// .
	///
	/// # Panics
	///
	/// Panics if .
	pub fn get_holidays(year: i32) -> Vec<PublicHoliday> {
		let mut holidays = Vec::new();
		holidays.push(PublicHoliday::new_fixed(
			year,
			1,
			1,
			"Neujahr".to_string(),
			"New Year's Day".to_string(),
			Self::COUNTRYCODE,
			Some(1967),
			None,
			PublicHolidayType::Public,
		));
		holidays.push(PublicHoliday::new_fixed(
			year,
			1,
			6,
			"Heilige Drei Könige".to_string(),
			"Epiphany".to_string(),
			Self::COUNTRYCODE,
			Some(1967),
			Some(vec!["DE-BW".to_string(), "DE-BY".to_string(), "DE-ST".to_string()]),
			PublicHolidayType::Public,
		));
		holidays.push(PublicHoliday::new_fixed(
			year,
			3,
			8,
			"Internationaler Frauentag".to_string(),
			"International Women's Day".to_string(),
			Self::COUNTRYCODE,
			Some(2019),
			Some(vec!["DE-BE".to_string()]),
			PublicHolidayType::Public,
		));
		holidays.push(CatholicProvider::good_friday("Karfreitag", year, Self::COUNTRYCODE));
		holidays.push(CatholicProvider::easter_sunday("Ostersonntag", 2022, Self::COUNTRYCODE));
		let holiday_length = &holidays.len();
		if let Some(holiday) = holidays.get_mut(*holiday_length) {
			holiday.set_counties(vec!["DE-BB".to_string(), "DE-HE".to_string()]);
		}
		holidays.push(CatholicProvider::easter_monday("Ostermontag", year, Self::COUNTRYCODE));
		let holiday_length = &holidays.len();
		if let Some(holiday) = holidays.get_mut(*holiday_length) {
			holiday.set_launch_year(1642);
		}
		holidays.push(PublicHoliday::new_fixed(
			year,
			5,
			1,
			"Tag der Arbeit".to_string(),
			"Labour Day".to_string(),
			Self::COUNTRYCODE,
			None,
			None,
			PublicHolidayType::Public,
		));
		holidays.push(CatholicProvider::ascension_day(
			"Christi Himmelfahrt",
			year,
			Self::COUNTRYCODE,
		));
		holidays.push(CatholicProvider::pentecost("Pfingstsonntag", year, Self::COUNTRYCODE));
		let holiday_length = &holidays.len();
		if let Some(holiday) = holidays.get_mut(*holiday_length) {
			holiday.set_counties(vec!["DE-BB".to_string(), "DE-HE".to_string()]);
		}
		holidays.push(CatholicProvider::whit_monday("Pfingstmontag", year, Self::COUNTRYCODE));
		holidays.push(CatholicProvider::corpus_christi(
			"Fronleichnam",
			year,
			Self::COUNTRYCODE,
		));
		let holiday_length = &holidays.len();
		if let Some(holiday) = holidays.get_mut(*holiday_length) {
			holiday.set_counties(vec![
				"DE-BW".to_string(),
				"DE-BY".to_string(),
				"DE-HE".to_string(),
				"DE-NW".to_string(),
				"DE-RP".to_string(),
				"DE-SL".to_string(),
			]);
		}
		holidays.push(PublicHoliday::new_fixed(
			year,
			8,
			15,
			"Mariä Himmelfahrt".to_string(),
			"Assumption Day".to_string(),
			Self::COUNTRYCODE,
			None,
			Some(vec!["DE-SL".to_string()]),
			PublicHolidayType::Public,
		));
		holidays.push(PublicHoliday::new_fixed(
			year,
			9,
			20,
			"Weltkindertag".to_string(),
			"World Children's Day".to_string(),
			Self::COUNTRYCODE,
			Some(2019),
			Some(vec!["DE-TH".to_string()]),
			PublicHolidayType::Public,
		));
		holidays.push(PublicHoliday::new_fixed(
			year,
			10,
			3,
			"Tag der Deutschen Einheit".to_string(),
			"German Unity Day".to_string(),
			Self::COUNTRYCODE,
			None,
			None,
			PublicHolidayType::Public,
		));
		holidays.push(PublicHoliday::new_fixed(
			year,
			11,
			1,
			"Allerheiligen".to_string(),
			"All Saints' Day".to_string(),
			Self::COUNTRYCODE,
			None,
			Some(vec![
				"DE-BW".to_string(),
				"DE-BY".to_string(),
				"DE-NW".to_string(),
				"DE-RP".to_string(),
				"DE-SL".to_string(),
			]),
			PublicHolidayType::Public,
		));
		holidays.push(PublicHoliday::new_fixed(
			year,
			12,
			25,
			"Erster Weihnachtstag".to_string(),
			"Christmas Day".to_string(),
			Self::COUNTRYCODE,
			None,
			None,
			PublicHolidayType::Public,
		));
		holidays.push(PublicHoliday::new_fixed(
			year,
			12,
			26,
			"Zweiter Weihnachtstag".to_string(),
			"St. Stephen's Day".to_string(),
			Self::COUNTRYCODE,
			None,
			None,
			PublicHolidayType::Public,
		));

		let prayer_day = Self::get_prayer_day(year, Self::COUNTRYCODE);
		if prayer_day.is_some() {
			holidays.push(prayer_day.unwrap());
		}

		let liberation_day = Self::get_liberation_day(year, Self::COUNTRYCODE);
		if liberation_day.is_some() {
			holidays.push(liberation_day.unwrap());
		}

		holidays.push(Self::get_reformation_day(year, Self::COUNTRYCODE));

		holidays.sort_by(|a, b| a.date.cmp(&b.date));

		return holidays;
	}

	fn get_reformation_day(year: i32, country_code: CountryCode) -> PublicHoliday {
		let local_name = "Reformationstag".to_string();
		let english_name = "Reformation Day".to_string();

		if year == 2017 {
			//In commemoration of the 500th anniversary of the beginning of the Reformation, it was unique as a whole German holiday
			return PublicHoliday::new_fixed(
				year,
				10,
				31,
				local_name,
				english_name,
				country_code,
				None,
				None,
				PublicHolidayType::Public,
			);
		}

		let mut counties = vec![
			"DE-BB".to_string(),
			"DE-MV".to_string(),
			"DE-SN".to_string(),
			"DE-ST".to_string(),
			"DE-TH".to_string(),
		];

		if year >= 2018 {
			counties.extend(vec![
				"DE-HB".to_string(),
				"DE-HH".to_string(),
				"DE-NI".to_string(),
				"DE-SH".to_string(),
			]);
		}

		PublicHoliday::new_fixed(
			year,
			10,
			31,
			local_name,
			english_name,
			country_code,
			None,
			Some(counties),
			PublicHolidayType::Public,
		)
	}

	fn get_prayer_day(year: i32, country_code: CountryCode) -> Option<PublicHoliday> {
		let day_of_prayer = CatholicProvider::advent_sunday(year) - Duration::days(11);
		let local_name = "Buß- und Bettag".to_string();
		let english_name = "Repentance and Prayer Day".to_string();

		if year >= 1934 && year < 1939 {
			Some(PublicHoliday::new_non_fixed(
				day_of_prayer,
				local_name,
				english_name,
				country_code,
				None,
				None,
				PublicHolidayType::Public,
			))
		} else if year >= 1945 && year <= 1980 {
			Some(PublicHoliday::new_non_fixed(
				day_of_prayer,
				local_name,
				english_name,
				country_code,
				None,
				Some(vec![
					"DE-BW".to_string(),
					"DE-BE".to_string(),
					"DE-HB".to_string(),
					"DE-HH".to_string(),
					"DE-HE".to_string(),
					"DE-NI".to_string(),
					"DE-NW".to_string(),
					"DE-RP".to_string(),
					"DE-SL".to_string(),
					"DE-SH".to_string(),
				]),
				PublicHolidayType::Public,
			))
		} else if year >= 1981 && year <= 1989 {
			Some(PublicHoliday::new_non_fixed(
				day_of_prayer,
				local_name,
				english_name,
				country_code,
				None,
				Some(vec![
					"DE-BW".to_string(),
					"DE-BY".to_string(),
					"DE-BE".to_string(),
					"DE-HB".to_string(),
					"DE-HH".to_string(),
					"DE-HE".to_string(),
					"DE-NI".to_string(),
					"DE-NW".to_string(),
					"DE-RP".to_string(),
					"DE-SL".to_string(),
					"DE-SH".to_string(),
				]),
				PublicHolidayType::Public,
			))
		} else if year >= 1990 && year <= 1994 {
			Some(PublicHoliday::new_non_fixed(
				day_of_prayer,
				local_name,
				english_name,
				country_code,
				None,
				None,
				PublicHolidayType::Public,
			))
		} else if year >= 1995 {
			Some(PublicHoliday::new_non_fixed(
				day_of_prayer,
				local_name,
				english_name,
				country_code,
				None,
				Some(vec!["DE-SN".to_string()]),
				PublicHolidayType::Public,
			))
		} else {
			None
		}
	}

	fn get_liberation_day(year: i32, country_code: CountryCode) -> Option<PublicHoliday> {
		let day_of_prayer = Utc.with_ymd_and_hms(2020, 5, 8, 0, 0, 0).unwrap();
		let local_name = "Tag der Befreiung".to_string();
		let english_name = "Liberation Day".to_string();

		if year == 2020 {
			Some(PublicHoliday::new_non_fixed(
				day_of_prayer,
				local_name,
				english_name,
				country_code,
				None,
				Some(vec!["DE-BE".to_string()]),
				PublicHolidayType::Public,
			))
		} else {
			None
		}
	}
}

#[cfg(test)]
mod tests {
	use super::GermanHolidayProvider;

	#[test]
	fn test_get_holidays() {
		let result = GermanHolidayProvider::get_holidays(2022);
		assert_eq!(result.len(), 19);
		assert_eq!(result.is_empty(), false);
		assert_eq!(result[5].local_name, "Ostermontag".to_string());
	}

	#[test]
	fn test_get_counties() {
		let result = GermanHolidayProvider::get_counties();
		assert_eq!(result.len(), 16);
	}
}
