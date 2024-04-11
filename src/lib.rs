#[cfg(test)]
mod tests;

use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};
use simple_2d_vector::{Vector2D, Point2D};

/// An enum type that defines the three planes that a [`Vector3D`] can occupy.
///
/// The first plane of the vector's `origin` or `target` is its x-y plane,
/// the second plane is its y-z plane, and its third plane is its z-x plane.
pub enum Plane {
	/// The x-y plane of the vector's `origin` or `target`.
	XY,
	/// The y-z plane of the vector's `origin` or `target`.
	YZ,
	/// The z-x plane of the vector's `origin` or `target`.
	ZX
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[must_use]
/// A grid-based three-dimensional representation of a mathematical vector.
pub struct Vector3D {
	/// Defines the origin point of the vector.
	pub origin: Point3D,
	/// The end of the vector in relation to its origin.
	///
	/// While this is a [`Point3D`] type,
	/// much like `origin`, it represents a relative value
	/// rather than an absolute one.
	pub target: Point3D,
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[must_use]
/// A structure that represents a point in three-dimensional space.
///
/// The grid is centered around (0, 0, 0).
pub struct Point3D {
	/// The x coordinate of the [`Point3D`]
	pub x: f64,
	/// The y coordinate of the [`Point3D`]
	pub y: f64,
	/// The z coordinate of the [`Point3D`]
	pub z: f64,
}

impl Point3D {
	/// Constructs a new [`Point3D`] at coordinates (0, 0, 0).
	pub fn zero() -> Self {
		Point3D {
			x: 0.0,
			y: 0.0,
			z: 0.0,
		}
	}

	/// Constructs a new [`Point3D`] at the given coordinates.
	pub fn new(x: f64, y: f64, z: f64) -> Self {
		Point3D {
			x,
			y,
			z,
		}
	}

	/// Sets the `x` coordinate of [`self`]
	pub fn set_x(mut self, x: f64) -> Self {
		self.x = x;
		self
	}

	/// Sets the `y` coordinate of [`self`]
	pub fn set_y(mut self, y: f64) -> Self {
		self.y = y;
		self
	}

	/// Sets the `z` coordinate of [`self`]
	pub fn set_z(mut self, z: f64) -> Self {
		self.z = z;
		self
	}

	/// Shifts the left-hand-side [`Point3D`] by the right-hand-side tuple of types
	/// which implement [`Into<f64>`].
	pub fn shift(mut self, shift: (impl Into<f64>, impl Into<f64>, impl Into<f64>)) -> Self {
		self.x += shift.0.into();
		self.y += shift.1.into();
		self.z += shift.2.into();
		self
	}

	/// Returns the distance between [`self`] and the given [`Point3D`].
	pub fn get_distance(&self, rhs: Point3D) -> f64 {
		f64::sqrt(
			(self.x - rhs.x).powf(2.0)
				+
				(self.y - rhs.y).powf(2.0)
				+
				(self.z - rhs.z).powf(2.0)
		)
	}

	/// Constructs a new [`Vector3D`] with an `origin` [`Point3D`]([`self`]) and an absolute
	/// `target` [`Point3D`]
	pub fn to_vector3d(&self, target: &Point3D) -> Vector3D {
		Vector3D {
			origin: *self,
			target: *target
		}
	}
}

impl Vector3D {
	/// A zero [`Vector3D`] with both `origin` and `target` at (0, 0, 0).
	pub fn zero() -> Self {
		Vector3D {
			origin: Point3D::new(
				0.0,
				0.0,
				0.0,
			),
			target: Point3D::new(
				0.0,
				0.0,
				0.0,
			),
		}
	}

	/// A [`Vector3D`] with a `target` of (0, 0, 0).
	///
	/// Also known as a zero-length vector
	pub fn null(origin: Point3D) -> Self {
		Vector3D {
			origin,
			target: Point3D::zero(),
		}
	}

	/// Constructs a [`Vector3D`] from the provided `origin` and `target`.
	pub fn new(origin: Point3D, target: Point3D) -> Self {
		Vector3D {
			origin,
			target,
		}
	}

