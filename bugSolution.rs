fn main() {
    let mut v = vec![1, 2, 3];
    let value = v[0]; // Access elements safely without raw pointers
    let new_vec = vec![10,value,3];
    println!("First element: {}", new_vec[0]);
    println!("Second element: {}", new_vec[1]);
    println!("Third element: {}", new_vec[2]);
} 