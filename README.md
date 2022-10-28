
# Table of Contents

1.  [finite fields](#orgbe48526)
2.  [What makes it different from other libraries?](#org8c1409c)
    1.  [Pros:](#orgcd196c8)
    2.  [Cons:](#org9f1220e)
3.  [Usage](#org0868a6e)
4.  [Examples](#orgd05353d)
    1.  [Prime Field](#org6bf6bb3)
    2.  [Galois Field](#orgcf32892)
    3.  [Polynomial over F<sub>p</sub>](#org684dac7)
    4.  [Polynomial over GF(p<sup>n</sup>)](#org4ec771b)
    5.  [Matrix over FiniteField](#orge3587d4)


<a id="orgbe48526"></a>

# finite fields

A Rust library for operations on finite field, featuring:

-   Sum, difference, product, and quotient of elements F<sub>p</sub>
-   Sum, difference, product, and quotient of elements GF(p<sup>n</sup>)
-   Obtaining the primitive polynomial
-   Sum, difference, product, quotient, and remainder of polynomial over F<sub>p</sub>
-   Sum, difference, product, quotient, and remainder of polynomial over GF(p<sup>n</sup>)
-   Sum, product of Matrix F<sub>p</sub>
-   Sum, product of Matrix GF(p<sup>n</sup>)
-   The sweep method (or Gaussian elimination) of matrices on finite bodies (F<sub>p</sub>, GF(p<sup>n</sup>)) is also available.


<a id="org8c1409c"></a>

# What makes it different from other libraries?


<a id="orgcd196c8"></a>

## Pros:

-   Can be calculated with F<sub>p</sub>, GF(p<sup>n</sup>) for any prime and any multiplier, not limited to a char 2.
-   Can freely compute six types of element: prime field, Galois field, polynomial of prime field, polynomial of Galois field, matrix of prime field, and matrix of Galois field.
-   Each can be calculated with +-\*/, so you can write natural code.
-   Matrix operations on finite field (F<sub>p</sub>, GF(p<sup>n</sup>)) can also be performed.
-   The sweep method can be available.


<a id="org9f1220e"></a>

## Cons:

-   It takes longer than other libraries because it is not optimized for each character.


<a id="org0868a6e"></a>

# Usage

Add this to your Cargo.toml:

    [dependencies]
    galois_field = "0.1.9"


<a id="orgd05353d"></a>

# Examples


<a id="org6bf6bb3"></a>

## Prime Field

    use galois_field::*;
    
    let char: u32 = 5;
    let x:FiniteField = FiniteField{
    	char: char,
    	element:Element::PrimeField{element:0} // 0 in F_5
    };
    let y:FiniteField = FiniteField{
    	char: char,
    	element:Element::PrimeField{element:1} // 1 in F_5
    };
    println!("x + y = {:?}", (x.clone() + y.clone()).element); // ->1
    println!("x - y = {:?}", (x.clone() - y.clone()).element); // -> 4
    println!("x * y = {:?}", (x.clone() * y.clone()).element); // -> 0
    println!("x / y = {:?}", (x.clone() / y.clone()).element); // -> 0


<a id="orgcf32892"></a>

## Galois Field

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


<a id="org684dac7"></a>

## Polynomial over F<sub>p</sub>

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


<a id="org4ec771b"></a>

## Polynomial over GF(p<sup>n</sup>)

Same as above


<a id="orge3587d4"></a>

## Matrix over FiniteField

    use galois_field::*;
    
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
    let mut matrix = Matrix{
        element: matrix_element,
    };
    
    println!("m+m = {:?}", m.clone()+m.clone());
    println!("m*m = {:?}", m.clone()*m.clone());
    
    let mut sweep_matrix = m.sweep_method();
    println!("{:?}", sweep_matrix);

