use crate::{
	providers::base::orthodoxprovider::OrthodoxProvider,
	types::{countrycode::CountryCode, publicholiday::PublicHoliday, publicholidaytype::PublicHolidayType},
};

pub struct UkrainHolidayProvider {}

impl UkrainHolidayProvider {
	const COUNTRY_CODE: CountryCode = CountryCode::UA;

	pub fn get_holidays(year: i32) -> Vec<PublicHoliday> {
		let mut holidays = vec![
			PublicHoliday::new_fixed(
				year,
				1,
				1,
				"Новий Рік",
				"New Year's Day",
				Self::COUNTRY_CODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				1,
				7,
				"Різдво",
				"(Julian) Christmas",
				Self::COUNTRY_CODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				3,
				8,
				"Міжнародний жіночий день",
				"International Women's Day",
				Self::COUNTRY_CODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			OrthodoxProvider::easter_sunday("Великдень", year, Self::COUNTRY_CODE),
			OrthodoxProvider::pentecost("Трійця", year, Self::COUNTRY_CODE),
			PublicHoliday::new_fixed(
				year,
				5,
				1,
				"День праці",
				"International Workers' Day",
				Self::COUNTRY_CODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				5,
				9,
				"День перемоги над нацизмом у Другій світовій війні",
				"Victory day over Nazism in World War II",
				Self::COUNTRY_CODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				6,
				28,
				"День Конституції",
				"Constitution Day",
				Self::COUNTRY_CODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				8,
				24,
				"День Незалежності",
				"Independence Day",
				Self::COUNTRY_CODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				10,
				14,
				"День захисника України",
				"Defender of Ukraine Day",
				Self::COUNTRY_CODE,
				None,
				None,
				PublicHolidayType::Public,
			),
			PublicHoliday::new_fixed(
				year,
				12,
				25,
				"Різдво",
				"(Gregorian and Revised Julian) Christmas",
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
	use super::UkrainHolidayProvider;

	#[test]
	fn test_get_holidays() {
		let result = UkrainHolidayProvider::get_holidays(2022);
		assert_eq!(result.len(), 11);
		assert_eq!(result.is_empty(), false);
		assert_eq!(
			result[5].local_name,
			"День перемоги над нацизмом у Другій світовій війні"
		);
	}
}
