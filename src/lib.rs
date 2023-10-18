use pyo3::prelude::*;
use std::f64::consts::PI;

// * Helper functions

// Pendulum
fn a_pendulum(x: f64, w: f64) -> f64 {
    return -w.powf(2.0) * x.sin();
}

fn yoshida_n4_sub_step(term: f64, cd: f64, ap: f64, dt: f64) -> f64 {
    return term + cd * ap * dt;
}

fn yoshida_n4_step_pendulum(x: f64, p: f64, c: f64, d: f64, dt: f64, w: f64) -> (f64, f64) {
    let nx: f64;
    let np: f64;
    let anx: f64;

    nx = yoshida_n4_sub_step(x, c, p, dt);
    anx = a_pendulum(nx, w);
    np = yoshida_n4_sub_step(p, d, anx, dt);

    return (nx, np);
}

fn yoshida_n4_pendulum(x: f64, p: f64, dt: f64, w: f64) -> (f64, f64) {
    let c1: f64 = 1_f64 / 2_f64 / (2_f64 - 2_f64.powf(1_f64 / 3_f64));
    let c4: f64 = c1;
    let c2: f64 = (1_f64 - 2_f64.powf(1_f64 / 3_f64)) * c1;
    let c3: f64 = c2;
    let d1: f64 = 1_f64 / (2_f64 - 2_f64.powf(1_f64 / 3_f64));
    let d3: f64 = d1;
    let d2: f64 = -(2_f64.powf(1_f64 / 3_f64)) / (2_f64 - 2_f64.powf(1_f64 / 3_f64));
    let d4: f64 = 0_f64;
    let carr: [f64; 4] = [c1, c2, c3, c4];
    let darr: [f64; 4] = [d1, d2, d3, d4];
    let n: i32 = 4;
    let mut xn: f64 = x;
    let mut pn: f64 = p;

    for i in 0..n {
        (xn, pn) = yoshida_n4_step_pendulum(xn, pn, carr[i as usize], darr[i as usize], dt, w);
    }

    return (xn, pn);
}

// Standard map
fn standard_map(theta: f64, p: f64, k: f64) -> (f64, f64) {
    let np: f64 = (p + k * theta.sin()) % (2_f64 * PI);
    let ntheta: f64 = (theta + np) % (2_f64 * PI);

    return (ntheta, np);
}

// * Python functions

/// Calculates the position and velocity of a pendulum over time using the given parameters.
///
/// # Arguments
///
/// * `theta` - The initial angle of the pendulum in radians.
/// * `theta_dot` - The initial angular velocity of the pendulum in radians per second.
/// * `w` - The frequency of the pendulum.
/// * `dt` - The time step between each iteration.
/// * `n` - The number of iterations to perform.
///
/// # Returns
///
/// A tuple containing the position and velocity of the pendulum over time.
///
/// # Example
///
/// ```
/// let (position, velocity) = pendulum_tracking(0.1, 0.0, 2*pi*0.5, 1e-2, 10);
/// ```
#[pyfunction]
fn pendulum_tracking(theta: f64, theat_dot: f64, w: f64, dt: f64, n: i32) -> (Vec<f64>, Vec<f64>) {
    let mut theta_out: Vec<f64> = vec![0.0; n as usize];
    let mut theta_dot_out: Vec<f64> = vec![0.0; n as usize];
    let mut x: f64 = theta;
    let mut p: f64 = theat_dot;

    for i in 0..n {
        theta_out[i as usize] = x;
        theta_dot_out[i as usize] = p;
        (x, p) = yoshida_n4_pendulum(x, p, dt, w);
    }

    return (theta_out, theta_dot_out);
}

/// Calculates the position and velocity of a particle in the standard map 
/// using the given parameters.
///
/// # Arguments
///
/// * `theta` - The initial angle of the particle in radians.
/// * `p` - The initial angular velocity of the particle in radians.
/// * `k` - The strength of the perturbation in the standard map.
/// * `n` - The number of iterations to perform.
///
/// # Returns
///
/// A tuple containing the position and velocity of the particle over time.
///
/// # Example
///
/// ```
/// let (position, velocity) = standard_map_tracking(0.1, 0.2, -0.5, 100);
/// ```
#[pyfunction]
fn standard_map_tracking(theta: f64, p: f64, k: f64, n: i32) -> (Vec<f64>, Vec<f64>) {
    let mut theta_out: Vec<f64> = vec![0.0; n as usize];
    let mut p_out: Vec<f64> = vec![0.0; n as usize];
    let mut x: f64 = theta;
    let mut y: f64 = p;

    for i in 0..n {
        theta_out[i as usize] = x;
        p_out[i as usize] = y;
        (x, y) = standard_map(x, y, k);
    }

    return (theta_out, p_out);
}

/// * A Python module implemented in Rust.
#[pymodule]
fn py_rust_maps(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pendulum_tracking, m)?)?;
    m.add_function(wrap_pyfunction!(standard_map_tracking, m)?)?;
    Ok(())
}