	/// Constructs a [`Vector3D`] from the provided `origin` and `target`.
	///
	/// The difference between this and [`Vector3D::new()`] is that this
	/// treats `target` as an absolute value,
	/// meaning that `Vector2D::with_absolute_target((1.0,0.0,2.2),(2.0,1.0,0.0))`
	/// constructs a vector with an `origin` of (1.0, 0.0, 2.2) but a `target`
	/// of (1.0, 1.0, -2.0) as opposed to a `target` of (2.0, 1.0, 0.0).
	pub fn with_absolute_target(origin: Point3D, target: Point3D) -> Self {
		Vector3D {
			origin,
			target: Point3D::new(
				target.x - origin.x,
				target.y - origin.y,
				target.z - origin.z
			),
		}
	}

	/// Constructs a [`Vector3D`] from the provided [`Vector2D`], `origin` and `target`.
	///
	/// This creates a new [`Vector3D`] with the same x and y coordinates as the input
	/// [`Vector2D`], but adds the third dimension as the z coordinate
	pub fn from_2d_vector(vector2d: Vector2D, origin_z: f64, target_z: f64) -> Self {
		let (x,y,z) = (vector2d.origin.x, vector2d.origin.y, origin_z);
		let (x_target,y_target,z_target) = (vector2d.target.x, vector2d.target.y, target_z);
		
		Vector3D {
			origin: Point3D::new(
				x,
				y,
				z,
			),
			target: Point3D::new(
				x_target,
				y_target,
				z_target,
			)
		}
	}

	/// Sets the `origin` of [`self`]
	pub fn set_origin(mut self, origin: Point3D) -> Self {
		self.origin = origin;
		self
	}

	/// Sets the `target` of [`self`]
	pub fn set_target(mut self, target: Point3D) -> Self {
		self.target = target;
		self
	}

	/// Sets the `target` of [`self`].
	///
	/// Uses an absolute `target` instead of a relative one.
	///
	/// Due to issues with imprecise float arithmetic operations,
	/// only the first six digits per coordinate of the input `target` are kept,
	/// and anything past that is rounded off.
	pub fn set_target_absolute(mut self, target: Point3D) -> Self {
		self.target = Point3D::new(
			((target.x - self.origin.x) * 1_000_000.0).round() / 1_000_000.0,
			((target.y - self.origin.y) * 1_000_000.0).round() / 1_000_000.0,
			((target.z - self.origin.z) * 1_000_000.0).round() / 1_000_000.0,
		);
		self
	}

	/// Shifts the left-hand-side [`Vector3D`] by the right-hand-side tuple of types
	/// which implement [`Into<f64>`].
	///
	/// The resulting [`Vector3D`] has the same `target` value, as it's relative to its `origin`,
	/// but a shifted `origin`, which results in an overall shift of the [`Vector3D`].
	pub fn shift(mut self, shift: (impl Into<f64>, impl Into<f64>, impl Into<f64>)) -> Self {
		self.origin = Point3D::new(
			self.origin.x + shift.0.into(),
			self.origin.y + shift.1.into(),
			self.origin.z + shift.2.into()
		);
		self
	}

	/// Returns the length of a [`Vector3D`].
	pub fn get_magnitude(&self) -> f64 {
		f64::sqrt(
			(self.origin.x - self.target.x).powf(2.0)
				+
				(self.origin.y - self.target.y).powf(2.0)
				+
				(self.origin.z - self.target.z).powf(2.0)
		)
	}

	/// Returns a [`Vector2D`] that represents the points
	/// along a specific [`Plane`] of the input [`Vector3D`]
	pub fn vector2d_along_plane(vector3d: &Vector3D, plane: &Plane) -> Vector2D {
		match plane {
			Plane::XY => {
				Vector2D {
					origin: Point2D::new(
						vector3d.origin.x,
						vector3d.origin.y,
					),
					target: Point2D::new(
						vector3d.target.x,
						vector3d.target.y,
					),
				}
			}
			Plane::YZ => {
				Vector2D {
					origin: Point2D::new(
						vector3d.origin.z,
						vector3d.origin.y,
					),
					target: Point2D::new(
						vector3d.target.z,
						vector3d.target.y,
					),
				}
			}
			Plane::ZX => {
				Vector2D {
					origin: Point2D::new(
						vector3d.origin.z,
						vector3d.origin.x,
					),
					target: Point2D::new(
						vector3d.target.z,
						vector3d.target.x,
					),
				}
			}
		}
	}

