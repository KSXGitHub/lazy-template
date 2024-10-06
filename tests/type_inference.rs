#![cfg(feature = "std")]
use lazy_template::simple_curly_braces;

#[test]
fn infer_type_of_query_param_of_closure() {
    let _ = simple_curly_braces().lazy_parse("").to_string(|query| {
        let _ = query.to_string();
        Ok::<_, ()>(0)
    });
    let _ = || {
        simple_curly_braces()
            .eager_parse::<Vec<_>>("")
            .unwrap()
            .to_template()
            .to_string(|query| {
                let _ = query.to_string();
                Ok::<_, ()>(0)
            })
    };
}
