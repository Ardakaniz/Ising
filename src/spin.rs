use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Spin {
	UP,
	DOWN,
}

impl fmt::Display for Spin {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}",
			match self {
				Spin::UP   => "+",
				Spin::DOWN => "-",
			}
		)
	}
}
impl TryFrom<i32> for Spin {
	type Error = &'static str;

	fn try_from(i: i32) -> Result<Self, Self::Error> {
		match i {
			1  => Ok(Spin::UP),
			-1 => Ok(Spin::DOWN),
			_  => Err("Invalid spin value"),
		}
	}
}
impl From<Spin> for i32 {
	fn from(s: Spin) -> Self {
		match s {
			Spin::UP   => 1,
			Spin::DOWN => -1,
		}
	}
}
impl From<bool> for Spin {
	fn from(b: bool) -> Self {
		match b {
			true => Spin::UP,
			false => Spin::DOWN,
		}
	}
}
impl From<Spin> for bool {
	fn from(s: Spin) -> Self {
		match s {
			Spin::UP   => true,
			Spin::DOWN => false,
		}
	}
}

impl Spin {
	pub fn flip(&mut self) {
		*self = match self {
			Spin::UP   => Spin::DOWN,
			Spin::DOWN => Spin::UP,
		}
	}
}