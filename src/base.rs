use rand::Rng;

#[derive(Debug, Clone)]
pub struct Tensor
{
    /// A flat vector that contains all the elements of the tensor 
    pub data: Vec<f32>,
    /// A vector of usize that represents the size of the tensor in each dimension. For a 2D tensor (matrix), the shape might be [rows, cols].
    pub shape: Vec<usize>,
    /// Strides are used to calculate the index of an element in the flat data vector based on its multi-dimensional indices. This is crucial for efficiently accessing and manipulating tensor elements.
    pub strides: Vec<usize>,
    /// Row-major length is the length of the flattened tensor
    pub row_major_length: usize
}

impl Tensor
{
    pub fn new(data: Vec<f32>, shape: Vec<usize>) -> Self {
        let row_major_length: usize = shape.iter().product();
        if data.len() != row_major_length {
            panic!("Data length does not match the product of the shape dimensions");
        }
        let strides: Vec<usize> = Self::compute_strides(&shape);
        Tensor{ data, shape, strides, row_major_length }
    }

    pub fn from_array(array: &[f32], shape: Vec<usize>) -> Self {
        let row_major_length: usize = shape.iter().product();
        if array.len() != row_major_length {
            panic!("Data length does not match the product of the shape dimensions");
        }
        let strides: Vec<usize> = Self::compute_strides(&shape);
        Tensor{ data, shape, strides, row_major_length }
    }

    pub fn ones(shape: Vec<usize>) -> Self {
        let row_major_length: usize = shape.iter().product();
        let data: Vec<f32> = vec![1.; row_major_length];
        let strides: Vec<usize> = Self::compute_strides(&shape);
        Tensor{ data, shape, strides, row_major_length }
    }

    pub fn zeros(shape: Vec<usize>) -> Self {
        let row_major_length: usize = shape.iter().product();
        let data: Vec<f32> = vec![0.; row_major_length];
        let strides: Vec<usize> = Self::compute_strides(&shape);
        Tensor{ data, shape, strides, row_major_length }
    }

    pub fn random(shape: Vec<usize>, min_bound: Option<f32>, max_bound: Option<f32>) -> Self {
        let row_major_length: usize = shape.iter().product();
        let mut data: Vec<f32> = Vec::with_capacity(row_major_length);
        let min_bound: f32 = min_bound.unwrap_or(0.);
        let max_bound: f32 = max_bound.unwrap_or(1.);
        if min_bound <= max_bound {
            panic!("When generating random fille tensors, the min bound must be smaller than max bound");
        }
        loop {
            data.push(rand::thread_rng().gen_range(min_bound..=max_bound));
            if (data.len() == row_major_length) {
                break
            }
        }
        let strides: Vec<usize> = Self::compute_strides(&shape);
        Tensor{ data, shape, strides, row_major_length }
    }
    
    pub fn eye(shape: Vec<usize>) -> Self {
        let row_major_length: usize = shape.iter().product();
        let mut data: Vec<f32> = vec![0.; row_major_length];
        let strides: Vec<usize> = Self::compute_strides(&shape);
        // TODO: Create constructors to eye, as well as the *_like versions
        Tensor{ data, shape, strides, row_major_length }
    }

    pub fn new_like(data: Vec<f32>, tensor: Tensor) -> Self {
        let shape: Vec<usize> = tensor.shape;
        let row_major_length: usize = shape.iter().product();
        if data.len() != row_major_length {
            panic!("Data length does not match the product of the shape dimensions");
        }
        let strides: Vec<usize> = Self::compute_strides(&shape);
        Tensor{ data, shape, strides, row_major_length }
    }

    pub fn ones_like(tensor: Tensor) -> Self {
        let shape: Vec<usize> = tensor.shape;
        let row_major_length: usize = shape.iter().product();
        let data: Vec<f32> = vec![1.; row_major_length];
        let strides: Vec<usize> = Self::compute_strides(&shape);
        Tensor{ data, shape, strides, row_major_length }
    }

    pub fn zeros_like(tensor: Tensor) -> Self {
        let shape: Vec<usize> = tensor.shape;
        let row_major_length: usize = shape.iter().product();
        let data: Vec<f32> = vec![0.; row_major_length];
        let strides: Vec<usize> = Self::compute_strides(&shape);
        Tensor{ data, shape, strides, row_major_length }
    }

    pub fn random_like(tensor: Tensor, min_bound: Option<f32>, max_bound: Option<f32>) -> Self {
        let shape: Vec<usize> = tensor.shape;
        let row_major_length: usize = shape.iter().product();
        let mut data: Vec<f32> = Vec::with_capacity(row_major_length);
        let min_bound: f32 = min_bound.unwrap_or(0.);
        let max_bound: f32 = max_bound.unwrap_or(1.);
        if min_bound <= max_bound {
            panic!("When generating random fille tensors, the min bound must be smaller than max bound");
        }
        loop {
            data.push(rand::thread_rng().gen_range(min_bound..=max_bound));
            if (data.len() == row_major_length) {
                break
            }
        }
        let strides: Vec<usize> = Self::compute_strides(&shape);
        Tensor{ data, shape, strides, row_major_length }
    }

    fn compute_strides(shape: &Vec<usize>) -> Vec<usize> {
        let mut strides: Vec<usize> = Vec::with_capacity(shape.len());
        let mut stride: usize = 1;
        for &dimension in shape.iter().rev() {
            strides.push(stride);
            stride *= dimension;
        }
        strides.reverse();
        return strides;
    }

    // TODO: Overload index accessor https://stackoverflow.com/questions/49593793/is-there-a-way-to-overload-the-index-assignment-operator

    fn get(&self, indices: &[usize]) -> Option<&T> {
        let index: usize = self.compute_flat_index(indices)?;
        return self.data.get(index);
    }

    fn compute_flat_index(&self, indices: &[usize]) -> Option<usize> {
        if indices.len() != self.shape.len() {
            return None;
        }
        
        let mut flat_index = 0;
        for (i, &idx) in indices.iter().enumerate() {
            if idx >= self.shape[i] {
                return None;
            }
            flat_index += self.strides[i] * idx;
        }

        return Some(flat_index);
    }

    // OVERLOADING
    // TODO: Pretty-printing
    // TODO: Aritmetic operations
    // TODO: Comparisons
}

// TODO: Modulus and other metrics as a trait

#[cfg(test)]
mod tests {
    use super::*;

    fn get_dummy_tensor_from_new() -> Tensor {
        let data: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
        let shape: Vec<usize> = vec![2, 3];
        let tensor: Tensor<i32> = Tensor::new(data, shape);
        tensor
    }

    #[test]
    fn test_new_attributes() {
        let tensor: Tensor<i32> = get_dummy_tensor_from_new();
        assert_eq!(tensor.shape, vec![2, 3]);
        assert_eq!(tensor.strides, vec![3, 1]);
    }
}