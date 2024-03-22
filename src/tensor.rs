
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Tensor {
    /// A flat vector that contains all the elements of the tensor
    pub data: Vec<f32>,
    /// A vector of usize that represents the size of the tensor in each dimension. For a 2D tensor (matrix), the shape might be [rows, cols].
    pub shape: Vec<usize>,
    /// Strides are used to calculate the index of an element in the flat data vector based on its multi-dimensional indices. This is crucial for efficiently accessing and manipulating tensor elements.
    pub strides: Vec<usize>,
    /// Row-major length is the length of the flattened tensor
    pub row_major_length: usize,
}

pub struct TensorMask {
    /// A flat vector that contains all the Boolean elements of the tensor
    pub data: Vec<bool>,
    /// A vector of usize that represents the size of the tensor in each dimension. For a 2D tensor (matrix), the shape might be [rows, cols].
    pub shape: Vec<usize>,
    /// Strides are used to calculate the index of an element in the flat data vector based on its multi-dimensional indices. This is crucial for efficiently accessing and manipulating tensor elements.
    pub strides: Vec<usize>,
    /// Row-major length is the length of the flattened tensor
    pub row_major_length: usize,
}

impl Tensor {
    pub fn new(data: Vec<f32>, shape: Vec<usize>) -> Self {
        let row_major_length: usize = shape.iter().product();
        if data.len() != row_major_length {
            panic!("Data length does not match the product of the shape dimensions");
        }
        let strides: Vec<usize> = Self::compute_strides(&shape);
        Tensor {
            data,
            shape,
            strides,
            row_major_length,
        }
    }

    // TODO: New from array

    pub fn ones(shape: Vec<usize>) -> Self {
        let row_major_length: usize = shape.iter().product();
        let data: Vec<f32> = vec![1.; row_major_length];
        let strides: Vec<usize> = Self::compute_strides(&shape);
        Tensor {
            data,
            shape,
            strides,
            row_major_length,
        }
    }

    pub fn zeros(shape: Vec<usize>) -> Self {
        let row_major_length: usize = shape.iter().product();
        let data: Vec<f32> = vec![0.; row_major_length];
        let strides: Vec<usize> = Self::compute_strides(&shape);
        Tensor {
            data,
            shape,
            strides,
            row_major_length,
        }
    }

    pub fn fill(shape: Vec<usize>, fill_value: f32) -> Self {
        let row_major_length: usize = shape.iter().product();
        let data: Vec<f32> = vec![fill_value; row_major_length];
        let strides: Vec<usize> = Self::compute_strides(&shape);
        Tensor {
            data,
            shape,
            strides,
            row_major_length,
        }
    }

    pub fn random(shape: Vec<usize>, min_bound: Option<f32>, max_bound: Option<f32>) -> Self {
        let row_major_length: usize = shape.iter().product();
        let mut data: Vec<f32> = Vec::with_capacity(row_major_length);
        let min_bound: f32 = min_bound.unwrap_or(0.);
        let max_bound: f32 = max_bound.unwrap_or(1.);
        if min_bound >= max_bound {
            panic!("When generating random filled tensors, the min bound must be smaller than max bound");
        }
        loop {
            data.push(rand::thread_rng().gen_range(min_bound..=max_bound));
            if data.len() == row_major_length {
                break;
            }
        }
        let strides: Vec<usize> = Self::compute_strides(&shape);
        Tensor {
            data,
            shape,
            strides,
            row_major_length,
        }
    }

    pub fn eye(shape: Vec<usize>, shift: Option<usize>) -> Self {
        let row_major_length: usize = shape.iter().product();
        let shift: usize = shift.unwrap_or(0);
        if shift >= row_major_length {
            panic!("Shift if bigger than the row-major length of the given array shape");
        }
        let mut data: Vec<f32> = vec![0.; row_major_length];
        let strides: Vec<usize> = Self::compute_strides(&shape);
        let mut i: usize = shift;
        while i < row_major_length {
            data[i] = 1.;
            if shape.len() == 1 {
                break;
            }
            let step: usize = strides.iter().sum();
            i += step;
        }
        Tensor {
            data,
            shape,
            strides,
            row_major_length,
        }
    }

    pub fn ones_like(tensor: &Self) -> Self {
        let shape: Vec<usize> = tensor.shape.to_vec();
        let row_major_length: usize = shape.iter().product();
        let data: Vec<f32> = vec![1.; row_major_length];
        let strides: Vec<usize> = Self::compute_strides(&shape);
        Tensor {
            data,
            shape,
            strides,
            row_major_length,
        }
    }

