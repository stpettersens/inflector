#![deny(warnings)]
use std::ascii::*;
use string::singularize::to_singular;
/// Converts a `String` to `ClassCase` `String`
///
/// #Examples
/// ```
///     use inflector::cases::classcase::to_class_case;
///     let mock_string: String = "FooBar".to_string();
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_class_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::to_class_case;
///     let mock_string: String = "FooBars".to_string();
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_class_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::to_class_case;
///     let mock_string: String = "Foo Bar".to_string();
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_class_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::to_class_case;
///     let mock_string: String = "foo-bar".to_string();
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_class_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::to_class_case;
///     let mock_string: String = "fooBar".to_string();
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_class_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::to_class_case;
///     let mock_string: String = "FOO_BAR".to_string();
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_class_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::to_class_case;
///     let mock_string: String = "foo_bar".to_string();
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_class_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::to_class_case;
///     let mock_string: String = "foo_bars".to_string();
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_class_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::to_class_case;
///     let mock_string: String = "Foo bar".to_string();
///     let expected_string: String = "FooBar".to_string();
///     let asserted_string: String = to_class_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn to_class_case(non_class_case_string: String) -> String {
    let mut new_word: bool = true;
    let mut last_char: char = ' ';
    let class_plural = non_class_case_string
        .chars()
        .fold("".to_string(), |mut result, character|
            if character == '-' || character == '_' || character == ' ' {
                new_word = true;
                result
            } else if new_word || (
                (last_char.is_lowercase() && character.is_uppercase()) &&
                (last_char != ' ')
                ){
                new_word = false;
                result.push(character.to_ascii_uppercase());
                result
            } else {
                last_char = character;
                result.push(character.to_ascii_lowercase());
                result
            }
        );
    let split: (&str, &str) = class_plural.split_at(class_plural.rfind(char::is_uppercase).unwrap_or(0));
    format!("{}{}", split.0, to_singular(split.1.to_string()))
}
/// Determines if a `String` is `ClassCase` `bool`
///
/// #Examples
/// ```
///     use inflector::cases::classcase::is_class_case;
///     let mock_string: String = "Foo".to_string();
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::is_class_case;
///     let mock_string: String = "foo".to_string();
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::is_class_case;
///     let mock_string: String = "FooBarIsAReallyReallyLongStrings".to_string();
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
///
/// ```
///     use inflector::cases::classcase::is_class_case;
///     let mock_string: String = "FooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::is_class_case;
///     let mock_string: String = "foo-bar-string-that-is-really-really-long".to_string();
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::is_class_case;
///     let mock_string: String = "foo_bar_is_a_really_really_long_strings".to_string();
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
///
/// ```
///     use inflector::cases::classcase::is_class_case;
///     let mock_string: String = "fooBarIsAReallyReallyLongString".to_string();
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::is_class_case;
///     let mock_string: String = "FOO_BAR_STRING_THAT_IS_REALLY_REALLY_LONG".to_string();
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::is_class_case;
///     let mock_string: String = "foo_bar_string_that_is_really_really_long".to_string();
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::is_class_case;
///     let mock_string: String = "Foo bar string that is really really long".to_string();
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
///
/// ```
///     use inflector::cases::classcase::is_class_case;
///     let mock_string: String = "Foo Bar Is A Really Really Long String".to_string();
///     let asserted_bool: bool = is_class_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
pub fn is_class_case(test_string: String) -> bool {
    test_string == to_class_case(test_string.clone())
}

#[cfg(all(feature = "unstable", test))]
mod tests {
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_class_case(b: &mut Bencher) {
        b.iter(|| super::to_class_case("Foo bar".to_string()));
    }

    #[bench]
    fn bench_is_class(b: &mut Bencher) {
        b.iter(|| super::is_class_case("Foo bar".to_string()));
    }

    #[bench]
    fn bench_class_from_snake(b: &mut Bencher) {
        b.iter(|| super::to_class_case("foo_bar".to_string()));
    }
}
