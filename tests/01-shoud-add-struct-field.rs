#[cfg(test)]
mod tests {
    use proc_macro_issue_minimal_example::AddField;

    
    #[test]
    fn add_field() {
        
        #[derive(Debug, Clone, AddField)]
        struct Foo {}

        // Foo should be expand to :
        // struct Foo {
        //  pub a: String
        // }

        let bar = Foo { a: "lorem ipsum".to_string()};
    }

}