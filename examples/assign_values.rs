is_field::*;


// # What is this?
// This is a library for working with finite fields. It is a work in progress.
// The following sequence tests the assignment to a polynomial.
// 1. get primitive polynomial of degree 3 over GF(2)
// 2. get element  0 in GF(2)
// 3. assign 0 to polynomial
// 4. get polynomial of [0,1] i.e. 0+1*x
// 5. assign1 to polynomial
fn main(){
	let char: u32 = 2;
	let length = 3;
	let mut pp = Polynomial::get_primitive_polynomial(char, length);
	println!("Primitive polynomial: {:?}", pp.coef);

	let element: FiniteField = FiniteField {
		char: char,
		element: Element::PrimeField { element: 0 },
	};

	let ans = pp.assign_value(element);
	println!("Assign value: {:?}", ans);


	let element0 = FiniteField {
		char: char,
		element: Element::PrimeField { element: 0 },
	};
	let element1 = FiniteField {
		char: char,
		element: Element::PrimeField { element: 1 },
	};

	let mut func: Polynomial = Polynomial {
		coef: vec![element0.clone(), element1.clone()],
	};
	println!("Function: {:?}", func.coef);

	let ans = func.assign_value(element1);
	println!("Assign value: {:?}", ans);
	
	
}
