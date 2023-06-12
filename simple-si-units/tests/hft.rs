use num_traits::CheckedMul;
use simple_si_units::hft_units::*;

#[test]
fn size_andPrice() {
    let size = Size::from(50);
    let price = Price::from(1400);

    let notional: Notional = price * size;

    println!("notional = {}", notional);

    // TODO
    // let _notional_deref = &price * &size;


    // let _zzz = size * size;


    assert_eq!(size * price, price * size, "commutative property");

}


#[test]
fn base_to_native() {

    let perp_base_lots = 10u64;
    let perp_ask_price_lots = 20_000u64;
    // perp market option:
    let market_quote_lot_size = 100u64;

    let perp_ask_quote_lots = perp_base_lots
        .checked_mul(perp_ask_price_lots)
        .unwrap();
    let perp_ask_quote_native = i64::try_from(
        perp_ask_quote_lots // price
            .checked_mul(market_quote_lot_size)
            .unwrap(),
    ).unwrap();
    assert_eq!(perp_ask_quote_native, 20000000);

}

#[test]
fn sol_trace() {
    // example sell 1 SOL (e9,l7), best bid is 19.71 USDC (e6, l2)
    // SOL-PERP: lot size = 1e7; decimals = 1e9
    // base_ui = 1
    // base_native  = 1e9
    // base_lots (SOL) = 1e9 / 1l7 = 1e2
    // price_lots (USDC) = 1971
    // quote_lots (USDC) = base_lots * price_lots = 1971e2
    // quote_native(USDC) = 1971e2 * 1l2 = 1971e4
    // quote_ui = 19.71

    let sol_native = 1_000_000_000; // =e9  smallest unit
    let sol_lot_size = 10_000_000; // =l7, number of native units in a lot
    let sol_qty_lots = 100; // qty of lots to trade

    let sol_base_ui = sol_qty_lots * sol_lot_size / sol_native;

    assert_eq!(sol_base_ui, 1);


}