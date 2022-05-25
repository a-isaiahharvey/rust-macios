use crate::{
    foundation::{traits::t_NSArray, MutableArray, NSNumber, String, UInt},
    objective_c_runtime::traits::t_NSObject,
};

use super::Array;

/// An iterator for `Array`
#[derive(Debug)]
pub struct Iter<'a, T>
where
    T: t_NSObject,
{
    pub(super) array: &'a Array<T>,
    pub(super) index: UInt,
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: t_NSObject,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.array.count() {
            None
        } else {
            let item = self.array.objectAt(self.index);
            self.index += 1;
            Some(item)
        }
    }
}

impl<'a, T> FromIterator<&'a T> for Array<T>
where
    T: t_NSObject + 'a,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a T>,
    {
        let mut mut_arr = MutableArray::new();
        for item in iter {
            mut_arr.add(item);
        }

        Array::from(mut_arr)
    }
}

impl FromIterator<String> for Array<String> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = String>,
    {
        let mut mut_arr = MutableArray::new();
        for item in iter {
            mut_arr.add(&item);
        }

        Array::from(mut_arr)
    }
}

impl FromIterator<NSNumber> for Array<NSNumber> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = NSNumber>,
    {
        let mut mut_arr = MutableArray::new();
        for item in iter {
            mut_arr.add(&item);
        }

        Array::from(mut_arr)
    }
}