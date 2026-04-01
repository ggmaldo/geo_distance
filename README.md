# geo_distance

Simple Rust crate to calculate distances between 2D points.

## Installation

```bash
cargo add geo_distance
```

## Basic usage

```rust
use geo_distance::Body;

fn main() {
    let a = Body::new(0.0, 0.0);
    let b = Body::new(3.0, 4.0);

    println!("Euclidean: {}", a.distance_to(&b));        // 5.0
    println!("Manhattan: {}", a.manhattan_distance_to(&b)); // 7.0
    println!("Chebyshev: {}", a.chebyshev_distance_to(&b)); // 4.0
}
```

## Available methods

- `distance_to()` - Euclidean distance
- `distance_squared_to()` - Distance squared (faster for comparisons)
- `manhattan_distance_to()` - Manhattan distance |dx| + |dy|
- `chebyshev_distance_to()` - Chebyshev distance max(|dx|, |dy|)

## License

MIT