    pub fn zeros_like(tensor: &Self) -> Self {
        let shape: Vec<usize> = tensor.shape.to_vec();
        let row_major_length: usize = shape.iter().product();
        let data: Vec<f32> = vec![0.; row_major_length];
        let strides: Vec<usize> = Self::compute_strides(&shape);
        Tensor {
            data,
            shape,
            strides,
            row_major_length,
        }
    }

    pub fn fill_like(tensor: &Self, fill_value: f32) -> Self {
        let shape: Vec<usize> = tensor.shape.to_vec();
        let row_major_length: usize = shape.iter().product();
        let data: Vec<f32> = vec![fill_value; row_major_length];
        let strides: Vec<usize> = Self::compute_strides(&shape);
        Tensor {
            data,
            shape,
            strides,
            row_major_length,
        }
    }

    pub fn random_like(tensor: &Self, min_bound: Option<f32>, max_bound: Option<f32>) -> Self {
        let shape: Vec<usize> = tensor.shape.to_vec();
        let row_major_length: usize = shape.iter().product();
        let mut data: Vec<f32> = Vec::with_capacity(row_major_length);
        let min_bound: f32 = min_bound.unwrap_or(0.);
        let max_bound: f32 = max_bound.unwrap_or(1.);
        if min_bound >= max_bound {
            panic!("When generating random fille tensors, the min bound must be smaller than max bound");
        }
        loop {
            data.push(rand::thread_rng().gen_range(min_bound..=max_bound));
            if data.len() == row_major_length {
                break;
            }
        }
        let strides: Vec<usize> = Self::compute_strides(&shape);
        Tensor {
            data,
            shape,
            strides,
            row_major_length,
        }
    }

    fn get(&self, idx: &[usize]) -> f32 {
        let length_idx: usize = idx.len();
        if self.shape.len() != length_idx {
            panic!("Cannot access to the request index, no matching dimensions");
        }
        let flat_idx: usize = idx
            .iter()
            .zip(self.strides.iter())
            .map(|(x, y)| x * y)
            .sum();
        self.data[flat_idx]
    }

    fn len(self) -> usize {
        self.shape[0]
    }

    fn new_transpose(self) -> Self {
        let mut new_shape: Vec<usize> = self.shape.clone();
        new_shape.reverse();
        Tensor::new(self.data, new_shape)
    }

    fn transpose(&mut self) {
        self.shape.reverse();
        self.strides = Self::compute_strides(&self.shape);
    }

    fn reshape(mut self, new_shape: Vec<usize>) {
        let target_row_major_length: usize = new_shape.iter().product();
        if self.row_major_length != target_row_major_length {
            panic!("Cannot reshape the Tensor, shapes not compatible");
        }
        self.strides = new_shape;
    }

    pub fn compute_strides(shape: &Vec<usize>) -> Vec<usize> {
        let mut strides: Vec<usize> = Vec::with_capacity(shape.len());
        let mut stride: usize = 1;
        for &dimension in shape.iter().rev() {
            strides.push(stride);
            stride *= dimension;
        }
        strides.reverse();
        strides
    }
}

mod tensor_overridings {
    use super::Tensor;
    use std::ops;

