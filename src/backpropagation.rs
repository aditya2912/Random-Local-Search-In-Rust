fn backpropagation() {
fn forward_multiply_gate(x: f64, y: f64) -> f64 {
    x * y
}

fn forward_add_gate(a: f64, b: f64) -> f64 {
    a + b
}
fn forward_circuit(x: f64, y: f64, z: f64) -> f64 {
    let mut q: f64 = forward_add_gate(x, y);
    let mut f: f64 = forward_multiply_gate(q, z);
    return f;
}
let mut x: f64 = -2.0;
let mut y: f64 = 5.0;
let mut z: f64 = -4.0;
let h: f64 = 0.0001;

let mut x_derivative: f64 = (forward_circuit((x + h), y, z) - forward_circuit(x,y,z)) / h;
let mut y_derivative: f64 = (forward_circuit(x, (y +  h), z) - forward_circuit(x,y,z)) / h;
let mut z_derivative: f64 = (forward_circuit(x,y, (z + h))  - forward_circuit(x,y,z)) / h;

println!(" {:?}  {:?} {:?}", x_derivative, y_derivative, z_derivative);
}
