use std::ops;
// type of number , ex: i32
type NumType = i32;

#[derive(Debug)]
enum Element{
	PrimeField(NumType),
	Polynomial(Vec<NumType>),
	GaloisField(Vec<NumType>,Vec<NumType>),
}

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
			Element::Polynomial(_) =>{
				let mut result:Vec<NumType> = Vec::new();
				let mut f:Vec<NumType> = Vec::new();
				let mut g:Vec<NumType> = Vec::new();

				if let Element::Polynomial(func_vec) = &self.element {
					f =  func_vec.clone();
				}
				if let Element::Polynomial(func_vec) = &other.element {
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
					element: Element::Polynomial(result)
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
	let element1:Element = Element::PrimeField(3);
	let element2:Element = Element::PrimeField(2);

	// let element1:Element = Element::GaloisField(vec![1,2,3],vec![2,2,3]);
	// let element2:Element = Element::GaloisField(vec![1,2],vec![2,2,3]);
	
	let x:FiniteField = FiniteField{char:char,element:element1};
	let y:FiniteField = FiniteField{char:char,element:element2};
	println!("{:?}",(x+y).element);

}
