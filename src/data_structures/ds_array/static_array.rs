/// We define an static array as a data structure consisting of a collection of elements (values or
/// variables), of same memory size, each identified by at least one array index or key. The key
/// property that defines the array as static is the fact of having a fixed length (or size) defined
/// when is created, whether or not the elements inside are immutable.
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

    fn _check_shape_capacity_match(_capacity: &usize, _shape: &[usize]) -> bool{
        let shape_product: usize = _shape.iter().product();
        if shape_product != *_capacity {
            panic!(
                "The given input array and shape does not match in capacity: {} != {}",
                shape_product, *_capacity
            );
        }
        true
    }

    fn _check_index_within_bounds(&self, indices: &[usize]) -> bool {
        if indices.len() != self.shape.len() {
            panic!("Number of indices does not match the shape dimensions");
        }
        for (i, &index) in indices.iter().enumerate() {
            if index >= self.shape[i] {
                panic!("Index out of bounds for dimension {}: {} >= {}", i, index, self.shape[i]);
            }
        }
        true
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

    pub fn get_at(&self, indices: &[usize]) -> f32 {
        self._check_index_within_bounds(indices);
        let flat_index: usize = self._calculate_flat_index(indices);
        self.data[flat_index]
    }

    // pub fn get_slice() -> StaticArray {

    // }

    // pub fn get_from_to() -> StaticArray {

    // }

    // pub fn get_view() -> StaticArray {

    // }
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
    fn test_get_static_array_element() {
        let array: StaticArray = StaticArray {
            capacity: 6,
            data: Vec::from([1., 2., 3., 4., 5., 6.]),
            shape: Vec::from([2, 3]),
        };
        assert_eq!(array.get_at(&[0, 0]), 1.);
        assert_eq!(array.get_at(&[0, 1]), 2.);
        assert_eq!(array.get_at(&[0, 2]), 3.);
        assert_eq!(array.get_at(&[1, 0]), 4.);
        assert_eq!(array.get_at(&[1, 1]), 5.);
        assert_eq!(array.get_at(&[1, 2]), 6.);
    }
}
