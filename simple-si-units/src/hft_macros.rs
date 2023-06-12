



#[macro_export] macro_rules! mul_div_unit {
    // $unit1 * $unit2 = $unit3
    // $unit2 * $unit1 = $unit3
    // $unit1 = $unit3 / $unit2
    // $unit2 = $unit3 / $unit1
    // unit4 = function
    ($unit1:ident, $unit2:ident, $unit3:ident, $unit4:ident) => {
        impl Mul<$unit2> for $unit1 {
            type Output = $unit3;

            fn mul(self: $unit1, rhs: $unit2) -> $unit3 {
                let lhs = self;
                $unit4(lhs, rhs)
            }
        }

        impl Mul<$unit1> for $unit2 {
            type Output = $unit3;

            fn mul(self: $unit2, rhs: $unit1) -> $unit3 {
                let lhs = self;
                $unit4(rhs, lhs)
            }
        }

        impl Div<$unit2> for $unit1 {
            type Output = $unit3;

            fn div(self: $unit1, rhs: $unit2) -> $unit3 {
                let lhs = self;
                $unit4(lhs, rhs)
            }
        }

        impl Div<$unit1> for $unit2 {
            type Output = $unit3;

            fn div(self: $unit2, rhs: $unit1) -> $unit3 {
                let lhs = self;
                $unit4(rhs, lhs)
            }
        }
    };

    // $unit1 * $unit1 = $unit2
    // $unit1 = $unit2 / $unit1
    ($unit1:ident, $unit2:ident) => {
        impl Mul<$unit1> for $unit1 {
            type Output = $unit2;

            fn mul(self: $unit1, $unit1(rhs): $unit1) -> $unit2 {
                let $unit1(lhs) = self;
                $unit2(lhs * rhs)
            }
        }

        impl Div<$unit1> for $unit2 {
            type Output = $unit1;

            fn div(self: $unit2, $unit1(rhs): $unit1) -> $unit1 {
                let $unit2(lhs) = self;
                $unit1(lhs / rhs)
            }
        }
    }
}
