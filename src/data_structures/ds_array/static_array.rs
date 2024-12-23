use std::ops::{Index, IndexMut};

/// We define an static array as a data structure consisting of a collection of elements (values or
/// variables), of same memory size, each identified by at least one array index or key. The key
/// property that defines the array as static is the fact of having a fixed length (or size) defined
/// when is created, whether or not the elements inside are immutable.
#[derive(Debug)]
struct StaticArray {
    capacity: usize,
    data: Vec<f32>,
    shape: Vec<usize>,
}

impl StaticArray {
    fn _new_array_with_value(_value: f32, _shape: Vec<usize>) -> Self {
        let capacity: usize = _shape.iter().product();
        let data: Vec<f32> = vec![_value; capacity];
        StaticArray {
            capacity,
            data,
            shape: _shape,
        }
    }

    fn _check_shape_capacity_match(_capacity: &usize, _shape: &[usize]) -> bool {
        let shape_product: usize = _shape.iter().product();
        if shape_product != *_capacity {
            panic!(
                "The given input array and shape does not match in capacity: {} != {}",
                shape_product, *_capacity
            );
        }
        true
    }

    fn _check_index_within_bounds(&self, _access_index: &[usize]) -> bool {
        if _access_index.len() != self.shape.len() {
            panic!("Number of indices does not match the shape dimensions");
        }
        for (i, &index) in _access_index.iter().enumerate() {
            if index >= self.shape[i] {
                panic!(
                    "Index out of bounds for dimension {}: {} >= {}",
                    i, index, self.shape[i]
                );
            }
        }
        true
    }

    fn _calculate_strides(&self) -> Vec<usize> {
        let mut strides: Vec<usize> = vec![1; self.shape.len()];
        for i in (0..self.shape.len() - 1).rev() {
            strides[i] = strides[i + 1] * self.shape[i + 1];
        }
        strides
    }

    fn _check_end_index_greater_than_start_index(
        &self,
        _start_index: &[usize],
        _end_index: &[usize],
    ) -> bool {
        if self._calculate_flat_index(_start_index) <= self._calculate_flat_index(_end_index) {
            true
        } else {
            panic!("The start index is greater than the end index");
        }
    }

    fn _calculate_flat_index(&self, indices: &[usize]) -> usize {
        let mut flat_index = 0;
        let mut stride = 1;
        for (i, &index) in indices.iter().rev().enumerate() {
            flat_index += index * stride;
            stride *= self.shape[self.shape.len() - 1 - i];
        }
        flat_index
    }

    pub fn new_zeros(shape: Vec<usize>) -> Self {
        Self::_new_array_with_value(0., shape)
    }

    pub fn new_ones(shape: Vec<usize>) -> Self {
        Self::_new_array_with_value(1., shape)
    }

    pub fn new_fill(shape: Vec<usize>, fill_value: f32) -> Self {
        Self::_new_array_with_value(fill_value, shape)
    }

    pub fn new_from_array(values: Vec<f32>, shape: Option<Vec<usize>>) -> Self {
        let capacity: usize = values.len();
        let shape: Vec<usize> = shape.unwrap_or_else(|| Vec::from([capacity]));
        Self::_check_shape_capacity_match(&capacity, &shape);
        StaticArray {
            capacity,
            data: values, // Move the input Vec<f32> into data
            shape,
        }
    }

    pub fn reshape(&mut self, new_shape: Vec<usize>) {
        let new_capacity: usize = new_shape.iter().product();
        if new_capacity != self.capacity {
            panic!("New shape must have the same number of elements as the old shape");
        }
        self.shape = new_shape;
    }

    pub fn get_element_at(&self, access_index: &[usize]) -> f32 {
        self._check_index_within_bounds(access_index);
        let flat_index: usize = self._calculate_flat_index(access_index);
        self.data[flat_index]
    }

