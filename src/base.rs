/// A N-dimensional tensor of generic data type
pub struct Tensor<T> {
    /// A flat vector that contains all the elements of the tensor. Rust's Vec<T> is a resizable array type that provides safe and efficient access to elements.
    data: Vec<T>,
    /// A vector of usize that represents the size of the tensor in each dimension. For a 2D tensor (matrix), the shape might be [rows, cols].
    shape: Vec<usize>,
    /// Number of dimensions, equal to the length of the shape vector.
    dims: usize,
    /// Strides are used to calculate the index of an element in the flat data vector based on its multi-dimensional indices. This is crucial for efficiently accessing and manipulating tensor elements.
    strides: Vec<usize>
}

impl<T> Tensor<T> {
    /// CONSTRUCTORS

    pub fn new(data: Vec<T>, shape: Vec<usize>) -> Self {
        // TODO: Do we actually require to pass the shape, or can be read from input data? 
        let strides = Self::compute_strides(&shape);
        let dims: usize = shape.len();
        return Tensor{ data, shape, dims, strides };
    }

    // TODO: Create constructos to ones, zeros, fill, random, eye, as well as the *_like versions

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
        let mut strides = Vec::with_capacity(shape.len());
        let mut stride = 1;
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
        let index = self.compute_flat_index(indices)?;
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

    // TODO: Pretty-printing
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_attributes() {

    }

    #[test]
    fn test_dims() {

    }

    #[test]
    fn test_get() {

    }
}