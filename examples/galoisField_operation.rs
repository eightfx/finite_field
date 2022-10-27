use galois_field::*;

fn main(){
	// consider GF(2^4)
	let char: u32 = 2;
	let n = 4;
	let primitive_polynomial = Polynomial::get_primitive_polynomial(char, n);
	let x:FiniteField = FiniteField{
 		char: char,
 		element:Element::GaloisField{element:vec![0,1],primitive_polynomial:primitive_polynomial.clone()} // i.e. [0,1] = x -> 2 over GF(2^4)
	};
	let y:FiniteField = FiniteField{
 		char: char,
 		element:Element::GaloisField{element:vec![0,0,1,1],primitive_polynomial:primitive_polynomial.clone()} // i.e. [0,0,1,1] = x^3 + x^2 -> 12 over GF(2^4)
	};
	println!("x + y = {:?}", (x.clone() + y.clone()).element);
	println!("x - y = {:?}", (x.clone() - y.clone()).element);
	println!("x * y = {:?}", (x.clone() * y.clone()).element);
	println!("x / y = {:?}", (x.clone() / y.clone()).element);

}
