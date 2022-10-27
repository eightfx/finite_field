use galois_field::*;

fn main() {
	// character
    let char: u32 = 2;

	let element0:FiniteField = FiniteField{
		char: char,
		element:Element::PrimeField{element:0} // 0 in F_5
	};
	let element1:FiniteField = FiniteField{
		char: char,
		element:Element::PrimeField{element:1} // 1 in F_5
	};


	let f: Polynomial = Polynomial {
        coef: vec![element1.clone(),element0.clone(),element0.clone(),element0.clone(),element1.clone()]
	};
    let g: Polynomial = Polynomial {
		coef: vec![element1.clone(),element0.clone(),element0.clone(),element1.clone(),element1.clone()]
    };
    println!("f + g = {:?}", (f.clone()+g.clone()).coef);
	println!("f - g = {:?}", (f.clone()-g.clone()).coef);
	println!("f * g = {:?}", (f.clone()*g.clone()).coef);
	println!("f / g = {:?}", (f.clone()/g.clone()).coef);
	println!("f % g = {:?}", (f.clone()%g.clone()).coef);
	
}
