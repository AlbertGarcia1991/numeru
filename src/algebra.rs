pub fn modulus(vector: Vec<f32>) -> f32 {
    let mut modulus: f32 = 0.;
    for elem in vector {
        modulus += elem.powf(2.);
    }
    modulus.sqrt()
}

pub fn norm(vector: Vec<f32>) -> Vec<f32> {
    let modulus: f32 = modulus(vector.clone());
    let vector_norm: Vec<f32> = vector.into_iter().map(|a: f32| a / modulus).collect();
    vector_norm
}

#[cfg(test)]
mod tests {
    use crate::algebra::*;

    #[test]
    fn test_modulus() {
        let two: f32 = 2.;
        let three: f32 = 3.;
        let thirty_six: f32 = 34.;
        assert_eq!(modulus(vec![1., -1.]), two.sqrt() / 1.);
        assert_eq!(modulus(vec![1., 1., 1.]), three.sqrt() / 1.);
        assert_eq!(modulus(vec![-5., 3.]), thirty_six.sqrt());
    }

    #[test]
    fn test_norm() {
        let twenty_nine: f32 = 29.;
        assert_eq!(norm(vec![1., 0.]), vec![1., 0.]);
        assert_eq!(norm(vec![-3., 4.]), vec![-3. / 5., 4. / 5.]);
        assert_eq!(
            norm(vec![2., 3., 4.]),
            vec![2. / twenty_nine.sqrt(), 3. / twenty_nine.sqrt(), 4. / twenty_nine.sqrt()]
        );
    }
}