    impl std::fmt::Display for Tensor {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?} (Shape: {:?})", self.data, self.shape)
        }
    }

    impl ops::Add<Tensor> for Tensor {
        type Output = Self;
        fn add(self, _rhs: Tensor) -> Self::Output {
            if self.shape != _rhs.shape {
                panic!("Shapes do not match for addition");
            }
            let data: Vec<f32> = self
                .data
                .iter()
                .zip(_rhs.data.iter())
                .map(|(a, b)| a + b)
                .collect();
            Tensor::new(data, self.shape)
        }
    }

    impl ops::Add<f32> for Tensor {
        type Output = Self;
        fn add(self, _rhs: f32) -> Self::Output {
            let data: Vec<f32> = self.data.iter().map(|&x| x + _rhs).collect();
            Tensor::new(data, self.shape)
        }
    }

    impl ops::AddAssign<Tensor> for Tensor {
        fn add_assign(&mut self, _rhs: Self) {
            if self.shape != _rhs.shape {
                panic!("Shapes do not match for addition");
            }
            for (a, b) in self.data.iter_mut().zip(_rhs.data.iter()) {
                *a += b;
            }
        }
    }

    impl ops::AddAssign<f32> for Tensor {
        fn add_assign(&mut self, _rhs: f32) {
            for a in self.data.iter_mut() {
                *a += _rhs;
            }
        }
    }

    impl ops::Sub<Tensor> for Tensor {
        type Output = Self;
        fn sub(self, _rhs: Tensor) -> Self::Output {
            if self.shape != _rhs.shape {
                panic!("Shapes do not match for subtraction");
            }
            let data: Vec<f32> = self
                .data
                .iter()
                .zip(_rhs.data.iter())
                .map(|(a, b)| a - b)
                .collect();
            Tensor::new(data, self.shape)
        }
    }

    impl ops::Sub<f32> for Tensor {
        type Output = Self;
        fn sub(self, _rhs: f32) -> Self::Output {
            let data = self.data.iter().map(|&x| x - _rhs).collect();
            Tensor::new(data, self.shape)
        }
    }

    impl ops::SubAssign<Tensor> for Tensor {
        fn sub_assign(&mut self, _rhs: Self) {
            if self.shape != _rhs.shape {
                panic!("Shapes do not match for subtraction");
            }
            for (a, b) in self.data.iter_mut().zip(_rhs.data.iter()) {
                *a -= b;
            }
        }
    }

    impl ops::SubAssign<f32> for Tensor {
        fn sub_assign(&mut self, _rhs: f32) {
            for a in self.data.iter_mut() {
                *a -= _rhs;
            }
        }
    }

    impl ops::Mul<Tensor> for Tensor {
        type Output = Self;
        fn mul(self, _rhs: Tensor) -> Self::Output {
            if self.shape != _rhs.shape {
                panic!("Shapes do not match for multiplication");
            }
            let data: Vec<f32> = self
                .data
                .iter()
                .zip(_rhs.data.iter())
                .map(|(a, b)| a * b)
                .collect();
            Tensor::new(data, self.shape)
        }
    }

    impl ops::Mul<f32> for Tensor {
        type Output = Self;
        fn mul(self, _rhs: f32) -> Self::Output {
            let data = self.data.iter().map(|&x| x * _rhs).collect();
            Tensor::new(data, self.shape)
        }
    }

    impl ops::MulAssign<Tensor> for Tensor {
        fn mul_assign(&mut self, _rhs: Self) {
            if self.shape != _rhs.shape {
                panic!("Shapes do not match for multiplication");
            }
            for (a, b) in self.data.iter_mut().zip(_rhs.data.iter()) {
                *a *= b;
            }
        }
    }

    impl ops::MulAssign<f32> for Tensor {
        fn mul_assign(&mut self, _rhs: f32) {
            for a in self.data.iter_mut() {
                *a *= _rhs;
            }
        }
    }

    impl ops::Div<Tensor> for Tensor {
        type Output = Self;
        fn div(self, _rhs: Tensor) -> Self::Output {
            if self.shape != _rhs.shape {
                panic!("Shapes do not match for multiplication");
            }
            let data: Vec<f32> = self
                .data
                .iter()
                .zip(_rhs.data.iter())
                .map(|(a, b)| a / b)
                .collect();
            Tensor::new(data, self.shape)
        }
    }

    impl ops::Div<f32> for Tensor {
        type Output = Self;
        fn div(self, _rhs: f32) -> Self::Output {
            let data = self.data.iter().map(|&x| x / _rhs).collect();
            Tensor::new(data, self.shape)
        }
    }

    impl ops::DivAssign<Tensor> for Tensor {
        fn div_assign(&mut self, _rhs: Self) {
            if self.shape != _rhs.shape {
                panic!("Shapes do not match for multiplication");
            }
            for (a, b) in self.data.iter_mut().zip(_rhs.data.iter()) {
                *a /= b;
            }
        }
    }

    impl ops::DivAssign<f32> for Tensor {
        fn div_assign(&mut self, _rhs: f32) {
            for a in self.data.iter_mut() {
                *a /= _rhs;
            }
        }
    }

    impl ops::BitXor<f32> for Tensor {
        type Output = Self;
        fn bitxor(self, _rhs: f32) -> Self::Output {
            let data: Vec<f32> = self.data.iter().map(|&x| x.powf(_rhs)).collect();
            Tensor::new(data, self.shape)
        }
    }

    impl ops::BitXorAssign<f32> for Tensor {
        fn bitxor_assign(&mut self, _rhs: f32) {
            for a in self.data.iter_mut() {
                *a = a.powf(_rhs);
            }
        }
    }

    impl ops::Neg for Tensor {
        type Output = Self;
        fn neg(self) -> Self::Output {
            let data: Vec<f32> = self.data.iter().map(|&x| -x).collect();
            Tensor::new(data, self.shape)
        }
    }
}

