use std::collections::HashMap;

use chrono::{TimeZone, Utc, Weekday};

use crate::{
	datesystem::DateSystem,
	providers::base::catholicprovider::CatholicProvider,
	types::{
		countrycode::CountryCode, occurrence::Occurrence, publicholiday::PublicHoliday,
		publicholidaytype::PublicHolidayType,
	},
};

pub struct UnitedStatesHolidayProvider {}

impl UnitedStatesHolidayProvider {
	const COUNTRYCODE: CountryCode = CountryCode::US;

	/// Gets a hashmap of the available counties of germany
	pub fn get_counties() -> HashMap<String, String> {
		HashMap::from([
			("US-AL".to_string(), "Alabama".to_string()),
			("US-AK".to_string(), "Alaska".to_string()),
			("US-AZ".to_string(), "Arizona".to_string()),
			("US-AR".to_string(), "Arkansas".to_string()),
			("US-CA".to_string(), "California".to_string()),
			("US-CO".to_string(), "Colorado".to_string()),
			("US-CT".to_string(), "Connecticut".to_string()),
			("US-DE".to_string(), "Delaware".to_string()),
			("US-FL".to_string(), "Florida".to_string()),
			("US-GA".to_string(), "Georgia".to_string()),
			("US-HI".to_string(), "Hawaii".to_string()),
			("US-ID".to_string(), "Idaho".to_string()),
			("US-IL".to_string(), "Illinois".to_string()),
			("US-IN".to_string(), "Indiana".to_string()),
			("US-IA".to_string(), "Iowa".to_string()),
			("US-KS".to_string(), "Kansas".to_string()),
			("US-KY".to_string(), "Kentucky".to_string()),
			("US-LA".to_string(), "Louisiana".to_string()),
			("US-ME".to_string(), "Maine".to_string()),
			("US-MD".to_string(), "Maryland".to_string()),
			("US-MA".to_string(), "Massachusetts".to_string()),
			("US-MI".to_string(), "Michigan".to_string()),
			("US-MN".to_string(), "Minnesota".to_string()),
			("US-MS".to_string(), "Mississippi".to_string()),
			("US-MO".to_string(), "Missouri".to_string()),
			("US-MT".to_string(), "Montana".to_string()),
			("US-NE".to_string(), "Nebraska".to_string()),
			("US-NV".to_string(), "Nevada".to_string()),
			("US-NH".to_string(), "New Hampshire".to_string()),
			("US-NJ".to_string(), "New Jersey".to_string()),
			("US-NM".to_string(), "New Mexico".to_string()),
			("US-NY".to_string(), "New York".to_string()),
			("US-NC".to_string(), "North Carolina".to_string()),
			("US-ND".to_string(), "North Dakota".to_string()),
			("US-OH".to_string(), "Ohio".to_string()),
			("US-OK".to_string(), "Oklahoma".to_string()),
			("US-OR".to_string(), "Oregon".to_string()),
			("US-PA".to_string(), "Pennsylvania".to_string()),
			("US-RI".to_string(), "Rhode Island".to_string()),
			("US-SC".to_string(), "South Carolina".to_string()),
			("US-SD".to_string(), "South Dakota".to_string()),
			("US-TN".to_string(), "Tennessee".to_string()),
			("US-TX".to_string(), "Texas".to_string()),
			("US-UT".to_string(), "Utah".to_string()),
			("US-VT".to_string(), "Vermont".to_string()),
			("US-VA".to_string(), "Virginia".to_string()),
			("US-WA".to_string(), "Washington".to_string()),
			("US-WV".to_string(), "West Virginia".to_string()),
			("US-WI".to_string(), "Wisconsin".to_string()),
			("US-WY".to_string(), "Wyoming".to_string()),
		])
	}

