fn rgb(r: i32, g: i32, b: i32) -> String {
    let fix = |v| {
        if v < 0 {
            return 0;
        }
        if v > 255 {
            return 255;
        }
        v
    };
    format!("{:02X}{:02X}{:02X}", fix(r), fix(g), fix(b))
}

macro_rules! compare {
    ( $got : expr, $expected : expr ) => {
        if $got != $expected {
            panic!("Got: {}\nExpected: {}\n", $got, $expected);
        }
    };
}

#[cfg(test)]
mod sample_tests {
    use self::super::*;

    #[test]
    fn tests() {
        compare!(rgb(0, 0, 0), "000000");
        compare!(rgb(1, 2, 3), "010203");
        compare!(rgb(255, 255, 255), "FFFFFF");
        compare!(rgb(254, 253, 252), "FEFDFC");
        compare!(rgb(-20, 275, 125), "00FF7D");
    }
}
