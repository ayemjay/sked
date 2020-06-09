use chrono::{prelude::*, DateTime, TimeZone};

/// A specifier for when something happens.
#[allow(dead_code)]
#[derive(Debug)]
pub enum Specifier<Tz: TimeZone> {
	/// A pattern of days and times which must be computed against to give a
	/// definitive answer.
	Weekly { day: String, time: String },

	/// A pattern of times
	Daily { time: String },

	/// An exact time
	Exact(DateTime<Tz>),
}

#[derive(Debug)]
pub struct Instances<'iteration, Tz: TimeZone> {
	specifier: &'iteration Specifier<Tz>,
	basis: DateTime<Tz>,
}

impl<'iteration, Tz: TimeZone> Iterator for Instances<'iteration, Tz> {
	type Item = chrono::DateTime<Tz>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.specifier {
			Specifier::Exact(dt) if dt != &self.basis => {
				let next = dt.to_owned();
				self.basis = dt.to_owned();
				Some(next)
			}
			Specifier::Exact(dt) if dt == &self.basis => None,
			Specifier::Exact(_) => panic!(),
			Specifier::Weekly { .. } => todo!(),
			Specifier::Daily { time } => {
				let specifier_time: chrono::NaiveTime = NaiveTime::parse_from_str(time, "%H:%M")
					.or(NaiveTime::parse_from_str(time, "%H:%M:%S"))
					.expect("invalid time specifier");

				let instance = self.basis.date().and_time(specifier_time).unwrap();

				self.basis = self.basis.to_owned() + chrono::Duration::days(1);

				Some(instance)
			}
		}
	}
}
