use super::{Specifier, Status};
use chrono::TimeZone;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Exception<Tz: TimeZone> {
	effect: Option<Status>,
	effective: Option<Specifier<Tz>>,
	expires: Option<Specifier<Tz>>,
}

impl<Tz: TimeZone> Default for Exception<Tz> {
	fn default() -> Self {
		Self {
			effect: None,
			effective: None,
			expires: None,
		}
	}
}

impl<Tz: TimeZone> Exception<Tz> {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn effect(mut self, effect: Status) -> Self {
		self.effect = Some(effect);
		self
	}

	pub fn effective(mut self, effective: Specifier<Tz>) -> Self {
		self.effective = Some(effective);
		self
	}

	pub fn expires(mut self, expires: Specifier<Tz>) -> Self {
		self.expires = Some(expires);
		self
	}
}