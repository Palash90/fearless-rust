#[derive(PartialEq, Debug)]
enum Scale {
    Celsius,
    Farhenheit,
}

#[derive(PartialEq, Debug)]
pub struct Temparature {
    value: f64,
    scale: Scale,
}

impl Temparature {
    fn new(value: f64, scale: Scale) -> Self {
        Temparature { value, scale }
    }
}

pub fn temp_conv(t: Temparature) -> Temparature {
    match t.scale {
        Scale::Celsius => {
            let value = (t.value * 1.8) + 32_f64;
            Temparature::new(value, Scale::Farhenheit)
        }
        Scale::Farhenheit => {
            let value = (t.value - 32_f64) * 5_f64 / 9_f64;
            Temparature::new(value, Scale::Celsius)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_to_f_works() {
        let source = Temparature::new(1_f64, Scale::Celsius);
        let expected = Temparature::new(33.8, Scale::Farhenheit);

        let derived = temp_conv(source);
        assert_eq!(expected, derived);
    }

    
}
