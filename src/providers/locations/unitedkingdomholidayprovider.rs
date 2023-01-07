use std::collections::HashMap;

use chrono::{Datelike, TimeZone, Utc, Weekday};

use crate::{
	datesystem::DateSystem,
	providers::base::catholicprovider::CatholicProvider,
	types::{
		countrycode::CountryCode, occurrence::Occurrence, publicholiday::PublicHoliday,
		publicholidaytype::PublicHolidayType,
	},
};

pub struct UnitedKingdomHolidayProvider {}

impl UnitedKingdomHolidayProvider {
	const COUNTRYCODE: CountryCode = CountryCode::GB;

	/// Gets a hashmap of the available counties of the united kingdom
	pub fn get_counties() -> HashMap<String, String> {
		HashMap::from([
			("GB-NIR".to_string(), "Northern Ireland".to_string()),
			("GB-SCT".to_string(), "Scotland".to_string()),
			("GB-ENG".to_string(), "England".to_string()),
			("GB-WLS".to_string(), "Wales".to_string()),
		])
	}

	/// .
	///
	/// # Panics
	///
	/// Panics if .
	pub fn get_holidays(year: i32) -> Vec<PublicHoliday> {
		let first_monday_in_august = DateSystem::find_day(year, 8, Weekday::Mon, Occurrence::First);
		let last_monday_in_august = DateSystem::find_last_day(year, 8, Weekday::Mon);

		let mut holidays = Vec::new();

		let new_years_day = Utc.with_ymd_and_hms(year, 1, 1, 0, 0, 0).unwrap();
		if new_years_day.weekday() == Weekday::Sat || new_years_day.weekday() == Weekday::Sun {
			let new_year_day_monday = DateSystem::find_day(year, 1, Weekday::Mon, Occurrence::First);
			let new_year_day_tuesday = DateSystem::find_day(year, 1, Weekday::Tue, Occurrence::First);

			holidays.push(PublicHoliday::new_non_fixed(
				new_years_day,
				"New Year's Day",
				"New Year's Day",
				Self::COUNTRYCODE,
				None,
				Some(vec!["GB-NIR".to_string()]),
				PublicHolidayType::Public,
			));

			holidays.push(PublicHoliday::new_non_fixed(
				new_year_day_monday.unwrap(),
				"New Year's Day",
				"New Year's Day",
				Self::COUNTRYCODE,
				None,
				Some(vec!["GB-ENG".to_string(), "GB-WLS".to_string()]),
				PublicHolidayType::Public,
			));
			holidays.push(PublicHoliday::new_non_fixed(
				new_year_day_tuesday.unwrap(),
				"New Year's Day",
				"New Year's Day",
				Self::COUNTRYCODE,
				None,
				Some(vec!["GB-SCT".to_string()]),
				PublicHolidayType::Public,
			));
		} else {
			holidays.push(PublicHoliday::new_non_fixed(
				new_years_day,
				"New Year's Day",
				"New Year's Day",
				Self::COUNTRYCODE,
				None,
				None,
				PublicHolidayType::Public,
			));
		}

		let new_year_day2 = DateSystem::shift(Utc.with_ymd_and_hms(year, 1, 2, 0, 0, 0).unwrap(), 2, 1, None);
		holidays.push(PublicHoliday::new_non_fixed(
			new_year_day2,
			"New Year's Day",
			"New Year's Day",
			Self::COUNTRYCODE,
			None,
			Some(vec!["GB-SCT".to_string()]),
			PublicHolidayType::Public,
		));

		holidays.push(PublicHoliday::new_fixed(
			year,
			3,
			17,
			"Saint Patrick's Day",
			"Saint Patrick's Day",
			Self::COUNTRYCODE,
			None,
			Some(vec!["GB-NIR".to_string()]),
			PublicHolidayType::Public,
		));
		holidays.push(CatholicProvider::good_friday("Good Friday", year, Self::COUNTRYCODE));

		holidays.push(CatholicProvider::easter_monday(
			"Easter Monday",
			year,
			Self::COUNTRYCODE,
		));
		let holiday_length = &holidays.len();
		if let Some(holiday) = holidays.get_mut(*holiday_length) {
			holiday.set_counties(vec!["GB-ENG".to_string(), "GB-WLS".to_string(), "GB-NIR".to_string()]);
		}

		holidays.push(PublicHoliday::new_fixed(
			year,
			7,
			12,
			"Battle of the Boyne",
			"Battle of the Boyne",
			Self::COUNTRYCODE,
			None,
			Some(vec!["GB-NIR".to_string()]),
			PublicHolidayType::Public,
		));
		holidays.push(PublicHoliday::new_fixed(
			year,
			11,
			30,
			"Saint Andrew's Day",
			"Saint Andrew's Day",
			Self::COUNTRYCODE,
			None,
			Some(vec!["GB-SCT".to_string()]),
			PublicHolidayType::Public,
		));

		if let Some(first_monday_in_august) = first_monday_in_august {
			holidays.push(PublicHoliday::new_non_fixed(
				first_monday_in_august,
				"Summer Bank Holiday",
				"Summer Bank Holiday",
				Self::COUNTRYCODE,
				Some(1971),
				Some(vec!["GB-SCT".to_string()]),
				PublicHolidayType::Public,
			));
		}
		if let Some(last_monday_in_august) = last_monday_in_august {
			holidays.push(PublicHoliday::new_non_fixed(
				last_monday_in_august,
				"Summer Bank Holiday",
				"Summer Bank Holiday",
				Self::COUNTRYCODE,
				Some(1971),
				Some(vec!["GB-ENG".to_string(), "GB-WLS".to_string(), "GB-NIR".to_string()]),
				PublicHolidayType::Public,
			));
		}

		let early_may_bank_holiday = Self::early_may_bank_holiday(year);
		if let Some(early_may_bank_holiday) = early_may_bank_holiday {
			holidays.push(early_may_bank_holiday);
		}

		let spring_bank_holiday = Self::spring_bank_holiday(year);
		if let Some(spring_bank_holiday) = spring_bank_holiday {
			holidays.push(spring_bank_holiday);
		}

		let queens_platinum_jubilee = Self::queens_platinum_jubilee(year);
		if let Some(queens_platinum_jubilee) = queens_platinum_jubilee {
			holidays.push(queens_platinum_jubilee);
		}

		let queens_state_funeral = Self::queens_state_funeral(year);
		if let Some(queens_state_funeral) = queens_state_funeral {
			holidays.push(queens_state_funeral);
		}

		let coronation_bank_holiday = Self::coronation_bank_holiday(year);
		if let Some(coronation_bank_holiday) = coronation_bank_holiday {
			holidays.push(coronation_bank_holiday);
		}

		let christmas_day = DateSystem::shift(Utc.with_ymd_and_hms(year, 12, 25, 0, 0, 0).unwrap(), 2, 2, None);
		holidays.push(PublicHoliday::new_non_fixed(
			christmas_day,
			"Christmas Day",
			"Christmas Day",
			Self::COUNTRYCODE,
			None,
			None,
			PublicHolidayType::Public,
		));

		let sankt_stehpen_day = DateSystem::shift(Utc.with_ymd_and_hms(year, 12, 26, 0, 0, 0).unwrap(), 2, 2, None);
		holidays.push(PublicHoliday::new_non_fixed(
			sankt_stehpen_day,
			"Boxing Day",
			"St. Stephen's Day",
			Self::COUNTRYCODE,
			None,
			None,
			PublicHolidayType::Public,
		));
		holidays.sort_by(|a, b| a.date.cmp(&b.date));

		holidays
	}

