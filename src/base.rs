/// A N-dimensional tensor of generic data type
#[derive(Debug, Clone)]
pub struct Tensor<T>
{
    /// A flat vector that contains all the elements of the tensor. Rust's Vec<T> is a resizable array type that provides safe and efficient access to elements.
    pub data: Vec<T>,
    /// A vector of usize that represents the size of the tensor in each dimension. For a 2D tensor (matrix), the shape might be [rows, cols].
    pub shape: Vec<usize>,
    /// Strides are used to calculate the index of an element in the flat data vector based on its multi-dimensional indices. This is crucial for efficiently accessing and manipulating tensor elements.
    pub strides: Vec<usize>,
    /// Row-major length is the length of the flattened tensor
    pub row_major_length: usize
}

impl<T> Tensor<T> where T: From<u8> + Copy
{

    // TODO: Use vec versus array

    pub fn new(data: Vec<T>, shape: Vec<usize>) -> Self {
        // TODO: Do we actually require to pass the shape, or can be read from input data? 
        if data.len() != shape.iter().product() {
            panic!("Data length does not match the product of the shape dimensions");
        }
        let strides: Vec<usize> = Self::compute_strides(&shape);
        let row_major_length: usize = shape.iter().product();
        Tensor{ data, shape, strides, row_major_length }
    }

    pub fn ones(shape: Vec<usize>) -> Self {
        let row_major_length: usize = shape.iter().product();
        let data: Vec<T> = vec![T::from(1); row_major_length];
        let strides: Vec<usize> = Self::compute_strides(&shape);
        Tensor{ data, shape, strides, row_major_length }
    }

    pub fn zeros(shape: Vec<usize>) -> Self {
        let row_major_length: usize = shape.iter().product();
        let data: Vec<T> = vec![T::from(0); row_major_length];
        let strides: Vec<usize> = Self::compute_strides(&shape);
        Tensor{ data, shape, strides, row_major_length }
    }

    // TODO: Create constructors to eye, as well as the *_like versions

    /// Strides are pivotal in efficiency accessing elements in a multi-dimensional tensor when it is stored in a linear memory space.
    /// Strides represents the "step" needed to move along each dimension of the tensor. In the concept of the Tensor, strides is a vector
    /// where each element corresponds to the number of elements you need to skip in the flat data array to move one unit along a particular
    /// dimension in the tensor. This function iterates over the shape in reverse, starting from the innermost dimension (assuming row-major 
    /// order).  It initializes the stride for the innermost dimension to 1, as moving one element along the innermost dimension equates to 
    /// moving one element in the flat array. For each subsequent (outer) dimension, it multiplies the stride of the previous (inner) dimension 
    /// by the size of the current dimension. This process accumulates the total number of elements that need to be skipped in the flat array 
    /// to move one unit along each dimension.
    ///
    /// # example
    /// Consider the 2D tensor with shape [2, 4] but row-major order [1, 2, 3, 4, 5, 6, 7, 8]
    /// In 2D, that tensor is represented as:
    ///  [1, 2, 3, 4]
    ///  [5, 6, 7, 8]
    /// Starting at the first element, 1, to move to the next one in a row, we need to move 1 position on the row-jamor order. However,
    /// to move to the next one in a column, we need to move 4 positions in the row-major order vector. That means that the strides for
    /// this tensor are [4, 1]
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
    // If possible, add test about that

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

    fn get_dummy_tensor_from_new() -> Tensor<i32> {
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
    
    #[test]
    fn test_ones() {        
        let tensor: Tensor<i32> = Tensor::ones(vec![2, 2]);
        assert_eq!(tensor.shape, vec![2, 2]);
        assert_eq!(tensor.strides, vec![2, 1]);
        for idx in 0..tensor.row_major_length {
            assert_eq!(tensor.data[idx], 1);
        }
    }

    #[test]
    fn test_zeros() {        
        let tensor: Tensor<i32> = Tensor::zeros(vec![2, 2]);
        assert_eq!(tensor.shape, vec![2, 2]);
        assert_eq!(tensor.strides, vec![2, 1]);
        for idx in 0..tensor.row_major_length {
            assert_eq!(tensor.data[idx], 0);
        }
    }
}