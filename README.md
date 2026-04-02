# geo_distance

Simple Rust crate to calculate distances between 2D points made by ggmaldo.

## Installation

```bash
cargo add geo_distance
```

### Structs:
For now (v1.3) we have 2 Structs which are:

```rust
Struct Body<T>:
    pub x: T
    pub y: T
```
and
```rust
Struct Vector<T>:
    x: T
    y: T
```

## Basic usage with Struct Body

```rust
use geo_distance::Body;

fn main() {
    let a = Body::<f32>::new(0.0, 0.0);
    let b = Body::<f32>::new(3.0, 4.0);

    println!("Euclidean: {}", a.distance_to(&b));       
    println!("Manhattan: {}", a.manhattan_distance_to(&b)); 
    println!("Chebyshev: {}", a.chebyshev_distance_to(&b)); 
    println!("Is a close to b: {}", a.is_close_to(&b));
}
```

## Available methods in Struct Body:

- `distance_to()` - Euclidean distance
- `distance_squared_to()` - Distance squared (faster for comparisons)
- `manhattan_distance_to()` - Manhattan distance |dx| + |dy|
- `chebyshev_distance_to()` - Chebyshev distance max(|dx|, |dy|)
- `is_close_to()` - Calculates if a point is close to another

## Basic usage with Struct Vector:
```rust
use geo_distance::Vector;

fn main() {
    let a = Vector::<f32>::new(0.0, 0.0);
    let b = Vector::<f32>::new(3.0, 4.0);

    println!("From Body: {}", a.from_body(&a, &b));       
    println!("Magnitude: {}", a.magnitude(&b)); 
    println!("{}", Vector::<f32>::new(6.0,4.0).magnitude(b));
}
```

# Supporting!
if you would like to support my work, just fork it and help me to make it better!

## License

MIT
