mod finite;
use crate::finite::*;


fn main() {
	let char: u32 = 2;
	let length = 4;
	let pp = get_primitive_polynomial(char, length);
	println!("Primitive polynomial: {:?}", pp.coef);
	
	let element1: Element = Element::PrimeField { element: 0 };
	let element2: Element = Element::PrimeField { element: 1 };
	let element3: Element = Element::PrimeField { element: 1 };

	// let coefs = vec![
	// 	FiniteField {
	// 		char: char,
	// 		element: Element::PrimeField { element: 1 },
	// 	},
	// 	FiniteField {
	// 		char: char,
	// 		element: Element::PrimeField { element: 1 },
	// 	},
	// 	FiniteField {
	// 		char: char,
	// 		element: Element::PrimeField { element: 0 },
	// 	},
	// 	FiniteField {
	// 		char: char,
	// 		element: Element::PrimeField { element: 0 },
	// 	},
	// 	FiniteField {
	// 		char: char,
	// 		element: Element::PrimeField { element: 1 },
	// 	},
	// ];
	// let primitive_polynomial:Polynomial = Polynomial{
	// 	char:char,
	// 	coef: coefs,
	// };
	
	// let element1: Element = Element::GaloisField {
	// 	element: vec![1, 0, 0, 1],
	// 	primitive_polynomial: primitive_polynomial.clone(),
	// };
	// let element2: Element = Element::GaloisField {
	// 	element: vec![1, 0, 0, 0],
	// 	primitive_polynomial: primitive_polynomial.clone(),
	// };
	// let element3: Element = Element::GaloisField {
	// 	element: vec![1, 1, 0, 1],
	// 	primitive_polynomial: primitive_polynomial.clone(),
	// };

	let element1: FiniteField = FiniteField {
		char: char,
		element: element1,
	};
	let element2: FiniteField = FiniteField {
		char: char,
		element: element2,
	};
	let element3: FiniteField = FiniteField {
		char: char,
		element: element3,
	};
	let poly1 = Polynomial {
    	char: char,
    	coef: vec![element2.clone(), element1.clone(), element2.clone()],
	};
	let poly2 = Polynomial {
    	char: char,
    	coef: vec![element2.clone(),element2.clone()],
	};

	// // println!("{:?}", (element2*element1).element);
	// println!("{:?}", (poly1/poly2).coef); 
	// println!("{:?}", poly1.gcd(poly2).coef);
}
