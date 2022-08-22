use libc::{c_double, c_int};
use rayon::prelude::*;

/*

The C facing function is going to need the following inputs
// that work as inputs
1. c_double to hold the input vector (input)
2. c_int that to hold the output length (length)
3. c_int that will hold the set size (set_size)
// that work as outputs
1. c_int to hold the group (group)
2. c_int to hold the n (n)
3. c_double to hold the mean (mean)
4. c_double to hold the sd (sd)
5. c_double to hold the se (se)
6. c_double to hold the d (d)
7. c_double to hold the t (t)
*/

#[no_mangle]
pub extern "C" fn welfords_wrapper(
    input: *const c_double,
    length: c_int,
    set_size: c_int,
    group: *mut c_int,
    n: *mut c_int,
    mean: *mut c_double,
    sd: *mut c_double,
    se: *mut c_double,
    d: *mut c_double,
    t: *mut c_double,
) {

    let mut input_vec: Vec<f64> = vec![0f64; length as usize];
    // some unsafe rust
    for i in 0..(length as usize) {
        unsafe {
            input_vec[i] = *(input.offset(i as isize)) as f64;
        }
    }

    // now some safe rust
    let results_vec: Vec<Welfords> = run_welfords(input_vec, set_size as usize);
    // now some more unsafe rust
    for (i, el) in results_vec.iter().enumerate() {
        unsafe {
            *group.add(i) = el.group as i32;
            *n.add(i) = el.count as i32;
            *mean.add(i) = el.mean;
            *sd.add(i) = el.sd;
            *se.add(i) = el.se;
            *d.add(i) = el.d;
            *t.add(i) = el.t;
        }
    }
}

fn run_welfords(input_vec: Vec<f64>, set_size: usize) -> Vec<Welfords> {

    input_vec
        .into_par_iter()
        .chunks(set_size as usize)
        .enumerate()
        .map(|(i, group)| {
            group
                .iter()
                .scan((i + 1, 0., 0., 0.), welfords)
                .collect::<Vec<Welfords>>()
        })
        .flatten()
        .collect()
}


struct Welfords {
    group: usize,
    count: f64,
    mean: f64,
    sd: f64,
    se: f64,
    d: f64,
    t: f64,
}

impl Welfords {
    fn new(
        group: usize,
        count: f64,
        mean: f64,
        sd: f64,
        se: f64,
        d: f64,
        t: f64,
    ) -> Welfords {
        Welfords {
            group,
            count,
            mean,
            sd,
            se,
            d,
            t,
        }
    }
}

fn welfords(
    (group, count, mean, squared_distances): &mut (
        usize,
        f64,
        f64,
        f64,
    ),
    new_value: &f64,
) -> Option<Welfords> {
    *count += 1.;
    let delta = new_value - *mean;
    *mean += delta / *count;
    let delta2 = new_value - *mean;
    *squared_distances += delta * delta2;
    let sd = (*squared_distances / (*count - 1.)).sqrt();
    let se = sd / count.sqrt();
    let t = *mean / se;
    let d = *mean / sd;
    Some(Welfords::new(
        *group, *count, *mean,
        sd, se, d, t,
    ))
}

#[cfg(test)]
mod tests {

    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn test_welfords() {
        let result = vec![1.2, 2.3, 3.4];
        let result = result
            .iter()
            .scan((1, 0., 0., 0., 0., 0., 0., 0.), welfords)
            .collect::<Vec<_>>();
        let f = result.iter().last().unwrap();
        assert_eq!(f.sd, 1.1);
        assert_eq!(f.mean, 2.3);
        assert_eq!(f.count, 3f64);
        assert_eq!(f.group, 1);
        // assert_eq!(f.dist, 2.42);

        assert!(approx_eq!(f64, f.t, 3.6215607794621976));
        assert!(approx_eq!(f64, f.d, 2.0909090909090904));
        assert!(approx_eq!(f64, f.se, 0.6350852961085884));
    }
}
