
fn main() {
	// character
    let char: u32 = 5;

	// Four arithmetic operations between Prime Field elements
    let element1:Element = Element::PrimeField(2);
    let element2:Element = Element::PrimeField(3);
    let x: FiniteField = FiniteField {
        char: char,
        element: element1,
    };
    let y: FiniteField = FiniteField {
        char: char,
        element: element2,
    };
    println!("{:?}", (y - x).element);


	// Four arithmetic operations between Polynomial on Prime Field elements
    let element1: Element = Element::Polynomial(vec![1, 2, 3, 4, 5]);
    let element2: Element = Element::Polynomial(vec![1, 2, 3, 1]);
    let x: FiniteField = FiniteField {
        char: char,
        element: element1,
    };
    let y: FiniteField = FiniteField {
        char: char,
        element: element2,
    };
    println!("{:?}", (y + x).element);


	// Four arithmetic operations between GaloisField elements
	// we need primitive_polynomial
	let element1:Element = Element::GaloisField{element:vec![4,2,3,4,3],primitive_polynomial:vec![2,2,3]};
	let element2:Element = Element::GaloisField{element:vec![1,2,3,1],primitive_polynomial:vec![2,2,3]};

	let x: FiniteField = FiniteField {
        char: char,
        element: element1,
    };
    let y: FiniteField = FiniteField {
        char: char,
        element: element2,
    };
    println!("{:?}", (y - x).element);
}
