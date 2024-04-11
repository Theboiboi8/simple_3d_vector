# Simple 3D Vector
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/theboiboi8/simple_3d_vector?style=for-the-badge&labelColor=000000&color=dfff00)
[![Static Badge](https://img.shields.io/badge/available-crates.io-dfff00?style=for-the-badge&label=Available%20on&labelColor=000000&color=DFFF00)](https://crates.io/crates/simple_3d_vector)

Simple grid-based 3-dimensional vectors in Rust.

Also provides advanced operations for working with [`simple_2d_vector`](https://crates.io/crates/simple_2d_vector)

## Getting Started
To get started with `simple_3d_vector`, add it to your project using `cargo add simple_3d_vector`.

You can then use it by using the provided `Vector3D` struct.

## Examples

#### Creating a `Vector3D` with `Vector3D::new()` and comparing it to a `Vector3D::null()`
```rust
use vector3d::{Vector3D, Point3D};

fn main() {
    let vector = Vector3D::new(
        Point3D::zero(), // The origin of the vector
        Poine3D::zero()  // The target of the vector
    );
    
    // Null vectors are vectors with a length of zero
    // They are also called zero-length vectors as they only have an origin
    let null_vector = Vector3D::null(Point3D::new(0.0, 0.0, 0.0)); // A null vector
    
    assert_eq!(vector, null_vector); // The two vectors are the same
}
```

#### Performing addition and subtraction with vectors
```rust
use vector3d::{Vector3D, Point3D};

fn main() {
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

	let result_vector_addition = Vector3D::new(
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
	let result_vector_subtraction = Vector3D::new(
		Point3D::new(
			10.0,
			10.0,
			10.0
		),
		Point3D::new(
			5.0,
			-5.0,
			0.0
		)
	);
    
    assert_eq!(vector1 + vector2, result_vector_addition);
    assert_eq!(vector1 - vector2, result_vector_subtraction);
}
```

#### Shifting a vector

```rust
use vector3d::{Vector3D, Point3D};

fn main() { 
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
    
    // `Vector3D.shift` automatically converts applicable types into f64
    let shift = (-2, 1.25, 2_u16); // This allows for a mismatch of types
    
    // Shifting a vector moves only its `origin`,
    // as it's `target` is relative to its `origin`
    let result_vector = Vector3D::new(
	    Point3D::new(
		    8.0,
		    11.25,
		    12.0
	    ),
	    Point3D::new(
		    10.0,
		    5.0,
		    5.0
	    )
    );
    
    assert_eq!(vector.shift(shift), result_vector);
}
```
