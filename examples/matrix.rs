use galois_field::*;

fn main(){
	let char = 3;
	let element0: FiniteField = FiniteField {
		char: char,
		element: Element::PrimeField { element: 0 },
	};
	let element1: FiniteField = FiniteField {
		char: char,
		element: Element::PrimeField { element: 1 },
	};
	let element2: FiniteField = FiniteField {
		char: char,
		element: Element::PrimeField { element: 2 },
	};


	let mut matrix_element:Vec<Vec<FiniteField>> = vec![
		vec![element0.clone(),element1.clone(), element0.clone()],
		vec![element2.clone(),element2.clone(), element1.clone()],
		vec![element1.clone(),element0.clone(), element1.clone()]
	];
	let mut m = Matrix{
		element: matrix_element,
	};


	println!("m+m = {:?}", m.clone()+m.clone());
	println!("m*m = {:?}", m.clone()*m.clone());

	let mut sweep_matrix = m.sweep_method();
	println!("{:?}", sweep_matrix);





	
}
