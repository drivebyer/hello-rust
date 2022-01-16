fn main() {
    let number_arr = [1, 2, 3];
    let number_vec = vec![1, 2, 3];

    // TODO: they variable slice_to_number_arr already fixed size, why compile error?
    let slice_to_number_arr = number_vec[0..1];

    let ref_to_number_arr = &number_arr;
    let ref_to_number_vec = &number_vec;

    // force convert to slice reference
    let slice_ref_to_number_arr: &[i32] = &number_arr;
    let slice_ref_to_number_vec: &[i32] = &number_vec;
}
