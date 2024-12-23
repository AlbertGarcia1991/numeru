/// We define an static array as a data structure consisting of a collection of elements (values or
/// variables), of same memory size, each identified by at least one array index or key. The key
/// property that defines the array as static is the fact of having a fixed length (or size) defined
/// when is created, whether or not the elements inside are immutable.
use std::alloc::{alloc, Layout};

struct StaticArray {
    capacity: usize,
    data: Box<[f32]>,
    shape: Box<[usize]>,
}

impl StaticArray {
    fn _new_array(value: f32, shape: Box<[usize]>) -> Self {
        let capacity: usize = shape.iter().product();

        // Allocate memory for the array
        let layout: Layout = Layout::array::<f32>(capacity).unwrap();
        let ptr: *mut f32 = unsafe { alloc(layout) as *mut f32 };

        if ptr.is_null() {
            panic!("Memory allocation failed");
        }

        // Initialize the array with the given value
        for i in 0..capacity {
            unsafe {
                *ptr.add(i) = value;
            }
        }

        // Convert the raw pointer to a Box<[f32]> to ensure it is properly deallocated
        unsafe {
            StaticArray {
                capacity,
                data: Box::from_raw(std::slice::from_raw_parts_mut(ptr, capacity)),
                shape,
            }
        }
    }

    fn _check_shape_capacity_match(_capacity: &usize, _shape: &[usize]) {
        let shape_product: usize = _shape.iter().product();
        if shape_product != *_capacity {
            panic!(
                "The given input array and shape does not match in capacity: {} != {}",
                shape_product, *_capacity
            );
        }
    }

    pub fn new_zeros(shape: Box<[usize]>) -> Self {
        Self::_new_array(0., shape)
    }

    pub fn new_ones(shape: Box<[usize]>) -> Self {
        Self::_new_array(1., shape)
    }

    pub fn new_fill(shape: Box<[usize]>, fill_value: f32) -> Self {
        Self::_new_array(fill_value, shape)
    }

    pub fn new_from_array(values: Box<[f32]>, shape: Option<Box<[usize]>>) -> Self {
        let capacity: usize = values.len();

        let shape: Box<[usize]> = shape.unwrap_or_else(|| Box::new([capacity]));

        StaticArray {
            capacity,
            data: values, // Move the input Box<[f32]> into data
            shape,
        }
    }

    pub fn get_at() -> f32 {
        3.14
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
    fn create_static_array_without_constructor() {
        let static_array: StaticArray = StaticArray {
            capacity: 1,
            data: Box::new([10.]),
            shape: Box::new([1]),
        };
        let values: Box<[f32]> = Box::new([10.]);
        assert_eq!(static_array.data, values);
    }

    #[test]
    fn create_static_array_with_zeros() {
        let ref_array: StaticArray = StaticArray {
            capacity: 2,
            data: Box::new([0., 0.]),
            shape: Box::new([1, 2]),
        };
        let array: StaticArray = StaticArray::new_zeros(Box::new([1, 2]));
        assert_eq!(ref_array.capacity, array.capacity);
        assert_eq!(ref_array.data, array.data);
        assert_eq!(ref_array.shape, array.shape);
    }

    #[test]
    fn create_static_array_with_ones() {
        let ref_array: StaticArray = StaticArray {
            capacity: 4,
            data: Box::new([1., 1., 1., 1.]),
            shape: Box::new([2, 2]),
        };
        let array: StaticArray = StaticArray::new_ones(Box::new([2, 2]));
        assert_eq!(ref_array.capacity, array.capacity);
        assert_eq!(ref_array.data, array.data);
        assert_eq!(ref_array.shape, array.shape);
    }

    #[test]
    fn create_static_array_with_fill() {
        let ref_array: StaticArray = StaticArray {
            capacity: 2,
            data: Box::new([3.14, 3.14]),
            shape: Box::new([2, 1]),
        };
        let array: StaticArray = StaticArray::new_fill(Box::new([2, 1]), 3.14);
        assert_eq!(ref_array.capacity, array.capacity);
        assert_eq!(ref_array.data, array.data);
        assert_eq!(ref_array.shape, array.shape);
    }

    #[test]
    fn create_static_array_from_array() {
        let ref_array: StaticArray = StaticArray {
            capacity: 6,
            data: Box::new([1., 2., 3., 4., 5., 6.]),
            shape: Box::new([2, 3]),
        };
        let values: Box<[f32]> = Box::new([1., 2., 3., 4., 5., 6.]);
        let array: StaticArray = StaticArray::new_from_array(values, Some(Box::new([2, 3])));
        assert_eq!(ref_array.capacity, array.capacity);
        assert_eq!(ref_array.data, array.data);
        assert_eq!(ref_array.shape, array.shape);

        let values: Box<[f32]> = Box::new([1., 2., 3., 4., 5., 6.]);
        let ref_array: StaticArray = StaticArray {
            capacity: 6,
            data: values,
            shape: Box::new([6]),
        };
        let values: Box<[f32]> = Box::new([1., 2., 3., 4., 5., 6.]);
        let array: StaticArray = StaticArray::new_from_array(values, None);
        assert_eq!(ref_array.capacity, array.capacity);
        assert_eq!(ref_array.data, array.data);
        assert_eq!(ref_array.shape, array.shape);
    }

    #[test]
    fn get_static_array_element() {
        let array: StaticArray = StaticArray {
            capacity: 6,
            data: Box::new([1., 2., 3., 4., 5., 6.]),
            shape: Box::new([2, 3]),
        };
        assert_eq!(array.get_at([0, 0]), 1.);
        assert_eq!(array.get_at([0, 1]), 2.);
        assert_eq!(array.get_at([0, 2]), 3.);
        assert_eq!(array.get_at([1, 0]), 4.);
        assert_eq!(array.get_at([1, 1]), 5.);
        assert_eq!(array.get_at([1, 2]), 6.);
    }
}
