use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc, Weekday};

use super::types::occurrence::Occurrence;

pub struct DateSystem {}

///Providers helper methods for date time handling
///
impl DateSystem {
	/// Gets a specific occurence for a weekday in a specific month for a specific year
	///
	/// # Example
	///
	/// ```
	///
	/// use horus::{datesystem::DateSystem, types::occurrence::Occurrence};
	/// use chrono::{TimeZone, Utc, Weekday};
	///
	/// let result = DateSystem::find_day(2023, 1, chrono::Weekday::Mon, Occurrence::First);
	///
	/// assert_eq!(result.is_some(), true);
	/// assert_eq!(result.unwrap(), Utc.with_ymd_and_hms(2023, 1, 2, 0, 0, 0).unwrap());
	/// ```
	pub fn find_day(year: i32, month: u32, day: Weekday, occurence: Occurrence) -> Option<DateTime<Utc>> {
		let oc = occurence as i32;
		if oc == 0 || oc > 5 {
			return None;
		}

		let first_day_of_month = NaiveDate::from_ymd_opt(year, month, 1).unwrap();

		let days_needed =
			(day.num_days_from_sunday() as i32) - (first_day_of_month.weekday().num_days_from_sunday() as i32);
		let resulted_day = if days_needed < 0 { days_needed + 7 } else { days_needed } + 1 + (7 * (oc - 1));

		if resulted_day > Self::get_days_of_month(year, month) {
			return None;
		}

		Some(DateTime::<Utc>::from_utc(
			NaiveDate::from_ymd_opt(year, month, resulted_day as u32)
				.unwrap()
				.and_hms_opt(0, 0, 0)
				.unwrap(),
			Utc,
		))
	}

	/// Gets a the last occurence of a weekday in a specific month for a specific year
	///
	/// # Example
	///
	/// ```
	///
	/// use horus::{datesystem::DateSystem, types::occurrence::Occurrence};
	/// use chrono::{TimeZone, Utc, Weekday};
	///
	/// let result = DateSystem::find_last_day(2023, 1, chrono::Weekday::Mon);
	///
	/// assert_eq!(result.is_some(), true);
	/// assert_eq!(result.unwrap(), Utc.with_ymd_and_hms(2023, 1, 30, 0, 0, 0).unwrap());
	/// ```
	pub fn find_last_day(year: i32, month: u32, day: Weekday) -> Option<DateTime<Utc>> {
		let resulted_day = Self::find_day(year, month, day, Occurrence::Fifth);
		if resulted_day.is_some() {
			return resulted_day;
		}

		Self::find_day(year, month, day, Occurrence::Fourth)
	}

	/// Shifts the date depending on the weekday and the given days
	///
	/// # Example
	///
	/// ```
	///
	/// use horus::{datesystem::DateSystem, types::occurrence::Occurrence};
	/// use chrono::{TimeZone, Utc, Weekday};
	///
	/// let date = Utc.with_ymd_and_hms(2023, 1, 29, 0, 0, 0).unwrap();
	/// let result = DateSystem::shift(date, 1, 1, None);
	///
	/// assert_eq!(result, Utc.with_ymd_and_hms(2023, 1, 30, 0, 0, 0).unwrap());
	/// ```
	pub fn shift(
		value: DateTime<Utc>,
		days_saturday: i64,
		days_sunday: i64,
		days_monday: Option<i64>,
	) -> DateTime<Utc> {
		match value.weekday() {
			Weekday::Sat => value + Duration::days(days_saturday),
			Weekday::Sun => value + Duration::days(days_sunday),
			Weekday::Mon => {
				if let Some(days_monday) = days_monday {
					value + Duration::days(days_monday)
				} else {
					value
				}
			}
			_ => value,
		}
	}

	fn get_days_of_month(year: i32, month: u32) -> i32 {
		if month == 12 {
			NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
		} else {
			NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
		}
		.signed_duration_since(NaiveDate::from_ymd_opt(year, month, 1).unwrap())
		.num_days() as i32
	}
}
