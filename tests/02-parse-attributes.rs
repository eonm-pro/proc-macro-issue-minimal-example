#[cfg(test)]
mod tests {
    use proc_macro_issue_minimal_example::add_field2;

    #[test]
    fn add_field() {
        #[add_field2("a")]
        #[derive(Debug, Clone)]
        struct Foo {}

        // Foo should be expand to :
        // struct Foo {
        //  pub a: String
        // }

        let bar = Foo {
            a: "lorem ipsum".to_string(),
        };

        assert_eq!(bar.a, "lorem ipsum");
    }

    #[test]
    fn keep_original_field() {
        #[add_field2("a", "b")]
        #[derive(Debug, Clone)]
        struct Foo {
            c: String,
        }

        // Foo should be expand to :
        // struct Foo {
        //  pub a: String
        //  pub b: String,
        //  pub c: String,
        // }

        let bar = Foo {
            a: "lorem ipsum".to_string(),
            b: "dolor sit amet".to_string(),
            c: "".to_string(),
        };

        assert_eq!(bar.a, "lorem ipsum");
        assert_eq!(bar.b, "dolor sit amet");
        assert_eq!(bar.c, "");
    }
}
