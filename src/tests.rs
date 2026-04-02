#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let a = Body::<f32>::new(0.0, 0.0);
        let b = Body::<f64>::new(3.0, 4.0);

        assert_eq!(a.distance_squared_to(&b), 25.0);
        assert_eq!(a.manhattan_distance_to(&b), 7.0);
        assert_eq!(a.chebyshev_distance_to(&b), 4.0);
        assert_eq!(a.distance_to(&b), 5.0);
        assert_eq!(a.is_close_to(&b, f32::EPILSON));
    }
}