	/// .
	///
	/// # Panics
	///
	/// Panics if .
	pub fn get_holidays(year: i32) -> Vec<PublicHoliday> {
		let third_monday_in_january = DateSystem::find_day(year, 1, Weekday::Mon, Occurrence::Third);
		let third_monday_in_february = DateSystem::find_day(year, 2, Weekday::Mon, Occurrence::Third);
		let last_monday_in_may = DateSystem::find_last_day(year, 5, Weekday::Mon);
		let first_monday_in_september = DateSystem::find_day(year, 9, Weekday::Mon, Occurrence::First);
		let second_monday_in_october = DateSystem::find_day(year, 10, Weekday::Mon, Occurrence::Second);
		let fourth_thursday_in_november = DateSystem::find_day(year, 11, Weekday::Thu, Occurrence::Fourth);

		let mut holidays = Vec::new();

		let new_years_day = DateSystem::shift(Utc.with_ymd_and_hms(year, 1, 1, 0, 0, 0).unwrap(), -1, 1, None);
		holidays.push(PublicHoliday::new_non_fixed(
			new_years_day,
			"New Year's Day",
			"New Year's Day",
			Self::COUNTRYCODE,
			None,
			None,
			PublicHolidayType::Public,
		));

		if let Some(third_monday_in_january) = third_monday_in_january {
			holidays.push(PublicHoliday::new_non_fixed(
				third_monday_in_january,
				"Martin Luther King, Jr. Day",
				"Martin Luther King, Jr. Day",
				Self::COUNTRYCODE,
				None,
				None,
				PublicHolidayType::Public,
			));
		}
		if let Some(third_monday_in_february) = third_monday_in_february {
			holidays.push(PublicHoliday::new_non_fixed(
				third_monday_in_february,
				"Presidents Day",
				"Washington's Birthday",
				Self::COUNTRYCODE,
				None,
				None,
				PublicHolidayType::Public,
			));
		}
		if let Some(last_monday_in_may) = last_monday_in_may {
			holidays.push(PublicHoliday::new_non_fixed(
				last_monday_in_may,
				"Memorial Day",
				"Memorial Day",
				Self::COUNTRYCODE,
				None,
				None,
				PublicHolidayType::Public,
			));
		}
		holidays.push(CatholicProvider::good_friday("Good Friday", year, Self::COUNTRYCODE));
		let holiday_length = &holidays.len();
		if let Some(holiday) = holidays.get_mut(*holiday_length) {
			holiday.set_counties(vec![
				"US-CT".to_string(),
				"US-DE".to_string(),
				"US-HI".to_string(),
				"US-IN".to_string(),
				"US-KY".to_string(),
				"US-LA".to_string(),
				"US-NC".to_string(),
				"US-ND".to_string(),
				"US-NJ".to_string(),
				"US-TN".to_string(),
			]);
		}
		holidays.push(CatholicProvider::good_friday("Good Friday", year, Self::COUNTRYCODE));
		let holiday_length = &holidays.len();
		if let Some(holiday) = holidays.get_mut(*holiday_length) {
			holiday.set_counties(vec!["US-TX".to_string()]);
			holiday.set_holiday_type(PublicHolidayType::Optional);
		}
		if year >= 2021 {
			let june_teenth = DateSystem::shift(Utc.with_ymd_and_hms(year, 6, 19, 0, 0, 0).unwrap(), -1, 1, None);
			holidays.push(PublicHoliday::new_non_fixed(
				june_teenth,
				"Juneteenth",
				"Juneteenth",
				Self::COUNTRYCODE,
				Some(2021),
				None,
				PublicHolidayType::Public,
			));
		}

		let independence_day = DateSystem::shift(Utc.with_ymd_and_hms(year, 7, 4, 0, 0, 0).unwrap(), -1, 1, None);
		holidays.push(PublicHoliday::new_non_fixed(
			independence_day,
			"Independence Day",
			"Independence Day",
			Self::COUNTRYCODE,
			None,
			None,
			PublicHolidayType::Public,
		));

		if let Some(first_monday_in_september) = first_monday_in_september {
			holidays.push(PublicHoliday::new_non_fixed(
				first_monday_in_september,
				"Labor Day",
				"Labor Day",
				Self::COUNTRYCODE,
				None,
				None,
				PublicHolidayType::Public,
			));
		}

		if let Some(second_monday_in_october) = second_monday_in_october {
			holidays.push(PublicHoliday::new_non_fixed(
				second_monday_in_october,
				"Columbus Day",
				"Columbus Day",
				Self::COUNTRYCODE,
				None,
				Some(vec![
					"US-AL".to_string(),
					"US-AZ".to_string(),
					"US-CO".to_string(),
					"US-CT".to_string(),
					"US-DC".to_string(),
					"US-GA".to_string(),
					"US-ID".to_string(),
					"US-IL".to_string(),
					"US-IN".to_string(),
					"US-IA".to_string(),
					"US-KS".to_string(),
					"US-KY".to_string(),
					"US-LA".to_string(),
					"US-ME".to_string(),
					"US-MD".to_string(),
					"US-MA".to_string(),
					"US-MS".to_string(),
					"US-MO".to_string(),
					"US-MT".to_string(),
					"US-NE".to_string(),
					"US-NH".to_string(),
					"US-NJ".to_string(),
					"US-NM".to_string(),
					"US-NY".to_string(),
					"US-NC".to_string(),
					"US-OH".to_string(),
					"US-OK".to_string(),
					"US-PA".to_string(),
					"US-RI".to_string(),
					"US-SC".to_string(),
					"US-TN".to_string(),
					"US-UT".to_string(),
					"US-VA".to_string(),
					"US-WV".to_string(),
				]),
				PublicHolidayType::Public,
			));
		}

		let veterans_day = DateSystem::shift(Utc.with_ymd_and_hms(year, 11, 11, 0, 0, 0).unwrap(), -1, 1, None);
		holidays.push(PublicHoliday::new_non_fixed(
			veterans_day,
			"Veterans Day",
			"Veterans Day",
			Self::COUNTRYCODE,
			None,
			None,
			PublicHolidayType::Public,
		));

		if let Some(fourth_thursday_in_november) = fourth_thursday_in_november {
			holidays.push(PublicHoliday::new_non_fixed(
				fourth_thursday_in_november,
				"Thanksgiving Day",
				"Thanksgiving Day",
				Self::COUNTRYCODE,
				Some(1863),
				None,
				PublicHolidayType::Public,
			));
		}

		let christmas_day = DateSystem::shift(Utc.with_ymd_and_hms(year, 12, 25, 0, 0, 0).unwrap(), -1, 1, None);
		holidays.push(PublicHoliday::new_non_fixed(
			christmas_day,
			"Christmas Day",
			"Christmas Day",
			Self::COUNTRYCODE,
			None,
			None,
			PublicHolidayType::Public,
		));

		if (year - 1) % 4 == 0 {
			if year >= 1937 {
				holidays.push(PublicHoliday::new_fixed(
					year,
					1,
					20,
					"Inauguration Day",
					"Inauguration Day",
					Self::COUNTRYCODE,
					None,
					Some(vec![
						"US-DC".to_string(),
						"US-LA".to_string(),
						"US-MD".to_string(),
						"US-VA".to_string(),
					]),
					PublicHolidayType::Public,
				));
			} else {
				holidays.push(PublicHoliday::new_fixed(
					year,
					3,
					4,
					"Inauguration Day",
					"Inauguration Day",
					Self::COUNTRYCODE,
					None,
					Some(vec![
						"US-DC".to_string(),
						"US-LA".to_string(),
						"US-MD".to_string(),
						"US-VA".to_string(),
					]),
					PublicHolidayType::Public,
				));
			}
		}

		holidays.sort_by(|a, b| a.date.cmp(&b.date));

		holidays
	}
}

#[cfg(test)]
mod tests {
	use super::UnitedStatesHolidayProvider;

	#[test]
	fn test_get_holidays() {
		let result = UnitedStatesHolidayProvider::get_holidays(2022);
		assert_eq!(result.len(), 13);
		assert_eq!(result.is_empty(), false);
		assert_eq!(result[5].local_name, "Memorial Day");
	}
}
