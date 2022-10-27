
# Table of Contents

1.  [finite fields](#org3259b18)
2.  [What makes it different from other libraries?](#org7978785)
    1.  [Pros:](#orgd403e91)
    2.  [Cons:](#org2790d14)
3.  [Usage](#org75399a8)
4.  [Examples](#orgc0e3013)
    1.  [Prime Field](#org6bbe7cf)
    2.  [Galois Field](#org71ba6be)
    3.  [Polynomial over F<sub>p</sub>](#orgb8a2cb5)
    4.  [Polynomial over GF(p<sup>n</sup>)](#org99d0356)


<a id="org3259b18"></a>

# finite fields

A Rust library for operations on finite field, featuring:

-   Sum, difference, product, and quotient of elements F<sub>p</sub>
-   Sum, difference, product, and quotient of elements GF(p<sup>n</sup>)
-   Obtaining the primitive polynomial
-   Sum, difference, product, quotient, and remainder of polynomial over F<sub>p</sub>
-   Sum, difference, product, quotient, and remainder of polynomial over GF(p<sup>n</sup>)


<a id="org7978785"></a>

# What makes it different from other libraries?


<a id="orgd403e91"></a>

## Pros:

-   Can be calculated with F<sub>p</sub>, GF(p<sup>n</sup>) for any prime and any multiplier, not limited to a char 2.
-   Can freely compute four types of polynomials: prime, Galois, polynomial of prime, and polynomial of Galois.
-   Each can be calculated with +-\*/, so you can write natural code.


<a id="org2790d14"></a>

## Cons:

-   It takes longer than other libraries because it is not optimized for each character.


<a id="org75399a8"></a>

# Usage

Add this to your Cargo.toml:

    [dependencies]
    finite_fields = "0.1.0"


<a id="orgc0e3013"></a>

# Examples


<a id="org6bbe7cf"></a>

## Prime Field

    use finite_field::*;
    
    // consider F_5
    let char:u32 = 5
    let x:FiniteField = FiniteField{
        char:char,
        element:Element::PrimeField(2)
        };
    let y:FiniteField = FiniteField{
        char:char,
        element:Element::PrimeField(3)
        };
    println!("x + y = {:?}", x + y);
    println!("x - y = {:?}", x - y);
    println!("x * y = {:?}", x * y);
    println!("x / y = {:?}", x / y);


<a id="org71ba6be"></a>

## Galois Field

    use finite_field::*;
    
    // consider GF(5^3)
    let char:u32 = 5
    let n:u32 = 3
    let primitive_polynomial = Polynomial::get_primitive_polynomial(char, n);
    let element1:Element = Element::GaloisField{element:vec![4,2,3,4,3],primitive_polynomial:primitive_polynomial.clone()};
    let element2:Element = Element::GaloisField{element:vec![1,2,3,1],primitive_polynomial:primitive_polynomial.clone()};
    
    let x: FiniteField = FiniteField {
        char: char,
        element: element1,
    };
    let y: FiniteField = FiniteField {
        char: char,
        element: element2,
    };
        println!("x + y = {:?}", (x + y).element);
        println!("x - y = {:?}", (x - y).element);
        println!("x * y = {:?}", (x * y).element);
        println!("x / y = {:?}", (x / y).element);


<a id="orgb8a2cb5"></a>

## Polynomial over F<sub>p</sub>

    use finite_field::*;
    
    let char:u32 = 2;
    
    let element0 = FiniteField {
        char: char,
        element: Element::PrimeField { element: 0 },
    };
    let element1 = FiniteField {
        char: char,
        element: Element::PrimeField { element: 1 },
    };
    
    let f1: Polynomial = Polynomial {
        char: char,
        coef: vec![element0.clone(), element1.clone()], // 0 + 1*x = x
    };
    let f2: Polynomial = Polynomial {
        char: char,
        coef: vec![element0.clone(), element1.clone(), element1.clone()], // 0 + 1*x + 1*x^2 = x + x^2
    };
    println!("f1 + f2 = {:?}", (f1 + f2).coef);
    println!("f1 - f2 = {:?}", (f1 - f2).coef);
    println!("f1 * f2 = {:?}", (f1 * f2).coef);
    println!("f1 / f2 = {:?}", (f1 / f2).coef);
    println!("f1 % f2 = {:?}", (f1 % f2).coef);


<a id="org99d0356"></a>

## Polynomial over GF(p<sup>n</sup>)

Same as above

