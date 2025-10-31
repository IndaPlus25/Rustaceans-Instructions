#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::super::compile;
    use crate::compiler;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn test() {
        let hm: HashMap<String, usize> = HashMap::new();
        println!(
            "{:?}",
            compiler::run(
                "PLEASE, ACCESS, THE FIRST REGISTRY, INPUTTING A VALUE, AS AN INTEGER. input x", &hm, &0
            )
        );
    }

    #[test]
    fn finding_label() {
        let slice = match compiler::find_labels("THE START: label") {
            Some(_index) => &"THE START: label"[.._index],
            None => ""
        };
        assert_eq!("THE START", slice)
    }

    #[test]
    fn compiler_test() {
        let args: Vec<String> = vec!("./input.sal".to_string());
        compile(args)
    }
}
