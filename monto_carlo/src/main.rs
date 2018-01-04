extern crate rand;
use std::io;
use rand::distributions::{IndependentSample, Range};
use std::cmp::Ordering;



// I have used rand documentation for monte carlo from rand crate docs

fn main() {

    // let the square be of 2 unit length
    // let the one unit length circle
    // circles center is 0,0 and this is also the center of square

    let mut total_limit = String::new();

    io::stdin().read_line(&mut total_limit)
        .expect("error reading");

    println!("{}", total_limit);

    let total_limit: u64 = total_limit.trim().parse()
        .expect("Please enter a number");

    let mut counter = 1; // number of iterations
    let mut in_circle = 0; // number of points inside circle

    let between = Range::new(-1f64, 1.0); // generate a range to be used.
    let mut rng = rand::thread_rng();

    loop {

        let x = between.ind_sample(&mut rng);
        let y = between.ind_sample(&mut rng);
        
        let length = x*x + y*y;


        if length <= 1.0 {
            in_circle += 1;
        }


        match total_limit.cmp(&counter) {
            // checks for total_limit compared to counter

            Ordering::Greater => {
                counter = counter + 1;                
            }

            Ordering::Less => {
                break;
                
            }

            Ordering::Equal => {
                break;
            }
        }

    }




    println!("total: {}, incircle: {}, pi approx: {}", total_limit, in_circle, 4. * (in_circle as f64) / (total_limit as f64));

}
