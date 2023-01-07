use super::{countrycode::CountryCode, publicholidaytype::PublicHolidayType};
use chrono::{DateTime, TimeZone, Utc};

#[derive(Debug)]
pub struct PublicHoliday {
	pub date: DateTime<Utc>,
	pub local_name: String,
	pub name: String,
	pub country_code: CountryCode,
	pub fixed: bool,
	pub counties: Option<Vec<String>>,
	pub holiday_type: PublicHolidayType,
	pub launch_year: Option<i32>,
}

impl PublicHoliday {
	#[allow(clippy::too_many_arguments)]
	pub fn new_fixed(
		year: i32,
		month: u32,
		day: u32,
		local_name: &str,
		name: &str,
		country_code: CountryCode,
		launch_year: Option<i32>,
		counties: Option<Vec<String>>,
		holiday_type: PublicHolidayType,
	) -> Self {
		let date = Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap();
		PublicHoliday {
			date,
			local_name: local_name.to_string(),
			name: name.to_string(),
			country_code,
			fixed: true,
			counties,
			holiday_type,
			launch_year,
		}
	}

	pub fn new_non_fixed(
		date: DateTime<Utc>,
		local_name: &str,
		name: &str,
		country_code: CountryCode,
		launch_year: Option<i32>,
		counties: Option<Vec<String>>,
		holiday_type: PublicHolidayType,
	) -> Self {
		PublicHoliday {
			date,
			local_name: local_name.to_string(),
			name: name.to_string(),
			country_code,
			fixed: false,
			counties,
			holiday_type,
			launch_year,
		}
	}

	pub fn global(&self) -> bool {
		self.counties.is_none()
	}

	pub fn set_counties(&mut self, counties: Vec<String>) -> &mut Self {
		self.counties = Some(counties);
		self
	}

	pub fn set_launch_year(&mut self, launch_year: i32) -> &mut Self {
		self.launch_year = Some(launch_year);
		self
	}

	pub fn set_holiday_type(&mut self, holiday_type: PublicHolidayType) -> &mut Self {
		self.holiday_type = holiday_type;
		self
	}
}

impl std::fmt::Display for PublicHoliday {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?} {}", self.date, self.name)
	}
}

#[cfg(test)]
mod tests {
	use chrono::Utc;

	use crate::types::{countrycode::CountryCode, publicholidaytype::PublicHolidayType};

	use super::PublicHoliday;

	#[test]
	fn test_set_counties() {
		let mut holiday = PublicHoliday::new_non_fixed(
			Utc::now(),
			"test",
			"test name",
			CountryCode::DE,
			Some(2022),
			None,
			PublicHolidayType::Public,
		);
		assert_eq!(holiday.counties.is_none(), true);
		holiday.set_counties(vec!["DE-NW".to_string()]);
		assert_eq!(holiday.counties.is_some(), true);
		assert_eq!(holiday.counties.unwrap()[0], "DE-NW".to_string());
	}

	#[test]
	fn test_set_holiday_type() {
		let mut holiday = PublicHoliday::new_non_fixed(
			Utc::now(),
			"test",
			"test name",
			CountryCode::DE,
			Some(2022),
			None,
			PublicHolidayType::Public,
		);
		assert_eq!(holiday.holiday_type, PublicHolidayType::Public);
		holiday.set_holiday_type(PublicHolidayType::Bank);
		assert_eq!(holiday.holiday_type, PublicHolidayType::Bank);
	}

	#[test]
	fn test_set_launch_year() {
		let mut holiday = PublicHoliday::new_non_fixed(
			Utc::now(),
			"test",
			"test name",
			CountryCode::DE,
			None,
			None,
			PublicHolidayType::Public,
		);
		assert_eq!(holiday.launch_year.is_none(), true);
		holiday.set_launch_year(1991);
		assert_eq!(holiday.launch_year.is_some(), true);
		assert_eq!(holiday.launch_year.unwrap(), 1991);
	}
}