	/// Returns a [`Vector2D`] that represents the points
	/// along a specific [`Plane`] of the input [`self`]
	pub fn to_vector2d_along_plane(&self, plane: &Plane) -> Vector2D {
		match plane {
			Plane::XY => {
				Vector2D {
					origin: Point2D::new(
						self.origin.x,
						self.origin.y,
					),
					target: Point2D::new(
						self.target.x,
						self.target.y,
					),
				}
			}
			Plane::YZ => {
				Vector2D {
					origin: Point2D::new(
						self.origin.z,
						self.origin.y,
					),
					target: Point2D::new(
						self.target.z,
						self.target.y,
					),
				}
			}
			Plane::ZX => {
				Vector2D {
					origin: Point2D::new(
						self.origin.z,
						self.origin.x,
					),
					target: Point2D::new(
						self.target.z,
						self.target.x,
					),
				}
			}
		}
	}
}

impl Display for Vector3D {
	/// Formats the [`Vector3D`] with the given formatter and prepares it for user-facing output.
	///
	/// The format is `(origin.x,origin.y,origin.z)[target.x,target.y,target.z]`.
	///
	/// Example:
	/// ```rust
	/// use simple_3d_vector::{Vector3D, Point3D};
	///
	/// let vector = Vector3D::new(
	///     Point3D::new(2.2, 1.1, 0.0),
	///     Point3D::new(0.0, 1.1, 2.2),
	/// );
	///
	/// assert_eq!(format!("{vector}"), "(2.2,1.1,0)[0,1.1,2.2]");
	/// ```
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"({},{},{})[{},{},{}]",
			self.origin.x,
			self.origin.y,
			self.origin.z,
			self.target.x,
			self.target.y,
			self.target.z,
		)
	}
}

impl Add for Vector3D {
	type Output = Self;

	/// Performs a mathematical addition of two [`Vector3D`]s.
	fn add(self, rhs: Self) -> Self::Output {
		Vector3D {
			origin: self.origin,
			target: Point3D::new(
				rhs.target.x + self.target.x,
				rhs.target.y + self.target.y,
				rhs.target.z + self.target.z,
			),
		}
	}
}

impl Sub for Vector3D {
	type Output = Self;

	/// Performs a mathematical subtraction of two [`Vector3D`]s.
	fn sub(self, rhs: Self) -> Self::Output {
		Vector3D {
			origin: self.origin,
			target: Point3D::new(
				self.target.x - rhs.target.x,
				self.target.y - rhs.target.y,
				self.target.z - rhs.target.z,
			),
		}
	}
}

impl Display for Point3D {
	/// Formats the [`Point3D`] with the given formatter and prepares it for user-facing output.
	///
	/// The format is `(x,y,z)`.
	///
	/// Example:
	/// ```rust
	/// use simple_3d_vector::Point3D;
	///
	/// let point = Point3D::new(
	///     2.2,
	///     1.1,
	///     0.0
	/// );
	///
	/// assert_eq!(format!("{point}"), "(2.2,1.1,0)");
	/// ```
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"({},{},{})",
			self.x,
			self.y,
			self.z,
		)
	}
}

impl<T: Into<f64>, E: Into<f64>, M: Into<f64>> From<(T, E, M)> for Point3D {
	fn from(value: (T, E, M)) -> Self {
		let value: (f64, f64, f64) = (value.0.into(), value.1.into(), value.2.into());

		Point3D {
			x: value.0,
			y: value.1,
			z: value.2,
		}
	}
}

impl Add for Point3D {
	type Output = Self;

	/// Performs a mathematical addition of two [`Point2D`]s.
	fn add(self, rhs: Self) -> Self::Output {
		Point3D {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z,
		}
	}
}

impl Sub for Point3D {
	type Output = Self;

	/// Performs a mathematical subtraction of two [`Point2D`]s.
	fn sub(self, rhs: Self) -> Self::Output {
		Point3D {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z,
		}
	}
}