// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    // 这里&表示引用，切片只能分配到堆上，因此要通过一个在栈上的引用来访问堆上数据
    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}