// TODO: Comparisons -> Mask
// TODO: Override ^ between tensors as dot product
// TODO: Override % between tensors as scalar product
// TODO: Override & between tensors as angle for each dimension
// TODO: Implement fn modulus (how to handle dimensions ?)
// TODO: Override ! unary tensors as normalization (how to handle dimensions ?)
// TODO: Replace panic! by Result<Self, Error>

#[cfg(test)]
mod tests {
    use super::*;

    fn get_dummy_tensor_from_new() -> Tensor {
        let data: Vec<f32> = vec![1., 2., 3., 4., 5., 6.];
        let shape: Vec<usize> = vec![2, 3];
        let tensor: Tensor = Tensor::new(data, shape);
        tensor
    }

    #[test]
    fn test_new_attributes() {
        let tensor: Tensor = get_dummy_tensor_from_new();
        assert_eq!(tensor.shape, vec![2, 3]);
        assert_eq!(tensor.strides, vec![3, 1]);
        assert_eq!(tensor.row_major_length, 6);
    }

    #[test]
    fn test_new_ones() {
        let tensor: Tensor = Tensor::ones(vec![3]);
        assert_eq!(tensor.data, vec![1.; 3]);
        let tensor: Tensor = Tensor::ones(vec![3, 2]);
        assert_eq!(tensor.data, vec![1.; 6]);
    }

    #[test]
    fn test_new_zeros() {
        let tensor: Tensor = Tensor::zeros(vec![3]);
        assert_eq!(tensor.data, vec![0.; 3]);
        let tensor: Tensor = Tensor::zeros(vec![3, 2]);
        assert_eq!(tensor.data, vec![0.; 6]);
    }

    #[test]
    fn test_new_fill() {
        let tensor: Tensor = Tensor::fill(vec![3], 4.5);
        assert_eq!(tensor.data, vec![4.5; 3]);
        let tensor: Tensor = Tensor::fill(vec![3], -2.);
        assert_eq!(tensor.data, vec![-2.; 3]);
    }

    #[test]
    fn test_new_eye() {
        let tensor: Tensor = Tensor::eye(vec![3], None);
        assert_eq!(tensor.data, vec![1., 0., 0.]);
        let tensor: Tensor = Tensor::eye(vec![2, 3], None);
        assert_eq!(tensor.data, vec![1., 0., 0., 0., 1., 0.]);
        let tensor: Tensor = Tensor::eye(vec![3, 3], None);
        assert_eq!(tensor.data, vec![1., 0., 0., 0., 1., 0., 0., 0., 1.]);
        let tensor: Tensor = Tensor::eye(vec![3, 2], None);
        assert_eq!(tensor.data, vec![1., 0., 0., 1., 0., 0.]);
        let tensor: Tensor = Tensor::eye(vec![2, 4], None);
        assert_eq!(tensor.data, vec![1., 0., 0., 0., 0., 1., 0., 0.]);
    }

    #[test]
    fn test_new_eye_shift() {
        let tensor: Tensor = Tensor::eye(vec![3], Some(1));
        assert_eq!(tensor.data, vec![0., 1., 0.]);
        let tensor: Tensor = Tensor::eye(vec![2, 3], Some(2));
        assert_eq!(tensor.data, vec![0., 0., 1., 0., 0., 0.]);
        let tensor: Tensor = Tensor::eye(vec![3, 3], Some(1));
        assert_eq!(tensor.data, vec![0., 1., 0., 0., 0., 1., 0., 0., 0.]);
        let tensor: Tensor = Tensor::eye(vec![3, 2], Some(2));
        assert_eq!(tensor.data, vec![0., 0., 1., 0., 0., 1.]);
        let tensor: Tensor = Tensor::eye(vec![2, 4], Some(3));
        assert_eq!(tensor.data, vec![0., 0., 0., 1., 0., 0., 0., 0.]);
    }

