use simple_si_units::base::Distance;
use simple_si_units::geometry::Area;

#[test]
fn simple() {

    // cylinder
    let base = Area::from_m2(40);
    let height = Distance::from_m(8);
    let len = Distance::from_m(3);

    println!("volume = {}", base * height);

    println!("volume = {}", height * base);

    println!("len^2 = {}", len * len);

    println!("len^3 = {}", len * len * len);

    // println!("len^4 = {}", len * len * len * len);

    // cannot multiply `Area<{integer}>` by `Area<{integer}>`
    // println!("volume = {}", base * base);

}

#[test]
fn sec2() {
    //

}