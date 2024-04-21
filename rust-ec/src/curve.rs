use std::str::FromStr;

use num_bigint::BigInt;
use num_traits::Zero;

pub mod params;
use params::{two, three};

mod math;
pub use math::modulo;

#[derive(Clone, Debug)]
pub struct Point{
    pub x: Option<BigInt>,
    pub y: Option<BigInt>
}
#[derive(Clone, Debug)]
pub struct Curve{
    pub a: BigInt,
    pub b: BigInt,
    pub p: BigInt
}

impl Curve{
    pub fn point_addition(&self, P: &Point, Q: &Point) -> Point{
        let two = two();
        let three = three();

        if (P.x.is_none()) && (P.y.is_none()){
            return Q.to_owned()
        };
        if (Q.x.is_none()) && (Q.y.is_none()){
            return P.to_owned()
        };

        // expect points to have x and y coordinate
        let x1: BigInt = P.x.clone().expect("P missing x coordinate");
        let y1: BigInt = P.y.clone().expect("P missing y coordinate");
        let x2: BigInt = Q.x.clone().expect("Q missing x coordinate");
        let y2: BigInt = Q.y.clone().expect("Q missing y coordinate");

        // calculate the gradient
        let mut m: BigInt = BigInt::zero();
        if (x1 == x2) && (y1 == y2){
            // calculate the slope of the tangent line to find curve intersections
            // division in modular arithmetic is equivalent to multiplying by the modular inverse
            m = modulo(&(modulo(&(three*&x1*&x1+&self.a),&self.p) * (modulo(&(&two*&y1),&self.p)).modpow(&(&self.p-&two), &self.p)),&self.p)
        }
        else{
            m = modulo(&((&y2-&y1)%&self.p * ((&x2-&x1)%&self.p).modpow(&(&self.p-&two), &self.p)),&self.p) //modular_inverse(&((&y2-&y1)%&self.p * ((&x2-&x1)%&self.p)), &self.p);
        };
        // evaluate new point on the curve that intersects the tangent line
        let x3: BigInt = modulo(&(&m*&m - &x1 - &x2), &self.p);
        let y3: BigInt = modulo(&(&m*(&x1-&x3) - &y1), &self.p);
        Point { 
            x: Some(x3),
            y: Some(y3) 
        }
    }

    pub fn double_and_add(&self, n: &BigInt, P: &Point) -> Point{
        if (P.x.is_none()) && (P.y.is_none()){
            return P.to_owned()
        };
        let mut temp_point = Point{
            x: P.x.clone(),
            y: P.y.clone()
        };
        let binary = format!("{:b}", n);
        for binary_char in binary.chars().skip(1) {
            temp_point = self.point_addition(&temp_point, &temp_point);
            if binary_char == '1' {
                temp_point = self.point_addition(&temp_point, &P);
            }
        }
        temp_point
    }
}


#[test]
fn verify_g2_secp256k1(){
    use params::SECP_256_K1;
    let secp = SECP_256_K1{};
    let curve: Curve = Curve{
        a: secp.a(),
        b: secp.b(),
        p: secp.p()
    };
    let point_g_2: Point = curve.double_and_add(&BigInt::from(2u8), &secp.g());
    // Todo: make this an assertion and expand on the tests
    println!("Point 2G: {:?}", &point_g_2);
    assert_eq!(&point_g_2.x.expect("Missing x-coordinate"), &BigInt::from_str("89565891926547004231252920425935692360644145829622209833684329913297188986597").expect("Failed to construct BigInt from str"));
    assert_eq!(&point_g_2.y.expect("Missing y-coordinate"), &BigInt::from_str("12158399299693830322967808612713398636155367887041628176798871954788371653930").expect("Failed to construct BigInt from str"));
}