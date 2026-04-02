use geo_distance::Body;

fn main() {
    println!("This Program is a test for geo_distance v1");
    // Here we create a new point at 0,0
    let a: Body = Body::new(0.0, 0.0);
    // Here we create a new point at 4,6
    let b: Body = Body::new(4.0, 6.0);
    // Calculate the distance between A - B
    let distance = a.distance_to(&b);
    println!("{:.2}", distance);
    let squared_to = a.distance_squared_to(&b);
    println!("{}", squared_to);
}
