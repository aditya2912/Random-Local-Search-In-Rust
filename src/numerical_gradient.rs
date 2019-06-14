use rand::Rng;

fn numerical_gradient() {
fn forward_multiply_gate(x: f64, y: f64) -> f64 {
    x * y
};

let gradient: f64 = 0.01;
let initial_x: f64 = -2.0;
let initial_y: f64 = 3.0;

let mut output: f64 = forward_multiply_gate(initial_x, initial_y);

let mut derivative_wrtx: f64 = initial_x + gradient;
let mut updated_output: f64 = forward_multiply_gate(derivative_wrtx, initial_y);
let mut x_derivative: f64 = (updated_output - output) / gradient;


let mut derivative_wrty: f64 = initial_y + gradient;
let mut updated_output2: f64 = forward_multiply_gate(initial_x, derivative_wrty);
let mut y_derivative: f64 = (updated_output2 - output) / gradient;


let step_size: f64 = 0.01;
let mut x_try: f64 = initial_x + step_size * x_derivative;
let mut y_try: f64 = initial_y + step_size * y_derivative;
let mut new_output: f64 = forward_multiply_gate(x_try, y_try);

println!("{:?} {:?} {:?}", new_output, updated_output2, updated_output);
}





























// numerical gradient
//
// fn forward_multiply_gate(x: f64, y: f64) -> f64 {
//     x * y
// };
//
// let gradient: f64 = 0.01;
// let initial_x: f64 = -2.0;
// let initial_y: f64 = 3.0;
//
// let mut output: f64 = forward_multiply_gate(initial_x, initial_y);
//
// let mut derivative_wrtx: f64 = initial_x + gradient;
// let mut updated_output: f64 = forward_multiply_gate(derivative_wrtx, initial_y);
// let mut x_derivative: f64 = (updated_output - output) / gradient;
//
//
// let mut derivative_wrty: f64 = initial_y + gradient;
// let mut updated_output2: f64 = forward_multiply_gate(initial_x, derivative_wrty);
// let mut y_derivative: f64 = (updated_output2 - output) / gradient;
//
//
// let step_size: f64 = 0.01;
// let mut x_try: f64 = initial_x + step_size * x_derivative;
// let mut y_try: f64 = initial_y + step_size * y_derivative;
// let mut new_output: f64 = forward_multiply_gate(x_try, y_try);
//
// println!("{:?} {:?} {:?}", new_output, updated_output2, updated_output);
