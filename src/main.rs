// type of number , ex: i32
type NumType = i32;

#[derive(Debug)]
enum Element{
	PrimeField(NumType),
	PolynomialField(Vec<NumType>),
	GaloisField(Vec<NumType>,Vec<NumType>),
}

struct FiniteField {
    char: u32,
    element: Element,
}

// polynomial
// impl FiniteField{
//     fn add(&self, other: &FiniteField<Vec<NumType>>) ->FiniteField<Vec<NumType>>{
// 		let mut result_elements = Vec::new();
// 		let mut f:Vec<NumType> = Vec::new();
// 		let mut g:Vec<NumType> = Vec::new();

// 		if self.element.len() > other.element.len() {
// 			f =  self.element.clone();
// 			g =  other.element.clone();
// 		}
// 		else{
// 			f =  other.element.clone();
// 			g =  self.element.clone();

// 		}
// 		for i in 0..g.len(){
// 			result_elements.push(f[i] + g[i]);
// 		}
// 		for i in g.len()..f.len(){
// 			result_elements.push(f[i]);
// 		}
// 		FiniteField{char: self.char, element: result_elements}
// 	}
// }

// prime fields
impl FiniteField {
	fn add(&self, other: &FiniteField) -> FiniteField{

		match self.element {
			Element::PrimeField(_) =>{
				let mut x:NumType = 0;
				let mut y:NumType = 0;

				// get element
				if let Element::PrimeField(a) = self.element {
					x = a;
				}				
				if let Element::PrimeField(a) = other.element {
					y = a;
				}

				FiniteField{
					char: self.char,
					element: Element::PrimeField((x + y) % self.char as NumType)
				}
			}
			Element::PolynomialField(_) =>{
				let mut result:Vec<NumType> = Vec::new();
				let mut f:Vec<NumType> = Vec::new();
				let mut g:Vec<NumType> = Vec::new();

				if let Element::PolynomialField(func_vec) = &self.element {
					f =  func_vec.clone();
				}
				if let Element::PolynomialField(func_vec) = &other.element {
					g =  func_vec.clone();
				}

				if f.len() < g.len() {
					(f,g) = (g,f);
				}

				for i in 0..g.len(){
					result.push((f[i] + g[i]) % self.char as NumType);
				}
				for i in g.len()..f.len(){
					result.push(f[i]);
				}
				FiniteField{
					char: self.char,
					element: Element::PolynomialField(result)
				}
			}
			Element::GaloisField(_,_) =>{
				let mut result:Vec<NumType> = Vec::new();
				let mut max_degree = 0;
				let mut f:Vec<NumType> = Vec::new();
				let mut g:Vec<NumType> = Vec::new();
				let mut h:Vec<NumType> = Vec::new();

				if let Element::GaloisField(func_vec,primitive_func) = &self.element {
					f =  func_vec.clone();
					h =  primitive_func.clone();
				}
				if let Element::GaloisField(func_vec,primitive_func) = &other.element {
					g =  func_vec.clone();
					h =  primitive_func.clone();
				}

				if f.len() < g.len() {
					(f,g) = (g,f);
				}

				for i in 0..g.len(){
					result.push((f[i] + g[i]) % self.char as NumType);
				}
				for i in g.len()..f.len(){
					result.push(f[i]);
				}

				FiniteField{
					char: self.char,
					element: Element::GaloisField(result, h)
				}
			}
			

			
		}
	}
}


fn main() {
	let char:u32 = 5;
	// let element1:Element = Element::PrimeField(1 as NumType);
	// let element2:Element = Element::PrimeField(2 as NumType);

	let element1:Element = Element::GaloisField(vec![1,2,3],vec![2,2,3]);
	let element2:Element = Element::GaloisField(vec![1,2],vec![2,2,3]);
	
	let x:FiniteField = FiniteField{char:char,element:element1};
	let y:FiniteField = FiniteField{char:char,element:element2};
	println!("{:?}",x.add(&y).element);

}
