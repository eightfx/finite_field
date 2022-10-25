use std::ops;
// type of number , ex: i32
type NumType = i32;

#[derive(Debug)]
enum Element{
	PrimeField(NumType),
	Polynomial(Vec<NumType>),
	GaloisField(Vec<NumType>,Vec<NumType>),
}

#[derive(Debug)]
struct FiniteField {
    char: u32,
    element: Element,
}

// prime fields
impl ops::Add for FiniteField {
	type Output = FiniteField;
	fn add(self, other: FiniteField) -> FiniteField{
		match self.element {
			Element::PrimeField(_) =>{
				let mut x:NumType = 0;
				let mut y:NumType = 0;

				// get element from enum
				if let Element::PrimeField(a) = self.element {
					x = a;
				}				
				if let Element::PrimeField(a) = other.element {
					y = a;
				}
				let mut tmp = (x + y) % self.char as NumType;

				if tmp < 0 {
					tmp += self.char as NumType;
				}else if tmp == self.char as NumType {
					tmp = 0;
				}

				FiniteField{
					char:self.char ,
					element: Element::PrimeField(tmp)
				}
			}
			Element::Polynomial(_) =>{
				let mut result:Vec<NumType> = Vec::new();
				let mut f:Vec<NumType> = Vec::new();
				let mut g:Vec<NumType> = Vec::new();

				// get element from enum
				if let Element::Polynomial(func_vec) = &self.element {
					f =  func_vec.clone();
				}
				if let Element::Polynomial(func_vec) = &other.element {
					g =  func_vec.clone();
				}

				// deg f <= deg g
				if f.len() > g.len() {
					(f,g) = (g,f);
				}

				// calculate
				for i in 0..f.len(){
					let finite_f = FiniteField{char:self.char , element:Element::PrimeField(f[i])};
					let finite_g = FiniteField{char:self.char , element:Element::PrimeField(g[i])};
					let temp = finite_f + finite_g;
					let mut answer = 0;
					if let Element::PrimeField(a) = temp.element {
						answer = a;
					}
					result.push(answer);
				}

				for i in f.len()..g.len(){
					let finite_f = FiniteField{char:self.char , element:Element::PrimeField(0)};
					let finite_g = FiniteField{char:self.char , element:Element::PrimeField(g[i])};
					let temp = finite_f + finite_g;
					let mut answer = 0;
					if let Element::PrimeField(a) = temp.element {
						answer = a;
					}
					result.push(answer);

				}
				// drop0
				let result = drop0(result);

				FiniteField{
					char:self.char ,
					element: Element::Polynomial(result)
				}
			}
			Element::GaloisField(_,_) =>{
				let mut result:Vec<NumType> = Vec::new();
				let mut f:Vec<NumType> = Vec::new();
				let mut g:Vec<NumType> = Vec::new();
				let mut h:Vec<NumType> = Vec::new();

				// get element from enum
				if let Element::GaloisField(func_vec,primitive_func) = &self.element {
					f =  func_vec.clone();
					h =  primitive_func.clone();
				}
				if let Element::GaloisField(func_vec,primitive_func) = &other.element {
					g =  func_vec.clone();
					h =  primitive_func.clone();
				}

				// deg f <= deg g
				if f.len() > g.len() {
					(f,g) = (g,f);
				}

				// calculate
				for i in 0..f.len(){
					let finite_f = FiniteField{char:self.char , element:Element::PrimeField(f[i])};
					let finite_g = FiniteField{char:self.char , element:Element::PrimeField(g[i])};
					let temp = finite_f + finite_g;
					let mut answer = 0;
					if let Element::PrimeField(a) = temp.element {
						answer = a;
					}
					result.push(answer);
				}

				for i in f.len()..g.len(){
					let finite_f = FiniteField{char:self.char , element:Element::PrimeField(0)};
					let finite_g = FiniteField{char:self.char , element:Element::PrimeField(g[i])};
					let temp = finite_f + finite_g;
					let mut answer = 0;
					if let Element::PrimeField(a) = temp.element {
						answer = a;
					}
					result.push(answer);

				}


				// drop0
				let result = drop0(result);
				FiniteField{
					char:self.char ,
					element: Element::GaloisField(result, h)
				}
			}
			

			
		}
	}
}

