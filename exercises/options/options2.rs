// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        match optional_target {
            Some(word)=>assert_eq!(word, target),
            None=>panic!("This should never happen!")
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        // 这里没有搞懂怎么做while循环的解法
        match optional_integers.pop() {
            Some(num)=> {
                assert_eq!(num, Some(cursor));
                // 解包
                cursor -= num.unwrap();
            },
            None=>(),
        }


        assert_eq!(cursor, 0);
    }
}
