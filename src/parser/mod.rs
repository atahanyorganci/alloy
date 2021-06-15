use pest::Parser;

#[derive(Parser)]
#[grammar = "parser/alloy.pest"]
pub struct AlloyParser;

pub fn alloy_integer(integer: &str) -> Result<i32, ()> {
    let replaced = integer.replace(|ch| ch == ' ' || ch == '_', "");
    match replaced.parse::<i32>() {
        Ok(int) => Ok(int),
        Err(_) => Err(()),
    }
}

pub fn alloy_float(integer: &str) -> Result<f64, ()> {
    let replaced = integer.replace(|ch| ch == ' ' || ch == '_', "");
    match replaced.parse::<f64>() {
        Ok(float) => Ok(float),
        Err(_) => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::{alloy_float, alloy_integer, AlloyParser, Parser, Rule};

    fn test_integer(string: &str, number: i32) {
        let int = AlloyParser::parse(Rule::integer, string).unwrap();
        assert_eq!(alloy_integer(int.as_str()).unwrap(), number);
    }

    fn test_float(string: &str, number: f64) {
        let int = AlloyParser::parse(Rule::float, string).unwrap();
        assert_eq!(alloy_float(int.as_str()).unwrap(), number);
    }

    #[test]
    fn parse_integer() {
        test_integer("10", 10);
        test_integer("1_000", 1_000);
        test_integer("1_000_000", 1_000_000);
        test_integer("- 100", -100);
        test_integer("- 1_200", -1200);
        test_integer("-100", -100);
        test_integer("-1_200", -1200);
        test_integer("+ 100", 100);
        test_integer("+ 1_200", 1200);
        test_integer("+100", 100);
        test_integer("+1_200", 1200);
    }

    #[test]
    fn overflow_test() {
        assert!(alloy_integer("1_000_000_000_000").is_err());
    }

    #[test]
    fn parse_float() {
        test_float("1.0", 1.0);
        test_float("-1.2", -1.2);
        test_float(".2", 0.2);
        test_float("1.", 1.0);
        test_float("-1.", -1.0);
        test_float("-.2", -0.2);
    }
}
