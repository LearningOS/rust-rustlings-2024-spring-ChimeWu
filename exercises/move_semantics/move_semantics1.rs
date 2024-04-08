// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);
    let mut length = vec1.len();

    println!("{} has length {} content `{:?}`", "vec1", length, vec1);

    vec1.push(88);
    length = vec1.len();
    println!("{} has length {} content `{:?}`", "vec1", length, vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
