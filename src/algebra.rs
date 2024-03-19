fn modulus(vector: Vec<f32>) -> f32 {
    let mut modulus: f32 = 0.;
    for elem in vector {
        modulus += elem.powf(2.);
    }
    modulus.sqrt()
}

#[cfg(test)]
mod tests {
    use crate::algebra::modulus;

    #[test]
    fn test_modulus() {
        let two: f32 = 2.;
        let three: f32 = 3.;
        let thirty_six: f32 = 34.;
        assert_eq!(modulus(vec![1., -1.]), two.sqrt() / 1.);
        assert_eq!(modulus(vec![1., 1., 1.]), three.sqrt() / 1.);
        assert_eq!(modulus(vec![-5., 3.]), thirty_six.sqrt());
    }
}