    pub fn get_elements_slice(
        &self,
        access_index_start: &[usize],
        access_index_end: &[usize],
    ) -> &[f32] {
        let ret_check_indices: bool =
            self._check_end_index_greater_than_start_index(access_index_start, access_index_end);
        if !ret_check_indices {
            panic!("The start index is greater than the end index");
        };
        self._check_index_within_bounds(access_index_start);
        self._check_index_within_bounds(access_index_end);
        let start_index: usize = self._calculate_flat_index(access_index_start);
        let end_index: usize = self._calculate_flat_index(access_index_end);
        &self.data[start_index..end_index]
    }

    pub fn get_subarray(&self, access_index: &[usize]) -> StaticArray {
        match access_index.len().cmp(&self.shape.len()) {
            std::cmp::Ordering::Greater => {
                panic!("The requested subarray is out of the bounds of the array to be sliced");
            }
            std::cmp::Ordering::Equal => {
                panic!(
                    "The requested subarray returns a unique element from the Array. Use 
                    get_element_at() instead."
                );
            }
            std::cmp::Ordering::Less => {
                let ret_shape: &[usize] = &self.shape[access_index.len()..];
                let ret_capacity: usize = ret_shape.iter().product();
                let strides: Vec<usize> = self._calculate_strides();
                let start_index: usize = access_index
                    .iter()
                    .zip(&strides)
                    .map(|(i, s)| i * s)
                    .sum::<usize>();
                let data: Vec<f32> = self.data[start_index..start_index + ret_capacity].to_vec();
                println!("Output size: {:?}", ret_shape);
                StaticArray {
                    capacity: ret_capacity,
                    data,
                    shape: Vec::from(ret_shape),
                }
            }
        }
    }

    pub fn get_view(&self, new_shape: Vec<usize>) -> StaticArray {
        let new_capacity: usize = new_shape.iter().product();
        Self::_check_shape_capacity_match(&self.capacity, &new_shape);
        StaticArray {
            capacity: new_capacity,
            data: self.data.clone(),
            shape: new_shape,
        }
    }
}

impl PartialEq<StaticArray> for StaticArray {
    fn eq(&self, other: &StaticArray) -> bool {
        let mut ret: bool = true;
        ret &= self.data == other.data;
        ret &= self.shape == other.shape;
        ret
    }
}

impl Index<&[usize]> for StaticArray {
    type Output = f32;

    fn index(&self, index: &[usize]) -> &Self::Output {
        self._check_index_within_bounds(index);
        let flat_index: usize = self._calculate_flat_index(index);
        &self.data[flat_index]
    }
}