	fn spring_bank_holiday(year: i32) -> Option<PublicHoliday> {
		let name = "Spring Bank Holiday";

		if year == 2022 {
			//https://www.gov.uk/government/news/extra-bank-holiday-to-mark-the-queens-platinum-jubilee-in-2022
			Some(PublicHoliday::new_non_fixed(
				Utc.with_ymd_and_hms(year, 6, 2, 0, 0, 0).unwrap(),
				name,
				name,
				Self::COUNTRYCODE,
				None,
				None,
				PublicHolidayType::Public,
			))
		} else {
			let last_monday_in_may = DateSystem::find_last_day(year, 5, Weekday::Mon);
			Some(PublicHoliday::new_non_fixed(
				last_monday_in_may.unwrap(),
				name,
				name,
				Self::COUNTRYCODE,
				Some(1971),
				None,
				PublicHolidayType::Public,
			))
		}
	}

	fn queens_platinum_jubilee(year: i32) -> Option<PublicHoliday> {
		if year == 2022 {
			//Majesty Queen Elizabeth II’s
			Some(PublicHoliday::new_fixed(
				year,
				6,
				3,
				"Queen’s Platinum Jubilee",
				"Queen’s Platinum Jubilee",
				Self::COUNTRYCODE,
				None,
				None,
				PublicHolidayType::Public,
			))
		} else {
			None
		}
	}

	fn queens_state_funeral(year: i32) -> Option<PublicHoliday> {
		if year == 2022 {
			//Majesty Queen Elizabeth II’s (https://www.gov.uk/government/news/bank-holiday-announced-for-her-majesty-queen-elizabeth-iis-state-funeral-on-monday-19-september)
			Some(PublicHoliday::new_fixed(
				year,
				9,
				19,
				"Queen’s State Funeral",
				"Queen’s State Funeral",
				Self::COUNTRYCODE,
				None,
				None,
				PublicHolidayType::Public,
			))
		} else {
			None
		}
	}

	fn coronation_bank_holiday(year: i32) -> Option<PublicHoliday> {
		if year == 2023 {
			//Bank holiday proclaimed in honour of the coronation of His Majesty King Charles III
			//https://www.gov.uk/government/news/bank-holiday-proclaimed-in-honour-of-the-coronation-of-his-majesty-king-charles-iii
			Some(PublicHoliday::new_fixed(
				year,
				5,
				8,
				"Coronation Bank Holiday",
				"Coronation Bank Holiday",
				Self::COUNTRYCODE,
				None,
				None,
				PublicHolidayType::Public,
			))
		} else {
			None
		}
	}

	fn early_may_bank_holiday(year: i32) -> Option<PublicHoliday> {
		let name = "Early May Bank Holiday";

		if year == 2020 {
			//https://www.bbc.co.uk/news/uk-48565417
			let second_friday_in_may = DateSystem::find_day(year, 5, Weekday::Fri, Occurrence::Second);
			Some(PublicHoliday::new_non_fixed(
				second_friday_in_may.unwrap(),
				name,
				name,
				Self::COUNTRYCODE,
				Some(1978),
				None,
				PublicHolidayType::Public,
			))
		} else {
			let first_monday_in_may = DateSystem::find_day(year, 5, Weekday::Mon, Occurrence::First);
			Some(PublicHoliday::new_non_fixed(
				first_monday_in_may.unwrap(),
				name,
				name,
				Self::COUNTRYCODE,
				Some(1978),
				None,
				PublicHolidayType::Public,
			))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::UnitedKingdomHolidayProvider;

	#[test]
	fn test_get_holidays() {
		let result = UnitedKingdomHolidayProvider::get_holidays(2022);
		assert_eq!(result.len(), 17);
		assert_eq!(result.is_empty(), false);
		assert_eq!(result[5].local_name, "Good Friday");
	}
}