    #[test]
    fn test_new_random() {
        let tensor: Tensor = Tensor::random(vec![1000], Some(0.5), None);
        tensor
            .data
            .iter()
            .for_each(|x: &f32| assert!(*x >= 0.5 && *x <= 1.));
    }

    #[test]
    fn test_new_ones_like() {
        let tensor: Tensor = Tensor::ones_like(&Tensor::zeros(vec![10]));
        assert_eq!(tensor.data, vec![1.; 10]);
        let tensor: Tensor = Tensor::ones_like(&Tensor::zeros(vec![6]));
        assert_eq!(tensor.data, vec![1.; 6]);
    }

    #[test]
    fn test_new_zeros_like() {
        let tensor_dummy: Tensor = get_dummy_tensor_from_new();
        let tensor: Tensor = Tensor::zeros_like(&tensor_dummy);
        assert_eq!(tensor.data, vec![0.; 6]);
        let tensor: Tensor = Tensor::zeros_like(&tensor_dummy);
        assert_eq!(tensor.data, vec![0.; 6]);
    }

    #[test]
    fn test_new_fill_like() {
        let tensor_dummy: Tensor = get_dummy_tensor_from_new();
        let tensor: Tensor = Tensor::fill_like(&tensor_dummy, 4.5);
        assert_eq!(tensor.data, vec![4.5; 6]);
        let tensor: Tensor = Tensor::fill_like(&tensor_dummy, -2.);
        assert_eq!(tensor.data, vec![-2.; 6]);
    }

    #[test]
    fn test_get() {
        let tensor_dummy: Tensor = get_dummy_tensor_from_new();
        assert_eq!(tensor_dummy.get(&[0, 0]), 1.);
        assert_eq!(tensor_dummy.get(&[0, 1]), 2.);
        assert_eq!(tensor_dummy.get(&[0, 2]), 3.);
        assert_eq!(tensor_dummy.get(&[1, 0]), 4.);
        assert_eq!(tensor_dummy.get(&[1, 1]), 5.);
        assert_eq!(tensor_dummy.get(&[1, 2]), 6.);
    }

    #[test]
    fn test_len() {
        let tensor_dummy: Tensor = get_dummy_tensor_from_new();
        assert_eq!(tensor_dummy.len(), 2);
    }

    #[test]
    fn test_transpose() {
        let mut tensor: Tensor = get_dummy_tensor_from_new();
        tensor.transpose();
        assert_eq!(tensor.shape, vec![3, 2]);
        assert_eq!(tensor.strides, vec![2, 1]);
    }

    #[test]
    fn test_new_transpose() {
        let tensor: Tensor = get_dummy_tensor_from_new();
        let reversed_tensor: Tensor = tensor.new_transpose();
        assert_eq!(reversed_tensor.shape, vec![3, 2]);
        assert_eq!(reversed_tensor.strides, vec![2, 1]);
    }

    #[test]
    fn test_neg() {
        let tensor1: Tensor = -get_dummy_tensor_from_new();
        assert_eq!(tensor1.data, vec![-1., -2., -3., -4., -5., -6.]);
    }

    #[test]
    fn test_add_tensor() {
        let tensor1: Tensor = get_dummy_tensor_from_new();
        let tensor2: Tensor = get_dummy_tensor_from_new();
        let tensor3: Tensor = tensor1 + tensor2;
        assert_eq!(tensor3.data, vec![2., 4., 6., 8., 10., 12.]);
    }

    #[test]
    fn test_add_scalar() {
        let tensor1: Tensor = get_dummy_tensor_from_new();
        let tensor2: Tensor = tensor1 + 1.;
        assert_eq!(tensor2.data, vec![2., 3., 4., 5., 6., 7.]);
    }

    #[test]
    fn test_add_tensor_inplace() {
        let mut tensor1: Tensor = get_dummy_tensor_from_new();
        let tensor2: Tensor = get_dummy_tensor_from_new();
        tensor1 += tensor2;
        assert_eq!(tensor1.data, vec![2., 4., 6., 8., 10., 12.]);
    }

    #[test]
    fn test_add_scalar_inplace() {
        let mut tensor1: Tensor = get_dummy_tensor_from_new();
        tensor1 += 1.;
        assert_eq!(tensor1.data, vec![2., 3., 4., 5., 6., 7.]);
    }

    #[test]
    fn test_print() {
        // TODO: Add strides and shape to print as N-Dimensional array
        let tensor: Tensor = get_dummy_tensor_from_new();
        println!("{}", tensor);
    }
}
