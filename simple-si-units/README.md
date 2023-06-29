
## What is this?
* attempt model price + size for trading domain using the same approach as simple_units ([GitHub](https://github.com/willi-kappler/simple_units/tree/master))
* adoptions of simple_units's macros (e.g. _mul_unit!_) to support more custom mul/div implementations

## Where to start?
* look at test _hft.rs_ -> _size_and_price_
* look into _src/notional.rs_

## Next steps
* continue to implement calculation in test case _sol_trace_
* collect all types and combinations (similar to _simple_units_)

## Alternatives considered
* uom
* fts_units
* simple-si-units
* dimensioned
