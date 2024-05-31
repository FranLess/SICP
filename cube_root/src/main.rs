use std::f64;

fn main() {
    print!("{}", cube_root(27f64));
}

fn cube_root(x: f64) -> f64 {
    cube_root_iter(1f64, x)
}

fn cube_root_iter(guess: f64, x: f64) -> f64 {
    if good_enough(guess, x) {
        guess
    } else {
        cube_root_iter(improve(guess, x), x)
    }
}

fn good_enough(guess: f64, x: f64) -> bool {
    let dif = abs(cube(guess) - x);
    dif < 0.001
}

fn improve(guess: f64, x: f64) -> f64 {
    (x / (square(guess)) + 2f64 * guess) / 3f64
}

fn square(x: f64) -> f64 {
    x * x
}

fn cube(x: f64) -> f64 {
    x * x * x
}

fn abs(x: f64) -> f64 {
    if x < 0f64 {
        -x
    } else {
        x
    }
}