impl IndexMut<&[usize]> for StaticArray {
    fn index_mut(&mut self, index: &[usize]) -> &mut Self::Output {
        self._check_index_within_bounds(index);
        let flat_index: usize = self._calculate_flat_index(index);
        &mut self.data[flat_index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_static_array_without_constructor() {
        let static_array: StaticArray = StaticArray {
            capacity: 1,
            data: Vec::from([10.]),
            shape: Vec::from([1]),
        };
        let values: Vec<f32> = Vec::from([10.]);
        assert_eq!(static_array.data, values);
    }

    #[test]
    fn test_create_static_array_with_zeros() {
        let ref_array: StaticArray = StaticArray {
            capacity: 2,
            data: Vec::from([0., 0.]),
            shape: Vec::from([1, 2]),
        };
        let array: StaticArray = StaticArray::new_zeros(Vec::from([1, 2]));
        assert_eq!(ref_array.capacity, array.capacity);
        assert_eq!(ref_array.data, array.data);
        assert_eq!(ref_array.shape, array.shape);
    }

    #[test]
    fn test_create_static_array_with_ones() {
        let ref_array: StaticArray = StaticArray {
            capacity: 4,
            data: Vec::from([1., 1., 1., 1.]),
            shape: Vec::from([2, 2]),
        };
        let array: StaticArray = StaticArray::new_ones(Vec::from([2, 2]));
        assert_eq!(ref_array.capacity, array.capacity);
        assert_eq!(ref_array.data, array.data);
        assert_eq!(ref_array.shape, array.shape);
    }

    #[test]
    fn test_create_static_array_with_fill() {
        let ref_array: StaticArray = StaticArray {
            capacity: 2,
            data: Vec::from([3.14, 3.14]),
            shape: Vec::from([2, 1]),
        };
        let array: StaticArray = StaticArray::new_fill(Vec::from([2, 1]), 3.14);
        assert_eq!(ref_array.capacity, array.capacity);
        assert_eq!(ref_array.data, array.data);
        assert_eq!(ref_array.shape, array.shape);
    }

    #[test]
    fn test_create_static_array_from_array() {
        let ref_array: StaticArray = StaticArray {
            capacity: 6,
            data: Vec::from([1., 2., 3., 4., 5., 6.]),
            shape: Vec::from([2, 3]),
        };
        let values: Vec<f32> = Vec::from([1., 2., 3., 4., 5., 6.]);
        let array: StaticArray = StaticArray::new_from_array(values, Some(Vec::from([2, 3])));
        assert_eq!(ref_array.capacity, array.capacity);
        assert_eq!(ref_array.data, array.data);
        assert_eq!(ref_array.shape, array.shape);

        let values: Vec<f32> = Vec::from([1., 2., 3., 4., 5., 6.]);
        let ref_array: StaticArray = StaticArray {
            capacity: 6,
            data: values,
            shape: Vec::from([6]),
        };
        let values: Vec<f32> = Vec::from([1., 2., 3., 4., 5., 6.]);
        let array: StaticArray = StaticArray::new_from_array(values, None);
        assert_eq!(ref_array.capacity, array.capacity);
        assert_eq!(ref_array.data, array.data);
        assert_eq!(ref_array.shape, array.shape);
    }

    #[test]
    #[should_panic]
    fn test_check_shape_capacity_match() {
        let mut capacity: usize = 6;
        let shape: Vec<usize> = Vec::from([2, 3]);
        assert!(StaticArray::_check_shape_capacity_match(&capacity, &shape));
        capacity = 5;
        StaticArray::_check_shape_capacity_match(&capacity, &shape);
    }

    #[test]
    #[should_panic]
    fn test_check_index_within_bounds() {
        let array: StaticArray = StaticArray {
            capacity: 6,
            data: Vec::from([1., 2., 3., 4., 5., 6.]),
            shape: Vec::from([2, 3]),
        };
        let indices: Vec<usize> = Vec::from([2, 1]);
        assert!(array._check_index_within_bounds(&indices));
        let indices: Vec<usize> = Vec::from([1, 3]);
        array._check_index_within_bounds(&indices);
    }

    #[test]
    fn test_calculate_flat_index() {
        let array: StaticArray = StaticArray {
            capacity: 6,
            data: Vec::from([1., 2., 3., 4., 5., 6.]),
            shape: Vec::from([2, 3]),
        };
        assert_eq!(array._calculate_flat_index(&Vec::from([0, 0])), 0);
        assert_eq!(array._calculate_flat_index(&Vec::from([0, 1])), 1);
        assert_eq!(array._calculate_flat_index(&Vec::from([0, 2])), 2);
        assert_eq!(array._calculate_flat_index(&Vec::from([1, 0])), 3);
        assert_eq!(array._calculate_flat_index(&Vec::from([1, 1])), 4);
        assert_eq!(array._calculate_flat_index(&Vec::from([1, 2])), 5);
    }

    #[test]
    fn test_get_element_at_static_array_element() {
        let array: StaticArray = StaticArray {
            capacity: 6,
            data: Vec::from([1., 2., 3., 4., 5., 6.]),
            shape: Vec::from([2, 3]),
        };
        assert_eq!(array.get_element_at(&[0, 0]), 1.);
        assert_eq!(array.get_element_at(&[0, 1]), 2.);
        assert_eq!(array.get_element_at(&[0, 2]), 3.);
        assert_eq!(array.get_element_at(&[1, 0]), 4.);
        assert_eq!(array.get_element_at(&[1, 1]), 5.);
        assert_eq!(array.get_element_at(&[1, 2]), 6.);
    }

    #[test]
    fn test_get_subarray_static_array_2d() {
        let array: StaticArray = StaticArray {
            capacity: 6,
            data: Vec::from([1., 2., 3., 4., 5., 6.]),
            shape: Vec::from([2, 3]),
        };
        let ref_array_0: StaticArray = StaticArray {
            capacity: 3,
            data: Vec::from([1., 2., 3.]),
            shape: Vec::from([3]),
        };
        let ref_array_1: StaticArray = StaticArray {
            capacity: 3,
            data: Vec::from([4., 5., 6.]),
            shape: Vec::from([3]),
        };
        assert_eq!(array.get_subarray(&[0]), ref_array_0);
        assert_eq!(array.get_subarray(&[1]), ref_array_1);
    }

    #[test]
    fn test_get_subarray_static_array_3d() {
        let array: StaticArray = StaticArray {
            capacity: 6,
            data: Vec::from([1., 2., 3., 4., 5., 6., 7., 8.]),
            shape: Vec::from([2, 2, 2]),
        };

        let ref_array_00: StaticArray = StaticArray {
            capacity: 2,
            data: Vec::from([1., 2.]),
            shape: Vec::from([2]),
        };
        assert_eq!(array.get_subarray(&[0, 0]), ref_array_00);

        let ref_array_01: StaticArray = StaticArray {
            capacity: 2,
            data: Vec::from([3., 4.]),
            shape: Vec::from([2]),
        };
        assert_eq!(array.get_subarray(&[0, 1]), ref_array_01);

        let ref_array_10: StaticArray = StaticArray {
            capacity: 2,
            data: Vec::from([5., 6.]),
            shape: Vec::from([2]),
        };
        assert_eq!(array.get_subarray(&[1, 0]), ref_array_10);

        let ref_array_11: StaticArray = StaticArray {
            capacity: 2,
            data: Vec::from([7., 8.]),
            shape: Vec::from([2]),
        };
        assert_eq!(array.get_subarray(&[1, 1]), ref_array_11);

        let ref_array_0: StaticArray = StaticArray {
            capacity: 4,
            data: Vec::from([1., 2., 3., 4.]),
            shape: Vec::from([2, 2]),
        };
        assert_eq!(array.get_subarray(&[0]), ref_array_0);

        let ref_array_1: StaticArray = StaticArray {
            capacity: 4,
            data: Vec::from([5., 6., 7., 8.]),
            shape: Vec::from([2, 2]),
        };
        assert_eq!(array.get_subarray(&[1]), ref_array_1);
    }

    #[test]
    fn test_strides() {
        let array: StaticArray = StaticArray::new_zeros(Vec::from([2]));
        assert_eq!(array._calculate_strides(), Vec::from([1]));
        let array: StaticArray = StaticArray::new_zeros(Vec::from([2, 3]));
        assert_eq!(array._calculate_strides(), Vec::from([3, 1]));
        let array: StaticArray = StaticArray::new_zeros(Vec::from([2, 3, 5]));
        assert_eq!(array._calculate_strides(), Vec::from([15, 5, 1]));
    }

    #[test]
    fn test_get_view() {
        let array: StaticArray = StaticArray::new_zeros(Vec::from([2, 3]));
        let ref_array: StaticArray = StaticArray::new_zeros(Vec::from([3, 2]));
        assert_eq!(array.get_view(Vec::from([3, 2])), ref_array);
    }

    #[test]
    fn test_get_elements_slice() {
        let array: StaticArray = StaticArray {
            capacity: 6,
            data: Vec::from([1., 2., 3., 4., 5., 6.]),
            shape: Vec::from([2, 3]),
        };
        let slice: &[f32] = array.get_elements_slice(&[0, 0], &[0, 2]);
        assert_eq!(slice, &[1., 2.]);

        let slice: &[f32] = array.get_elements_slice(&[0, 1], &[1, 0]);
        assert_eq!(slice, &[2., 3.]);

        let slice: &[f32] = array.get_elements_slice(&[1, 0], &[1, 2]);
        assert_eq!(slice, &[4., 5.]);

        let slice: &[f32] = array.get_elements_slice(&[0, 0], &[1, 1]);
        assert_eq!(slice, &[1., 2., 3., 4.]);
    }

    #[test]
    fn test_index_accessor() {
        let array: StaticArray = StaticArray {
            capacity: 6,
            data: Vec::from([1., 2., 3., 4., 5., 6.]),
            shape: Vec::from([2, 3]),
        };
        assert_eq!(array[&[0, 0]], 1.);
        assert_eq!(array[&[0, 1]], 2.);
        assert_eq!(array[&[0, 2]], 3.);
        assert_eq!(array[&[1, 0]], 4.);
        assert_eq!(array[&[1, 1]], 5.);
        assert_eq!(array[&[1, 2]], 6.);
    }

    #[test]
    fn test_index_mut_accessor() {
        let mut array: StaticArray = StaticArray {
            capacity: 6,
            data: Vec::from([1., 2., 3., 4., 5., 6.]),
            shape: Vec::from([2, 3]),
        };
        array[&[0, 0]] = 10.;
        array[&[0, 1]] = 20.;
        array[&[0, 2]] = 30.;
        array[&[1, 0]] = 40.;
        array[&[1, 1]] = 50.;
        array[&[1, 2]] = 60.;
        assert_eq!(array[&[0, 0]], 10.);
        assert_eq!(array[&[0, 1]], 20.);
        assert_eq!(array[&[0, 2]], 30.);
        assert_eq!(array[&[1, 0]], 40.);
        assert_eq!(array[&[1, 1]], 50.);
        assert_eq!(array[&[1, 2]], 60.);
    }

    #[test]
    fn test_static_array_mutability() {
        let mut array: StaticArray = StaticArray::new_zeros(Vec::from([2, 2]));
        array[&[0, 0]] = 1.0;
        array[&[0, 1]] = 2.0;
        array[&[1, 0]] = 3.0;
        array[&[1, 1]] = 4.0;
        assert_eq!(array[&[0, 0]], 1.0);
        assert_eq!(array[&[0, 1]], 2.0);
        assert_eq!(array[&[1, 0]], 3.0);
        assert_eq!(array[&[1, 1]], 4.0);
    }

    #[test]
    fn test_static_array_capacity_immutability() {
        let array: StaticArray = StaticArray::new_zeros(Vec::from([2, 2]));
        assert_eq!(array.capacity, 4);
        // Attempting to change capacity should not be possible
        // array.capacity = 5; // This line should cause a compile-time error
    }

    #[test]
    fn test_reshape_method() {
        let mut array: StaticArray = StaticArray::new_zeros(Vec::from([2, 2]));
        array.reshape(Vec::from([4]));
        assert_eq!(array.shape, Vec::from([4]));
        assert_eq!(array.capacity, 4);
        assert_eq!(array.data, Vec::from([0.0, 0.0, 0.0, 0.0]));

        let mut array: StaticArray = StaticArray::new_zeros(Vec::from([4]));
        array.reshape(Vec::from([2, 2]));
        assert_eq!(array.shape, Vec::from([2, 2]));
        assert_eq!(array.capacity, 4);
        assert_eq!(array.data, Vec::from([0.0, 0.0, 0.0, 0.0]));
    }
}
