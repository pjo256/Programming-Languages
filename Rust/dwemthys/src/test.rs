
fn main() {
	let mut x = 5;
{
    let y = &mut x;
    *y += 1;
}
{
	let z = & x;
	println!("NOT MUT {}", z);
}
println!("{}", x);
}