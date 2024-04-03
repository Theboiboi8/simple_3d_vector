use std::ops::{Add, Sub};
use crate::{Vector3D, Point3D};

#[test]
fn new_vector() {
	let new_vector = Vector3D::new(
		Point3D::zero(),
		Point3D::zero()
	);
	let vector = Vector3D {
		origin: Point3D::zero(),
		target: Point3D::zero(),
	};
	assert_eq!(vector, new_vector);
}
#[test]
fn zero_vector() {
	let zero_vector = Vector3D::zero();
	let vector = Vector3D::new(
		Point3D::zero(),
		Point3D::zero()
	);
	assert_eq!(vector, zero_vector);
}
#[test]
fn null_vector() {
	let null_vector = Vector3D::null(Point3D::new(5.0, 5.0, 5.0));
	let vector = Vector3D::new(
		Point3D::new(
			5.0,
			5.0,
			5.0
		),
		Point3D::zero()
	);
	assert_eq!(vector, null_vector);
}
#[test]
fn with_absolute_target_vector() {
	let vector = Vector3D::new(
		Point3D::new(
			5.0,
			5.0,
			5.0
		),
		Point3D::new(
			5.0,
			5.0,
			5.0
		)
	);
	let absolute_target_vector = Vector3D::with_absolute_target(
		Point3D::new(
			5.0,
			5.0,
			5.0
		),
		Point3D::new(
			10.0,
			10.0,
			10.0
		)
	);
	assert_eq!(vector, absolute_target_vector);
}
#[test]
fn set_origin() {
	let set_origin_vector = Vector3D::zero()
		.set_origin(Point3D::new(1.0, 2.0, 3.0));
	let vector = Vector3D::new(
		Point3D::new(
			1.0,
			2.0,
			3.0
		),
		Point3D::new(
			0.0,
			0.0,
			0.0
		)
	);
	assert_eq!(vector, set_origin_vector);
}
#[test]
fn set_target() {
	let set_target_vector = Vector3D::zero()
		.set_target(Point3D::new(1.0, 2.0, 3.0));
	let vector = Vector3D::new(
		Point3D::new(
			0.0,
			0.0,
			0.0
		),
		Point3D::new(
			1.0,
			2.0,
			3.0
		)
	);
	assert_eq!(vector, set_target_vector);
}
#[test]
fn set_target_absolute() {
	let set_target_vector = Vector3D::null(Point3D::new(1.0, 5.0, 2.5))
		.set_target_absolute(Point3D::new(1.1, 2.2, 5.0));
	let vector = Vector3D::new(
		Point3D::new(
			1.0,
			5.0,
			2.5
		),
		Point3D::new(
			0.1,
			-2.8,
			2.5
		)
	);
	assert_eq!(vector, set_target_vector);
}
#[test]
fn display_vector() {
	let vector = Vector3D::new(
		Point3D::new(
			3.3,
			2.2,
			1.1
		),
		Point3D::new(
			1.1,
			2.2,
			3.3
		)
	);
	assert_eq!(format!("{vector}"), "(3.3,2.2,1.1)[1.1,2.2,3.3]");
}
#[test]
fn add_vectors() {
	let vector1 = Vector3D::new(
		Point3D::new(
			10.0,
			10.0,
			10.0
		),
		Point3D::new(
			10.0,
			5.0,
			5.0
		)
	);
	let vector2 = Vector3D::new(
		Point3D::new(
			10.0,
			10.0,
			10.0
		),
		Point3D::new(
			5.0,
			10.0,
			5.0
		)
	);
	let result_vector = Vector3D::new(
		Point3D::new(
			10.0,
			10.0,
			10.0
		),
		Point3D::new(
			15.0,
			15.0,
			10.0
		)
	);
	assert_eq!(vector1 + vector2, result_vector);
}
#[test]
fn addition() {
	let vector1 = Vector3D::new(
		Point3D::new(
			10.0,
			10.0,
			10.0
		),
		Point3D::new(
			10.0,
			5.0,
			5.0
		)
	);
	let vector2 = Vector3D::new(
		Point3D::new(
			10.0,
			10.0,
			10.0
		),
		Point3D::new(
			5.0,
			10.0,
			10.0
		)
	);
	assert_eq!(vector1 + vector2, vector1.add(vector2));
}
#[test]
fn sub_vectors() {
	let vector1 = Vector3D::new(
		Point3D::new(
			10.0,
			10.0,
			10.0
		),
		Point3D::new(
			10.0,
			5.0,
			5.0
		)
	);
	let vector2 = Vector3D::new(
		Point3D::new(
			10.0,
			10.0,
			10.0
		),
		Point3D::new(
			5.0,
			10.0,
			10.0
		)
	);
	let result_vector = Vector3D::new(
		Point3D::new(
			10.0,
			10.0,
			10.0
		),
		Point3D::new(
			5.0,
			-5.0,
			-5.0
		)
	);
	assert_eq!(vector1 - vector2, result_vector);
}
#[test]
fn subtraction() {
	let vector1 = Vector3D::new(
		Point3D::new(
			10.0,
			10.0,
			10.0
		),
		Point3D::new(
			10.0,
			5.0,
			5.0
		)
	);
	let vector2 = Vector3D::new(
		Point3D::new(
			10.0,
			10.0,
			10.0
		),
		Point3D::new(
			10.0,
			5.0,
			10.0
		)
	);
	assert_eq!(vector1 - vector2, vector1.sub(vector2));
}
#[test]
fn shift_positive() {
	let vector = Vector3D::new(
		Point3D::new(
			10.0,
			10.0,
			10.0
		),
		Point3D::new(
			10.0,
			5.0,
			5.0
		)
	);
	let shift = (2u16, 1.25, 1);
	let result_vector = Vector3D::new(
		Point3D::new(
			12.0,
			11.25,
			11.0
		),
		Point3D::new(
			10.0,
			5.0,
			5.0
		)
	);
	assert_eq!(vector.shift(shift), result_vector);
}
#[test]
fn shift_negative() {
	let vector = Vector3D::new(
		Point3D::new(
			10.0,
			10.0,
			10.0
		),
		Point3D::new(
			10.0,
			5.0,
			5.0
		)
	);
	let shift = (-2i16, -1.25, -1);
	let result_vector = Vector3D::new(
		Point3D::new(
			8.0,
			8.75,
			9.0
		),
		Point3D::new(
			10.0,
			5.0,
			5.0
		)
	);
	assert_eq!(vector.shift(shift), result_vector);
}
#[test]
fn shift_mixed() {
	let vector = Vector3D::new(
		Point3D::new(
			10.0,
			10.0,
			10.0
		),
		Point3D::new(
			10.0,
			5.0,
			5.0
		)
	);
	let shift = (-2i16, 1.25, 0);
	let result_vector = Vector3D::new(
		Point3D::new(
			8.0,
			11.25,
			10.0
		),
		Point3D::new(
			10.0,
			5.0,
			5.0
		)
	);
	assert_eq!(vector.shift(shift), result_vector);
}
#[test]
fn get_magnitude() {
	let error_margin = f64::EPSILON;
	let vector = Vector3D::new(
		Point3D::zero(),
		Point3D::new(
			3.0,
			4.0,
			0.0
		)
	);
	let magnitude = 5.0;
	assert!((vector.get_magnitude() - magnitude).abs() < error_margin);
}
#[test]
fn new_point() {
	let point = Point3D {
		x: 0.0,
		y: 1.1,
		z: 2.2
	};
	let new_point = Point3D::new(
		0.0,
		1.1,
		2.2
	);
	assert_eq!(point, new_point);
}
#[test]
fn zero_point() {
	let point = Point3D::new(
		0.0,
		0.0,
		0.0
	);
	let zero_point = Point3D::zero();
	assert_eq!(point, zero_point);
}
#[test]
fn point_distance() {
	let point1 = Point3D::new(
		0.0,
		0.0,
		0.0
	);
	let point2 = Point3D::new(
		1.0,
		0.0,
		0.0
	);
	let point3 = Point3D::new(
		0.0,
		1.0,
		0.0
	);
	let point4 = Point3D::new(
		0.0,
		0.0,
		1.0
	);
	let distance = 1.0;
	assert_eq!(point1.get_distance(point2), distance);
	assert_eq!(point1.get_distance(point3), distance);
	assert_eq!(point1.get_distance(point4), distance);
}
#[test]
fn to_vector3d() {
	let point1 = Point3D::new(
		0.0,
		0.0,
		0.0
	);
	let point2 = Point3D::new(
		1.2,
		2.2,
		1.5
	);
	let vector = Vector3D::new(
		Point3D::new(
			0.0,
			0.0,
			0.0
		),
		Point3D::new(
			1.2,
			2.2,
			1.5
		),
	);
	assert_eq!(point1.to_vector3d(&point2), vector);
}