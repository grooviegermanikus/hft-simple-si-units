use simple_si_units::hft::*;

#[test]
fn foo() {
    let size = Size::from(50);
    let price = Price::from(1400);

    let notional = price * size;

    println!("notional = {}", notional);

    // let _zzz = size * size;


    assert_eq!(size * price, price * size, "commutative property");

}