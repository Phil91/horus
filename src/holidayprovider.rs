use std::collections::HashMap;

use crate::{
	providers::locations::{
		germanholidayprovider::GermanHolidayProvider, greeceholidayprovider::GreeceHolidayProvider,
		polandholidayprovider::PolandHolidayProvider, russiaholidayprovider::RussiaHolidayProvider,
		ukrainholidayprovider::UkrainHolidayProvider, unitedkingdomholidayprovider::UnitedKingdomHolidayProvider,
		unitedstatesholidayprovider::UnitedStatesHolidayProvider,
	},
	types::{countrycode::CountryCode, publicholiday::PublicHoliday},
};

pub struct HolidayProvider {}

/// Holiday provider to get either the counties or all holidays for a specific country
///
/// # Example
///
/// ```
/// use horus::{holidayprovider::HolidayProvider, types::countrycode::CountryCode};
///
/// let holidays = HolidayProvider::get_holidays(2022, CountryCode::DE);
///
/// assert_eq!(holidays.len(), 19);
/// assert_eq!(holidays.is_empty(), false);
/// assert_eq!(holidays[5].local_name, "Ostermontag".to_string());
/// ```
impl HolidayProvider {
	/// Gets all holidays for the given year and country code
	///
	/// # Example
	///
	/// ```
	/// use horus::{holidayprovider::HolidayProvider, types::countrycode::CountryCode};
	///
	/// let holidays = HolidayProvider::get_holidays(2022, CountryCode::DE);
	///
	/// assert_eq!(holidays.len(), 19);
	/// assert_eq!(holidays.is_empty(), false);
	/// assert_eq!(holidays[5].local_name, "Ostermontag".to_string());
	/// ```
	pub fn get_holidays(year: i32, country_code: CountryCode) -> Vec<PublicHoliday> {
		if country_code == CountryCode::DE {
			return GermanHolidayProvider::get_holidays(year);
		} else if country_code == CountryCode::GB {
			return UnitedKingdomHolidayProvider::get_holidays(year);
		} else if country_code == CountryCode::GR {
			return GreeceHolidayProvider::get_holidays(year);
		} else if country_code == CountryCode::PL {
			return PolandHolidayProvider::get_holidays(year);
		} else if country_code == CountryCode::RU {
			return RussiaHolidayProvider::get_holidays(year);
		} else if country_code == CountryCode::UA {
			return UkrainHolidayProvider::get_holidays(year);
		} else if country_code == CountryCode::US {
			return UnitedStatesHolidayProvider::get_holidays(year);
		}

		Vec::new()
	}

	/// Gets all counties for the country code. If no counties are existing, None will be returned.
	///
	/// # Example
	///
	/// ```
	/// use horus::{holidayprovider::HolidayProvider, types::countrycode::CountryCode};
	///
	/// let counties = HolidayProvider::get_counties(CountryCode::DE);
	///
	/// assert_eq!(counties.unwrap().len(), 16);
	/// ```
	pub fn get_counties(country_code: CountryCode) -> Option<HashMap<String, String>> {
		if country_code == CountryCode::DE {
			return Some(GermanHolidayProvider::get_counties());
		} else if country_code == CountryCode::GB {
			return Some(UnitedKingdomHolidayProvider::get_counties());
		} else if country_code == CountryCode::US {
			return Some(UnitedStatesHolidayProvider::get_counties());
		}

		None
	}
}
