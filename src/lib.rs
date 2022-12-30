#[macro_use]
extern crate lazy_static;

use crate::modules::providers::{
	germanholidayprovider::GermanHolidayProvider, greeceholidayprovider::GreeceHolidayProvider,
};

mod modules;

fn main() {
	GermanHolidayProvider::get_holidays(2022)
		.iter()
		.for_each(|element| println!("{}", element));
	GreeceHolidayProvider::get_holidays(2022)
		.iter()
		.for_each(|element| println!("{}", element));
}
