fn forward_multiply_gate(x: f64, y: f64) -> f64  {
    return x * y
}
pub fn main() {
    let mut x: f64 = -2.0;
    let mut y: f64 = 3.0;

    let mut output = forward_multiply_gate(x, y);

    let mut x_gradient: f64 = y;
    let mut y_gradient: f64 = x;


    let step_size: f64 = 0.01;

    x += step_size * x_gradient;
    y += step_size * y_gradient;

    let mut new_output = forward_multiply_gate(x, y);

    println!(" New Output {:?}", new_output);
}


random local search
use rand::Rng;
fn forward_multiply_gate(x: f64, y: f64) -> f64  {
    return x * y
}

fn main() {
    let tweak_amount: f64 = 0.01;
    let mut best_out: f64 = std::i32::MIN as f64; //-0f64;
    let initial_x: f64 = -2.0;
    let initial_y: f64 = 3.0;
    let mut best_x: f64 = initial_x;
    let mut best_y: f64 = initial_y;

    let mut rng = rand::thread_rng();
    for iter in 0..10 {
        let random_number1: f64 = rng.gen_range(0f64, 1f64);
        let random_number2: f64 = rng.gen_range(0f64, 1f64);
        let mut x_try: f64 = initial_x + tweak_amount *((random_number1 * 2.0) - 1.0);
        let mut y_try: f64 = initial_y + tweak_amount *((random_number2 * 2.0) - 1.0);
        let mut output: f64 = forward_multiply_gate(x_try, y_try);
        if (output > best_out) {
            best_out = output;
            best_x = x_try;
            best_y = y_try;
            // let updated_output: f64 = forward_multiply_gate(best_x, best_y);
            // println!("UPDATED OUTPUT {:?}", updated_output);
        }

        println!("{:?} {:?} {:?}", best_out, best_x, best_y);
        // println!(" OUTPUT {:?}", output);
        // println!(" BEST_X {:?}", best_x);
        // println!(" BEST_Y {:?}", best_y);
        // println!(" BEST OUTPUT {:?}", best_out);
    }

}
