#[cfg(test)]
mod test {
    use crate::parser::{check_matching_pair, check_symbol_string, get_content_type, get_expression_data, ContentType, ExpressionData, TagType};

    #[test]
    fn check_literal_test() {
        let s = "<h1>Hello world</h1>";
        assert_eq!(ContentType::Literal(s.to_string()), get_content_type(s));
    }    
    #[test]
    fn check_template_car_test() {
        let content = ExpressionData {
            head: Some("Hi ".to_string()),
            variable: "name".to_string(),
            tail: Some(" ,welcome".to_string())
        };
        assert_eq!(
            ContentType::TemplateVariable(content),
            get_content_type("Hi {{name}} ,welcome")
        )
    }
    #[test]
    fn check_for_tag_test() {
        assert_eq!(
            ContentType::Tag(TagType::ForTag),
            get_content_type("{% for name in names %} bye")
        );
    }
    #[test]
    fn check_if_tag_test() {
        assert_eq!(
            ContentType::Tag(TagType::IfTag),
            get_content_type("{% if name == 'Bob' endif %}")
        );
    }
    // helper functions tests
    #[test]
    fn check_symbol_string_test() {
        assert_eq!(true, check_symbol_string("{{Hello}}", "{{"))
    }
    #[test]
    fn check_matching_pair_test() {
        assert_eq!(true, check_matching_pair("{{Hello}}", "{{", "}}"))
    }
     #[test]
    fn check_get_expression_data_test() {
        let expression_data = ExpressionData {
            head: Some("Hi ".to_string()),
            variable: "name".to_string(),
            tail: Some(",welcome".to_string()),
        };

        assert_eq!(expression_data, get_expression_data("Hi {{name}},welcome"));
    }
    #[test]
    fn check_symbol_pair_test() {
        assert_eq!(true, check_matching_pair("{{Hello}}", "{{", "}}"));
    }
}