impl ops::Sub for FiniteField {
	type Output = FiniteField;
	fn sub(self, other: FiniteField) -> FiniteField{
		match self.element {
			Element::PrimeField(_) =>{
				let mut x:NumType = 0;
				let mut y:NumType = 0;

				// get element from enum
				if let Element::PrimeField(a) = self.element {
					x = a;
				}				
				if let Element::PrimeField(a) = other.element {
					y = a;
				}
				let mut tmp = (x - y) % self.char as NumType;
				if tmp < 0 {
					tmp += self.char as NumType;
				}else if tmp == self.char as NumType {
					tmp = 0;
				}

				FiniteField{
					char:self.char ,
					element: Element::PrimeField(tmp)
				}
			}
			Element::Polynomial(_) =>{
				let mut result:Vec<NumType> = Vec::new();
				let mut f:Vec<NumType> = Vec::new();
				let mut g:Vec<NumType> = Vec::new();

				// get element from enum
				if let Element::Polynomial(func_vec) = &self.element {
					f =  func_vec.clone();
				}
				if let Element::Polynomial(func_vec) = &other.element {
					g =  func_vec.clone();
				}

				// get each degree
				let (mut max_degree, mut min_degree) = get_maxmin_degree(&f,&g);

				// calculate
				for i in 0..min_degree{
					let finite_f = FiniteField{char:self.char , element:Element::PrimeField(f[i])};
					let finite_g = FiniteField{char:self.char , element:Element::PrimeField(g[i])};
					let temp = finite_f - finite_g;
					let mut answer = 0;
					if let Element::PrimeField(a) = temp.element {
						answer = a;
					}
					result.push(answer);
				}

				for i in min_degree..max_degree{
					let mut answer = 0;
					if f.len() > g.len(){
						let finite_f = FiniteField{char:self.char , element:Element::PrimeField(f[i])};
						let finite_g = FiniteField{char:self.char , element:Element::PrimeField(0)};
						let temp = finite_f - finite_g;
						if let Element::PrimeField(a) = temp.element {
							answer = a;
						}
					}else{
						let finite_f = FiniteField{char:self.char , element:Element::PrimeField(0)};
						let finite_g = FiniteField{char:self.char , element:Element::PrimeField(g[i])};
						let temp = finite_f - finite_g;
						if let Element::PrimeField(a) = temp.element {
							answer = a;
						}
					}

					result.push(answer);
				}

				// drop0
				let result = drop0(result);

				FiniteField{
					char:self.char ,
					element: Element::Polynomial(result)
				}
			}
			Element::GaloisField(_,_) =>{
				let mut result:Vec<NumType> = Vec::new();
				let mut f:Vec<NumType> = Vec::new();
				let mut g:Vec<NumType> = Vec::new();
				let mut h:Vec<NumType> = Vec::new();

				// get element from enum
				if let Element::GaloisField(func_vec,primitive_func) = &self.element {
					f =  func_vec.clone();
					h =  primitive_func.clone();
				}
				if let Element::GaloisField(func_vec,primitive_func) = &other.element {
					g =  func_vec.clone();
					h =  primitive_func.clone();
				}

				// get each degree
				let (mut max_degree, mut min_degree) = get_maxmin_degree(&f,&g);


				// calculate
				for i in 0..min_degree{
					let finite_f = FiniteField{char:self.char , element:Element::PrimeField(f[i])};
					let finite_g = FiniteField{char:self.char , element:Element::PrimeField(g[i])};
					let temp = finite_f - finite_g;
					let mut answer = 0;
					if let Element::PrimeField(a) = temp.element {
						answer = a;
					}
					result.push(answer);
				}

				for i in min_degree..max_degree{
					let mut answer = 0;
					if f.len() > g.len(){
						let finite_f = FiniteField{char:self.char , element:Element::PrimeField(f[i])};
						let finite_g = FiniteField{char:self.char , element:Element::PrimeField(0)};
						let temp = finite_f - finite_g;
						if let Element::PrimeField(a) = temp.element {
							answer = a;
						}
					}else{
						let finite_f = FiniteField{char:self.char , element:Element::PrimeField(0)};
						let finite_g = FiniteField{char:self.char , element:Element::PrimeField(g[i])};
						let temp = finite_f - finite_g;
						if let Element::PrimeField(a) = temp.element {
							answer = a;
						}
					}

					result.push(answer);
				}


				// drop0
				let result = drop0(result);
				FiniteField{
					char:self.char ,
					element: Element::GaloisField(result, h)
				}
			}
			

			
		}
	}
}

fn drop0(vec:Vec<NumType>) -> Vec<NumType>{
	// drop0
	let mut vec_inverse = vec.into_iter().rev().collect::<Vec<NumType>>();
	for i in 0..vec_inverse.len()-1{
		if vec_inverse[0] != 0 {
			break;
		}
		else{
			vec_inverse.remove(0);
		}
	}
	let vec = vec_inverse.into_iter().rev().collect::<Vec<NumType>>();
	vec
}

fn get_maxmin_degree(f:&Vec<NumType>,g:&Vec<NumType>) -> (usize,usize){
	let mut max_degree = 0;
	let mut min_degree = 0;
	if f.len() < g.len() {
		max_degree = g.len();
		min_degree = f.len();
	}
	else{
		max_degree = f.len();
		min_degree = g.len();
	}
	(max_degree,min_degree)
}

fn main() {
	let char:u32 =5;
	// let element1:Element = Element::PrimeField(2);
	// let element2:Element = Element::PrimeField(3);

	// let element1:Element = Element::Polynomial(vec![1,2,3,4,5]);
	// let element2:Element = Element::Polynomial(vec![1,2,3,1]);
	
	let element1:Element = Element::GaloisField(vec![4,2,3,4,3],vec![2,2,3]);
	let element2:Element = Element::GaloisField(vec![3,3,3,4,3],vec![2,2,3]);
	
	let x:FiniteField = FiniteField{char:char,element:element1};
	let y:FiniteField = FiniteField{char:char,element:element2};
	println!("{:?}",(x-y).element);

}
