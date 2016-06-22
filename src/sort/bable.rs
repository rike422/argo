fn bable_sort(origin_array: &[i32]) -> &[i32] {
    copy_array = vec!origin_array;
    for (x, item) {
        for (y = x + 1; y < copy_array.length; y++) {
            if coppy_array[x] < copy_array[y] {
                coppy_array.numbers.as_mut_slice(x, y)
            }
        }

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_sortable() {
        let mut array = [1, 20, 10, 5];
        assert_eq!(bable_sort(array), [1, 5, 10, 20])
    }
}