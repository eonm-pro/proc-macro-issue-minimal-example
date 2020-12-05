#[cfg(test)]
mod tests {
    use proc_macro_issue_minimal_example::add_field;

    #[test]
    fn add_field() {
        #[add_field]
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
        #[add_field]
        #[derive(Debug, Clone)]
        struct Foo {
            b: String,
        }

        // Foo should be expand to :
        // struct Foo {
        //  pub a: String
        //  pub b: String,
        // }

        let bar = Foo {
            a: "lorem ipsum".to_string(),
            b: "dolor sit amet".to_string(),
        };

        assert_eq!(bar.a, "lorem ipsum");
        assert_eq!(bar.b, "dolor sit amet");
    }
}
