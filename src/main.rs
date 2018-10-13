// main.rs
// :PROPERTIES:
// :header-args: :tangle src/main.rs
// :END:

// [[file:~/Workspace/Programming/rust-libs/cg_descent.rs/cg_descent.note::*main.rs][main.rs:1]]
use std::ptr::null_mut;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    const n: usize = 100;
    // set starting guess
    let mut x = [1.0; n];

    // set default parameter values
    let mut param: cg_parameter;
    unsafe {
        param = ::std::mem::uninitialized();
        cg_default(&mut param);
    }

    param.QuadStep = 0;
    param.PrintLevel = 1;

    // run the code
    let rt = unsafe {
        cg_descent(x.as_mut_ptr(),
                   n as ::std::os::raw::c_long,
                   null_mut(),
                   &mut param,
                   1E-8,
                   Some(myvalue),
                   Some(mygrad),
                   Some(myvalgrad),
                   null_mut())
    };
    println!("status code = {:}", rt);
}

extern fn myvalgrad(g: *mut f64, x: *mut f64, n: ::std::os::raw::c_long) -> f64 {
    let n = n as usize;
    let x = unsafe {
        ::std::slice::from_raw_parts(x, n)
    };

    let mut g = unsafe {
        ::std::slice::from_raw_parts_mut(g, n)
    };

    let mut ex = 0.0;
    let mut t = 0.0;
    let mut f = 0.0;
    for i in 0..n {
        t = i as f64 + 1.;
        t = t.sqrt();
        ex = x[i].exp();
        f += ex - t*x[i];
        g[i] = ex - t;
    }

    f
}

extern fn myvalue(x: *mut f64, n: ::std::os::raw::c_long) -> f64 {
    let n = n as usize;
    let x = unsafe {
        ::std::slice::from_raw_parts(x, n)
    };

    let mut t = 0.0;
    let mut f = 0.0;
    for i in 0..n {
        t = i as f64 + 1.;
        t = t.sqrt();
        let ex = x[i].exp();
        f += ex - t*x[i];
    }

    f
}

extern fn mygrad(g: *mut f64, x: *mut f64, n: ::std::os::raw::c_long) {
    let n = n as usize;
    let x = unsafe {
        ::std::slice::from_raw_parts(x, n)
    };

    let mut g = unsafe {
        ::std::slice::from_raw_parts_mut(g, n)
    };

    let mut ex = 0.0;
    let mut t = 0.0;
    let mut f = 0.0;
    for i in 0..n {
        t = i as f64 + 1.;
        t = t.sqrt();
        ex = x[i].exp();
        g[i] = ex - t;
    }
}
// main.rs:1 ends here
