// driver2.rs
// :PROPERTIES:
// :header-args: :tangle examples/driver2.rs
// :END:

// [[file:~/Workspace/Programming/rust-libs/cg_descent.rs/cg_descent.note::*driver2.rs][driver2.rs:1]]
use cg_descent::*;

fn main() {
    // construct CGDescent object
    let mut cgd = CGDescent::default();

    // Change print level
    cgd.param.PrintLevel = 2;

    // Set callback function to evaluate value
    cgd.set_val_fn(|x: &[f64]| {
        let mut t = 0.0;
        let mut f = 0.0;
        for i in 0..x.len() {
            t = i as f64 + 1.;
            t = t.sqrt();
            let ex = x[i].exp();
            f += ex - t*x[i];
        }

        Ok(f)
    });

    // Set callback function to evaluate value and gradient
    cgd.set_grd_fn(|x: &[f64], g: &mut [f64]| {
        let mut ex = 0.0;
        let mut t = 0.0;
        let mut f = 0.0;
        for i in 0..x.len() {
            t = i as f64 + 1.;
            t = t.sqrt();
            ex = x[i].exp();
            f += ex - t*x[i];
            g[i] = ex - t;
        }

        Ok(f)

    });

    // set starting guess
    let mut x = [1.0; 100];
    cgd.run(&mut x).expect("cgd");
}
// driver2.rs:1 ends here
