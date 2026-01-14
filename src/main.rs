use rs_plotting::plotting::*;
fn main() {
    let mut xs = linspace(0., 1., 2);
    let mut ys = transform(&xs, |x| 2. * x);
    println!("{:?}",xs);
    plot(xs, ys);
}
