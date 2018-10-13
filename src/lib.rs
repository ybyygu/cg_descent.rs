// base

// [[file:~/Workspace/Programming/rust-libs/cg_descent.rs/cg_descent.note::*base][base:1]]
use std::ptr::null_mut;
use std::os::raw::{c_int, c_void, c_long};
use quicli::prelude::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub type CGParameter = cg_parameter;

impl Default for CGParameter {
    /// default LBFGS parameter
    fn default() -> Self {
        let mut param: cg_parameter;
        unsafe {
            param = ::std::mem::uninitialized();
            cg_default(&mut param);
        }

        param
    }
}
// base:1 ends here

// cg

// [[file:~/Workspace/Programming/rust-libs/cg_descent.rs/cg_descent.note::*cg][cg:1]]
#[repr(C)]
pub struct CGDescent {
    /// CG parameters
    pub param: CGParameter,
    /// A closure to evaluate the objective function
    val_fn: Box<FnMut(&[f64]) -> Result<f64>>,
    /// A closure to evaluate gradient
    grd_fn: Box<FnMut(&[f64], &mut [f64]) -> Result<f64>>,
}

impl Default for CGDescent {
    fn default() -> Self {
        let mut param = CGParameter::default();
        param.QuadStep = 0;
        param.PrintLevel = 1;

        CGDescent {
            param,
            val_fn: Box::new(value_default),
            grd_fn: Box::new(valgrad_default),
        }
    }
}

impl CGDescent {
    pub fn set_val_fn<CB: 'static + FnMut(&[f64]) -> Result<f64>>(&mut self, c: CB) {
        self.val_fn = Box::new(c);
    }

    pub fn set_grd_fn<CB: 'static + FnMut(&[f64], &mut [f64]) -> Result<f64>>(&mut self, c: CB) {
        self.grd_fn = Box::new(c);
    }

    pub fn run(&mut self, arr_x: &mut [f64]) -> Result<()> {
        // Cast CGDescent as a void pointer for passing it to cg_descent
        // function as the instance parameter let instance = &self.to_ptr();
        let instance = self as *const _ as *mut c_void;

        // run ffi code
        let n = arr_x.len();
        let rt = unsafe {
            cg_descent(
                arr_x.as_mut_ptr(),
                n as c_long,
                null_mut(),
                &mut self.param,
                1E-8,
                Some(value_wrapper),
                Some(gradient_wrapper),
                None,
                null_mut(),
                instance,
            )
        };
        println!("status code = {:}", rt);

        Ok(())
    }
}

#[test]
fn test_cgdescent() {
    let mut param = CGParameter::default();
    param.QuadStep = 0;
    param.PrintLevel = 1;

    let mut cgd = CGDescent::default();

    // set starting guess
    let mut x = [1.0; 100];

    let rt = cgd.run(&mut x).expect("cgd");
}
// cg:1 ends here

// value callback

// [[file:~/Workspace/Programming/rust-libs/cg_descent.rs/cg_descent.note::*value%20callback][value callback:1]]
extern fn value_wrapper(x: *mut f64, n: c_long, instance: *mut c_void) -> f64 {
    let n = n as usize;
    let x = unsafe {
        ::std::slice::from_raw_parts(x, n)
    };

    // cast as Rust instance
    let f = unsafe {
        let ptr_cg = instance as *mut CGDescent;
        ((*ptr_cg).val_fn)(&x).expect("valgrad callback")
    };

    f
}

/// default value evaluator, adopted from driver2.c
pub fn value_default(x: &[f64]) -> Result<f64> {
    let mut t = 0.0;
    let mut f = 0.0;
    for i in 0..x.len() {
        t = i as f64 + 1.;
        t = t.sqrt();
        let ex = x[i].exp();
        f += ex - t*x[i];
    }

    Ok(f)
}
// value callback:1 ends here

// gradient callback

// [[file:~/Workspace/Programming/rust-libs/cg_descent.rs/cg_descent.note::*gradient%20callback][gradient callback:1]]
extern fn gradient_wrapper(g: *mut f64, x: *mut f64, n: c_long, instance: *mut c_void) {
    let n = n as usize;
    let x = unsafe {
        ::std::slice::from_raw_parts(x, n)
    };

    let mut g = unsafe {
        ::std::slice::from_raw_parts_mut(g, n)
    };

    // cast as Rust instance
    unsafe {
        let ptr_cg = instance as *mut CGDescent;
        ((*ptr_cg).grd_fn)(&x, &mut g).expect("valgrad callback");
    }
}

/// default gradient evaluator, adopted from driver2.c
pub fn gradient_default(x: &[f64], g: &mut [f64]) {
    let mut ex = 0.0;
    let mut t = 0.0;
    for i in 0..x.len() {
        t = i as f64 + 1.;
        t = t.sqrt();
        ex = x[i].exp();
        g[i] = ex - t;
    }
}
// gradient callback:1 ends here

// valgrad callback

// [[file:~/Workspace/Programming/rust-libs/cg_descent.rs/cg_descent.note::*valgrad%20callback][valgrad callback:1]]
// extern fn valgrad_wrapper(g: *mut f64, x: *mut f64, n: c_long, instance: *mut c_void) -> f64 {
//     let n = n as usize;
//     let x = unsafe {
//         ::std::slice::from_raw_parts(x, n)
//     };

//     let mut g = unsafe {
//         ::std::slice::from_raw_parts_mut(g, n)
//     };

//     // cast as Rust instance
//     let f = unsafe {
//         let ptr_cg = instance as *mut CGDescent;
//         (ptr_cg.grd_fn)(&x, &mut g).expect("valgrad callback")
//     };

//     f
// }

/// default value and gradient evaluator, adopted from driver2.c
pub fn valgrad_default(x: &[f64], g: &mut [f64]) -> Result<f64>{
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
}
// valgrad callback:1 ends here
