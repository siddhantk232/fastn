use fastn_grammar::evalexpr::ContextWithMutableFunctions;

/**
* The `default_aliases` function is intended to provide default aliases for the `ftd` module,
* with the only default alias being "ftd" itself. This allows users to reference the `ftd` module
* using this alias instead of the full module name.
**/
pub fn default_aliases() -> ftd::Map<String> {
    std::iter::IntoIterator::into_iter([
        ("ftd".to_string(), "ftd".to_string()),
        ("inherited".to_string(), "inherited".to_string()),
    ])
    .collect()
}

/**
The `default_functions` function returns a map of string keys to Function values. These functions
are built-in and available for use in the evaluation of an expression.

1. `is_empty` - This function takes an argument and returns a boolean value indicating whether or not
the argument is empty. It checks for empty values, strings, and tuples.

2. `enable_dark_mode` - This function takes no arguments and returns an empty value. It is used to
enable dark mode in the application.

3. `enable_light_mode` - This function takes no arguments and returns an empty value. It is used to
enable light mode in the application.

4. `enable_system_mode` - This function takes no arguments and returns an empty value. It is used to
enable system mode in the application, which means the application will use the system's default
color scheme.
**/
pub fn default_functions() -> ftd::Map<fastn_grammar::evalexpr::Function> {
    use fastn_grammar::evalexpr::*;

    std::iter::IntoIterator::into_iter([
        (
            "ftd.clean_code".to_string(),
            Function::new(|argument| {
                if argument.as_empty().is_ok() {
                    Ok(Value::String("".to_string()))
                } else if let Ok(s) = argument.as_string() {
                    let mut new_string = vec![];
                    for line in s.split('\n') {
                        new_string.push(
                            ftd::interpreter::FTD_HIGHLIGHTER.replace(line, regex::NoExpand("")),
                        );
                    }
                    Ok(Value::String(new_string.join("\n")))
                } else if let Ok(tuple) = argument.as_tuple() {
                    if tuple.len().ne(&2) {
                        Err(
                            fastn_grammar::evalexpr::error::EvalexprError::WrongFunctionArgumentAmount {
                                expected: 2,
                                actual: tuple.len(),
                            },
                        )
                    } else {
                        let s = tuple.first().unwrap().as_string()?;
                        let lang = tuple.last().unwrap().as_string()?;
                        if lang.eq("ftd") {
                            let mut new_string = vec![];
                            for line in s.split('\n') {
                                new_string.push(
                                    ftd::interpreter::FTD_HIGHLIGHTER
                                        .replace(line, regex::NoExpand("")),
                                );
                            }
                            Ok(Value::String(new_string.join("\n")))
                        } else {
                            Ok(Value::String(s))
                        }
                    }
                } else {
                    Err(fastn_grammar::evalexpr::error::EvalexprError::ExpectedString {
                        actual: argument.clone(),
                    })
                }
            }),
        ),
        (
            "ftd.is_empty".to_string(),
            Function::new(|argument| {
                if argument.as_empty().is_ok() {
                    Ok(Value::Boolean(true))
                } else if let Ok(s) = argument.as_string() {
                    Ok(Value::Boolean(s.is_empty()))
                } else if let Ok(s) = argument.as_tuple() {
                    Ok(Value::Boolean(s.is_empty()))
                } else {
                    Ok(Value::Boolean(false)) //todo: throw error
                }
            }),
        ),
        (
            "ftd.append".to_string(),
            Function::new(|argument| {
                if let Ok(s) = argument.as_tuple() {
                    if s.len() != 2 {
                        Err(
                            fastn_grammar::evalexpr::error::EvalexprError::WrongFunctionArgumentAmount {
                                expected: 2,
                                actual: s.len(),
                            },
                        )
                    } else {
                        let mut argument = s.first().unwrap().as_tuple()?;
                        let value = s.last().unwrap();
                        argument.push(value.to_owned());
                        Ok(Value::Tuple(argument))
                    }
                } else {
                    Ok(Value::Boolean(false)) //todo: throw error
                }
            }),
        ),
        (
            "enable_dark_mode".to_string(),
            Function::new(|_| Ok(Value::Empty)),
        ),
        (
            "enable_light_mode".to_string(),
            Function::new(|_| Ok(Value::Empty)),
        ),
        (
            "enable_system_mode".to_string(),
            Function::new(|_| Ok(Value::Empty)),
        ),
    ])
    .collect()
}

pub fn default_context() -> ftd::interpreter::Result<fastn_grammar::evalexpr::HashMapContext> {
    let mut context = fastn_grammar::evalexpr::HashMapContext::new();
    for (key, function) in default_functions() {
        context.set_function(key, function)?;
    }
    Ok(context)
}

/**
The `default_bag` function is a public function that returns a `Map` of `Thing`s.

The `Map` is a data structure that stores key-value pairs in a hash table. In this case, the keys
are `String`s representing the names of different `Thing`s, and the values are the `Thing`s
themselves.
**/
pub fn default_bag() -> indexmap::IndexMap<String, ftd::interpreter::Thing> {
    let record = |n: &str, r: &str| (n.to_string(), ftd::interpreter::Kind::record(r));
    let _color = |n: &str| record(n, "ftd#color");
    let things = vec![
        (
            "ftd#row".to_string(),
            ftd::interpreter::Thing::Component(row_function()),
        ),
        (
            "ftd#rive".to_string(),
            ftd::interpreter::Thing::Component(rive_function()),
        ),
        (
            "ftd#container".to_string(),
            ftd::interpreter::Thing::Component(container_function()),
        ),
        (
            "ftd#desktop".to_string(),
            ftd::interpreter::Thing::Component(desktop_function()),
        ),
        (
            "ftd#mobile".to_string(),
            ftd::interpreter::Thing::Component(mobile_function()),
        ),
        (
            "ftd#code".to_string(),
            ftd::interpreter::Thing::Component(code_function()),
        ),
        (
            "ftd#iframe".to_string(),
            ftd::interpreter::Thing::Component(iframe_function()),
        ),
        (
            "ftd#column".to_string(),
            ftd::interpreter::Thing::Component(column_function()),
        ),
        (
            "ftd#document".to_string(),
            ftd::interpreter::Thing::Component(document_function()),
        ),
        (
            "ftd#text".to_string(),
            ftd::interpreter::Thing::Component(markup_function()),
        ),
        (
            "ftd#integer".to_string(),
            ftd::interpreter::Thing::Component(integer_function()),
        ),
        (
            "ftd#decimal".to_string(),
            ftd::interpreter::Thing::Component(decimal_function()),
        ),
        (
            "ftd#boolean".to_string(),
            ftd::interpreter::Thing::Component(boolean_function()),
        ),
        (
            "ftd#text-input".to_string(),
            ftd::interpreter::Thing::Component(text_input_function()),
        ),
        (
            "ftd#checkbox".to_string(),
            ftd::interpreter::Thing::Component(checkbox_function()),
        ),
        (
            "ftd#image".to_string(),
            ftd::interpreter::Thing::Component(image_function()),
        ),
        (
            "ftd#video".to_string(),
            ftd::interpreter::Thing::Component(video_function()),
        ),
        (
            "ftd#set-rive-boolean".to_string(),
            ftd::interpreter::Thing::Function(ftd::interpreter::Function {
                name: "ftd#set-rive-boolean".to_string(),
                return_kind: ftd::interpreter::KindData {
                    kind: ftd::interpreter::Kind::void(),
                    caption: false,
                    body: false,
                },
                arguments: vec![
                    ftd::interpreter::Argument {
                        name: "rive".to_string(),
                        kind: ftd::interpreter::Kind::string().into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Argument {
                        name: "input".to_string(),
                        kind: ftd::interpreter::Kind::string().into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Argument {
                        name: "value".to_string(),
                        kind: ftd::interpreter::Kind::boolean().into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ],
                expression: vec![
                    ftd::interpreter::things::function::Expression {
                        expression: "ftd.set_rive_boolean(rive, input, value)".to_string(),
                        line_number: 0,
                    }
                ],
                js: None,
                line_number: 0,
                external_implementation: true,
            })
        ),
        (
            "ftd#toggle-rive-boolean".to_string(),
            ftd::interpreter::Thing::Function(ftd::interpreter::Function {
                name: "ftd#toggle-rive-boolean".to_string(),
                return_kind: ftd::interpreter::KindData {
                    kind: ftd::interpreter::Kind::void(),
                    caption: false,
                    body: false,
                },
                arguments: vec![
                    ftd::interpreter::Argument {
                        name: "rive".to_string(),
                        kind: ftd::interpreter::Kind::string().into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Argument {
                        name: "input".to_string(),
                        kind: ftd::interpreter::Kind::string().into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ],
                expression: vec![
                    ftd::interpreter::things::function::Expression {
                        expression: "ftd.toggle_rive_boolean(rive, input)".to_string(),
                        line_number: 0,
                    }
                ],
                js: None,
                line_number: 0,
                external_implementation: true
            })
        ),
        (
            "ftd#set-rive-integer".to_string(),
            ftd::interpreter::Thing::Function(ftd::interpreter::Function {
                name: "ftd#set-rive-integer".to_string(),
                return_kind: ftd::interpreter::KindData {
                    kind: ftd::interpreter::Kind::void(),
                    caption: false,
                    body: false,
                },
                arguments: vec![
                    ftd::interpreter::Argument {
                        name: "rive".to_string(),
                        kind: ftd::interpreter::Kind::string().into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Argument {
                        name: "input".to_string(),
                        kind: ftd::interpreter::Kind::string().into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Argument {
                        name: "value".to_string(),
                        kind: ftd::interpreter::Kind::integer().into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ],
                expression: vec![
                    ftd::interpreter::things::function::Expression {
                        expression: "ftd.set_rive_integer(rive, input, value)".to_string(),
                        line_number: 0,
                    }
                ],
                js: None,
                line_number: 0,
                external_implementation: true
            })
        ),
        (
            "ftd#fire-rive".to_string(),
            ftd::interpreter::Thing::Function(ftd::interpreter::Function {
                name: "ftd#fire-rive".to_string(),
                return_kind: ftd::interpreter::KindData {
                    kind: ftd::interpreter::Kind::void(),
                    caption: false,
                    body: false,
                },
                arguments: vec![
                    ftd::interpreter::Argument {
                        name: "rive".to_string(),
                        kind: ftd::interpreter::Kind::string().into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Argument {
                        name: "input".to_string(),
                        kind: ftd::interpreter::Kind::string().into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ],
                expression: vec![
                    ftd::interpreter::things::function::Expression {
                        expression: "ftd.fire_rive(rive, input)".to_string(),
                        line_number: 0,
                    }
                ],
                js: None,
                line_number: 0,
                external_implementation: true
            })
        ),
        (
            "ftd#play-rive".to_string(),
            ftd::interpreter::Thing::Function(ftd::interpreter::Function {
                name: "ftd#play-rive".to_string(),
                return_kind: ftd::interpreter::KindData {
                    kind: ftd::interpreter::Kind::void(),
                    caption: false,
                    body: false,
                },
                arguments: vec![
                    ftd::interpreter::Argument {
                        name: "rive".to_string(),
                        kind: ftd::interpreter::Kind::string().into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Argument {
                        name: "input".to_string(),
                        kind: ftd::interpreter::Kind::string().into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ],
                expression: vec![
                    ftd::interpreter::things::function::Expression {
                        expression: "ftd.play_rive(rive, input)".to_string(),
                        line_number: 0,
                    }
                ],
                js: None,
                line_number: 0,
                external_implementation: true
            })
        ),
        (
            "ftd#pause-rive".to_string(),
            ftd::interpreter::Thing::Function(ftd::interpreter::Function {
                name: "ftd#pause-rive".to_string(),
                return_kind: ftd::interpreter::KindData {
                    kind: ftd::interpreter::Kind::void(),
                    caption: false,
                    body: false,
                },
                arguments: vec![
                    ftd::interpreter::Argument {
                        name: "rive".to_string(),
                        kind: ftd::interpreter::Kind::string().into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Argument {
                        name: "input".to_string(),
                        kind: ftd::interpreter::Kind::string().into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ],
                expression: vec![
                    ftd::interpreter::things::function::Expression {
                        expression: "ftd.pause_rive(rive, input)".to_string(),
                        line_number: 0,
                    }
                ],
                js: None,
                line_number: 0,
                external_implementation: true
            })
        ),
        (
            "ftd#toggle-play-rive".to_string(),
            ftd::interpreter::Thing::Function(ftd::interpreter::Function {
                name: "ftd#toggle-play-rive".to_string(),
                return_kind: ftd::interpreter::KindData {
                    kind: ftd::interpreter::Kind::void(),
                    caption: false,
                    body: false,
                },
                arguments: vec![
                    ftd::interpreter::Argument {
                        name: "rive".to_string(),
                        kind: ftd::interpreter::Kind::string().into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Argument {
                        name: "input".to_string(),
                        kind: ftd::interpreter::Kind::string().into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ],
                expression: vec![
                    ftd::interpreter::things::function::Expression {
                        expression: "ftd.toggle_play_rive(rive, input)".to_string(),
                        line_number: 0,
                    }
                ],
                js: None,
                line_number: 0,
                external_implementation: true
            })
        ),
        (
            "ftd#toggle".to_string(),
            ftd::interpreter::Thing::Function(ftd::interpreter::Function {
                name: "ftd#toggle".to_string(),
                return_kind: ftd::interpreter::KindData {
                    kind: ftd::interpreter::Kind::void(),
                    caption: false,
                    body: false,
                },
                arguments: vec![
                    ftd::interpreter::Argument {
                        name: "a".to_string(),
                        kind: ftd::interpreter::KindData {
                            kind: ftd::interpreter::Kind::boolean(),
                            caption: false,
                            body: false,
                        },
                        mutable: true,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ],
                expression: vec![
                    ftd::interpreter::things::function::Expression {
                        expression: "a = !a".to_string(),
                        line_number: 0,
                    }
                ],
                js: None,
                line_number: 0,
                external_implementation: false
            })
        ),
        (
            "ftd#increment".to_string(),
            ftd::interpreter::Thing::Function(ftd::interpreter::Function {
                name: "ftd#increment".to_string(),
                return_kind: ftd::interpreter::KindData {
                    kind: ftd::interpreter::Kind::void(),
                    caption: false,
                    body: false,
                },
                arguments: vec![
                    ftd::interpreter::Argument {
                        name: "a".to_string(),
                        kind: ftd::interpreter::KindData {
                            kind: ftd::interpreter::Kind::integer(),
                            caption: false,
                            body: false,
                        },
                        mutable: true,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ],
                expression: vec![
                    ftd::interpreter::things::function::Expression {
                        expression: "a = a + 1".to_string(),
                        line_number: 0,
                    }
                ],
                js: None,
                line_number: 0,
                external_implementation: false
            })
        ),
        (
            "ftd#increment-by".to_string(),
            ftd::interpreter::Thing::Function(ftd::interpreter::Function {
                name: "ftd#increment-by".to_string(),
                return_kind: ftd::interpreter::KindData {
                    kind: ftd::interpreter::Kind::void(),
                    caption: false,
                    body: false,
                },
                arguments: vec![
                    ftd::interpreter::Argument {
                        name: "a".to_string(),
                        kind: ftd::interpreter::KindData {
                            kind: ftd::interpreter::Kind::integer(),
                            caption: false,
                            body: false,
                        },
                        mutable: true,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Argument {
                        name: "v".to_string(),
                        kind: ftd::interpreter::KindData {
                            kind: ftd::interpreter::Kind::integer(),
                            caption: false,
                            body: false,
                        },
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ],
                expression: vec![
                    ftd::interpreter::things::function::Expression {
                        expression: "a = a + v".to_string(),
                        line_number: 0,
                    }
                ],
                js: None,
                line_number: 0,
                external_implementation: false
            })
        ),
        (
            "ftd#decrement".to_string(),
            ftd::interpreter::Thing::Function(ftd::interpreter::Function {
                name: "ftd#decrement".to_string(),
                return_kind: ftd::interpreter::KindData {
                    kind: ftd::interpreter::Kind::void(),
                    caption: false,
                    body: false,
                },
                arguments: vec![
                    ftd::interpreter::Argument {
                        name: "a".to_string(),
                        kind: ftd::interpreter::KindData {
                            kind: ftd::interpreter::Kind::integer(),
                            caption: false,
                            body: false,
                        },
                        mutable: true,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ],
                expression: vec![
                    ftd::interpreter::things::function::Expression {
                        expression: "a = a - 1".to_string(),
                        line_number: 0,
                    }
                ],
                js: None,
                line_number: 0,
                external_implementation: false
            })
        ),
        (
            "ftd#decrement-by".to_string(),
            ftd::interpreter::Thing::Function(ftd::interpreter::Function {
                name: "ftd#decrement-by".to_string(),
                return_kind: ftd::interpreter::KindData {
                    kind: ftd::interpreter::Kind::void(),
                    caption: false,
                    body: false,
                },
                arguments: vec![
                    ftd::interpreter::Argument {
                        name: "a".to_string(),
                        kind: ftd::interpreter::KindData {
                            kind: ftd::interpreter::Kind::integer(),
                            caption: false,
                            body: false,
                        },
                        mutable: true,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Argument {
                        name: "v".to_string(),
                        kind: ftd::interpreter::KindData {
                            kind: ftd::interpreter::Kind::integer(),
                            caption: false,
                            body: false,
                        },
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ],
                expression: vec![
                    ftd::interpreter::things::function::Expression {
                        expression: "a = a - v".to_string(),
                        line_number: 0,
                    }
                ],
                js: None,
                line_number: 0,
                external_implementation: false
            })
        ),
        (
            "ftd#enable-light-mode".to_string(),
            ftd::interpreter::Thing::Function(ftd::interpreter::Function {
                name: "ftd#enable-light-mode".to_string(),
                return_kind: ftd::interpreter::KindData {
                    kind: ftd::interpreter::Kind::void(),
                    caption: false,
                    body: false,
                },
                arguments: vec![
                ],
                expression: vec![
                    ftd::interpreter::things::function::Expression {
                        expression: "enable_light_mode()".to_string(),
                        line_number: 0,
                    }
                ],
                js: None,
                line_number: 0,
                external_implementation: false
            })
        ),
        (
            "ftd#enable-dark-mode".to_string(),
            ftd::interpreter::Thing::Function(ftd::interpreter::Function {
                name: "ftd#enable-dark-mode".to_string(),
                return_kind: ftd::interpreter::KindData {
                    kind: ftd::interpreter::Kind::void(),
                    caption: false,
                    body: false,
                },
                arguments: vec![
                ],
                expression: vec![
                    ftd::interpreter::things::function::Expression {
                        expression: "enable_dark_mode()".to_string(),
                        line_number: 0,
                    }
                ],
                js: None,
                line_number: 0,
                external_implementation: false
            })
        ),
        (
            "ftd#enable-system-mode".to_string(),
            ftd::interpreter::Thing::Function(ftd::interpreter::Function {
                name: "ftd#enable-system-mode".to_string(),
                return_kind: ftd::interpreter::KindData {
                    kind: ftd::interpreter::Kind::void(),
                    caption: false,
                    body: false,
                },
                arguments: vec![
                ],
                expression: vec![
                    ftd::interpreter::things::function::Expression {
                        expression: "enable_system_mode()".to_string(),
                        line_number: 0,
                    }
                ],
                js: None,
                line_number: 0,
                external_implementation: false
            })
        ),
        (
            "ftd#clean-code".to_string(),
            ftd::interpreter::Thing::Function(ftd::interpreter::Function {
                name: "ftd#clean-code".to_string(),
                return_kind: ftd::interpreter::KindData {
                    kind: ftd::interpreter::Kind::string(),
                    caption: false,
                    body: false,
                },
                arguments: vec![
                    ftd::interpreter::Argument {
                        name: "a".to_string(),
                        kind: ftd::interpreter::KindData {
                            kind: ftd::interpreter::Kind::string(),
                            caption: false,
                            body: false,
                        },
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Argument {
                        name: "lang".to_string(),
                        kind: ftd::interpreter::KindData {
                            kind: ftd::interpreter::Kind::string(),
                            caption: false,
                            body: false,
                        },
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ],
                expression: vec![
                    ftd::interpreter::things::function::Expression {
                        expression: "ftd.clean_code(a, lang)".to_string(),
                        line_number: 0,
                    }
                ],
                js: None,
                line_number: 0,
                external_implementation: true
            })
        ),
        (
            "ftd#copy-to-clipboard".to_string(),
            ftd::interpreter::Thing::Function(ftd::interpreter::Function {
                name: "ftd#copy-to-clipboard".to_string(),
                return_kind: ftd::interpreter::KindData {
                    kind: ftd::interpreter::Kind::void(),
                    caption: false,
                    body: false,
                },
                arguments: vec![
                    ftd::interpreter::Argument {
                        name: "a".to_string(),
                        kind: ftd::interpreter::KindData {
                            kind: ftd::interpreter::Kind::string(),
                            caption: false,
                            body: false,
                        },
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ],
                expression: vec![
                    ftd::interpreter::things::function::Expression {
                        expression: "ftd.copy_to_clipboard(a)".to_string(),
                        line_number: 0,
                    }
                ],
                js: None,
                line_number: 0,
                external_implementation: true
            })
        ),
        (
            "ftd#set-bool".to_string(),
            ftd::interpreter::Thing::Function(ftd::interpreter::Function {
                name: "ftd#set-bool".to_string(),
                return_kind: ftd::interpreter::KindData {
                    kind: ftd::interpreter::Kind::void(),
                    caption: false,
                    body: false,
                },
                arguments: vec![
                    ftd::interpreter::Argument {
                        name: "a".to_string(),
                        kind: ftd::interpreter::KindData {
                            kind: ftd::interpreter::Kind::boolean(),
                            caption: false,
                            body: false,
                        },
                        mutable: true,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Argument {
                        name: "v".to_string(),
                        kind: ftd::interpreter::KindData {
                            kind: ftd::interpreter::Kind::boolean(),
                            caption: false,
                            body: false,
                        },
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ],
                expression: vec![
                    ftd::interpreter::things::function::Expression {
                        expression: "a = v".to_string(),
                        line_number: 0,
                    }
                ],
                js: None,
                line_number: 0,
                external_implementation: false
            })
        ),
        (
            "ftd#set-boolean".to_string(),
            ftd::interpreter::Thing::Function(ftd::interpreter::Function {
                name: "ftd#set-boolean".to_string(),
                return_kind: ftd::interpreter::KindData {
                    kind: ftd::interpreter::Kind::void(),
                    caption: false,
                    body: false,
                },
                arguments: vec![
                    ftd::interpreter::Argument {
                        name: "a".to_string(),
                        kind: ftd::interpreter::KindData {
                            kind: ftd::interpreter::Kind::boolean(),
                            caption: false,
                            body: false,
                        },
                        mutable: true,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Argument {
                        name: "v".to_string(),
                        kind: ftd::interpreter::KindData {
                            kind: ftd::interpreter::Kind::boolean(),
                            caption: false,
                            body: false,
                        },
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ],
                expression: vec![
                    ftd::interpreter::things::function::Expression {
                        expression: "a = v".to_string(),
                        line_number: 0,
                    }
                ],
                js: None,
                line_number: 0,
                external_implementation: false
            })
        ),
        (
            "ftd#set-string".to_string(),
            ftd::interpreter::Thing::Function(ftd::interpreter::Function {
                name: "ftd#set-string".to_string(),
                return_kind: ftd::interpreter::KindData {
                    kind: ftd::interpreter::Kind::void(),
                    caption: false,
                    body: false,
                },
                arguments: vec![
                    ftd::interpreter::Argument {
                        name: "a".to_string(),
                        kind: ftd::interpreter::KindData {
                            kind: ftd::interpreter::Kind::string(),
                            caption: false,
                            body: false,
                        },
                        mutable: true,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Argument {
                        name: "v".to_string(),
                        kind: ftd::interpreter::KindData {
                            kind: ftd::interpreter::Kind::string(),
                            caption: false,
                            body: false,
                        },
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ],
                expression: vec![
                    ftd::interpreter::things::function::Expression {
                        expression: "a = v".to_string(),
                        line_number: 0,
                    }
                ],
                js: None,
                line_number: 0,
                external_implementation: false
            })
        ),
        (
            "ftd#set-integer".to_string(),
            ftd::interpreter::Thing::Function(ftd::interpreter::Function {
                name: "ftd#set-integer".to_string(),
                return_kind: ftd::interpreter::KindData {
                    kind: ftd::interpreter::Kind::void(),
                    caption: false,
                    body: false,
                },
                arguments: vec![
                    ftd::interpreter::Argument {
                        name: "a".to_string(),
                        kind: ftd::interpreter::KindData {
                            kind: ftd::interpreter::Kind::integer(),
                            caption: false,
                            body: false,
                        },
                        mutable: true,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Argument {
                        name: "v".to_string(),
                        kind: ftd::interpreter::KindData {
                            kind: ftd::interpreter::Kind::integer(),
                            caption: false,
                            body: false,
                        },
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ],
                expression: vec![
                    ftd::interpreter::things::function::Expression {
                        expression: "a = v".to_string(),
                        line_number: 0,
                    }
                ],
                js: None,
                line_number: 0,
                external_implementation: false
            })
        ),
        (
            ftd::interpreter::FTD_IMAGE_SRC.to_string(),
            ftd::interpreter::Thing::Record(ftd::interpreter::Record {
                name: ftd::interpreter::FTD_IMAGE_SRC.to_string(),
                fields: std::iter::IntoIterator::into_iter([
                    ftd::interpreter::Field {
                        name: "light".to_string(),
                        kind: ftd::interpreter::Kind::string().into_kind_data().caption(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "dark".to_string(),
                        kind: ftd::interpreter::Kind::string().into_kind_data(),
                        mutable: false,
                        value: Some(ftd::interpreter::PropertyValue::Reference {
                            name: ftd::interpreter::FTD_IMAGE_SRC_LIGHT.to_string(),
                            kind: ftd::interpreter::Kind::string().into_kind_data(),
                            source: ftd::interpreter::PropertyValueSource::Local(
                                ftd::interpreter::FTD_IMAGE_SRC.to_string(),
                            ),
                            is_mutable: false,
                            line_number: 0,
                        }),
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ])
                .collect(),
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_VIDEO_SRC.to_string(),
            ftd::interpreter::Thing::Record(ftd::interpreter::Record {
                name: ftd::interpreter::FTD_VIDEO_SRC.to_string(),
                fields: std::iter::IntoIterator::into_iter([
                    ftd::interpreter::Field {
                        name: "light".to_string(),
                        kind: ftd::interpreter::Kind::string().into_kind_data().caption(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "dark".to_string(),
                        kind: ftd::interpreter::Kind::string().into_kind_data(),
                        mutable: false,
                        value: Some(ftd::interpreter::PropertyValue::Reference {
                            name: ftd::interpreter::FTD_VIDEO_SRC_LIGHT.to_string(),
                            kind: ftd::interpreter::Kind::string().into_kind_data(),
                            source: ftd::interpreter::PropertyValueSource::Local(
                                ftd::interpreter::FTD_VIDEO_SRC.to_string(),
                            ),
                            is_mutable: false,
                            line_number: 0,
                        }),
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ])
                .collect(),
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_RAW_IMAGE_SRC.to_string(),
            ftd::interpreter::Thing::Record(ftd::interpreter::Record {
                name: ftd::interpreter::FTD_RAW_IMAGE_SRC.to_string(),
                fields: std::iter::IntoIterator::into_iter([
                    ftd::interpreter::Field {
                        name: "src".to_string(),
                        kind: ftd::interpreter::Kind::string().into_kind_data().caption(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ])
                    .collect(),
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_COLOR.to_string(),
            ftd::interpreter::Thing::Record(ftd::interpreter::Record {
                name: ftd::interpreter::FTD_COLOR.to_string(),
                fields: std::iter::IntoIterator::into_iter([
                    ftd::interpreter::Field {
                        name: "light".to_string(),
                        kind: ftd::interpreter::Kind::string().into_kind_data().caption(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "dark".to_string(),
                        kind: ftd::interpreter::Kind::string().into_kind_data(),
                        mutable: false,
                        value: Some(ftd::interpreter::PropertyValue::Reference {
                            name: ftd::interpreter::FTD_COLOR_LIGHT.to_string(),
                            kind: ftd::interpreter::Kind::string().into_kind_data(),
                            source: ftd::interpreter::PropertyValueSource::Local(
                                ftd::interpreter::FTD_COLOR.to_string(),
                            ),
                            is_mutable: false,
                            line_number: 0,
                        }),
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ])
                .collect(),
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_SHADOW.to_string(),
            ftd::interpreter::Thing::Record(ftd::interpreter::Record {
                name: ftd::interpreter::FTD_SHADOW.to_string(),
                fields: std::iter::IntoIterator::into_iter([
                    ftd::interpreter::Field {
                        name: "x-offset".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                            .into_kind_data(),
                        mutable: false,
                        value: Some(ftd::interpreter::PropertyValue::Value {
                            value: ftd::interpreter::Value::OrType {
                                name: ftd::interpreter::FTD_LENGTH.to_string(),
                                variant: ftd::interpreter::FTD_LENGTH_PX.to_string(),
                                full_variant: ftd::interpreter::FTD_LENGTH_PX.to_string(),
                                value: Box::new
                                    (ftd::interpreter::PropertyValue::Value {
                                        value: ftd::interpreter::Value::Integer {
                                            value: 0
                                        },
                                        is_mutable: false,
                                        line_number: 0
                                    }),
                            },
                            is_mutable: false,
                            line_number: 0,
                        }),
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "y-offset".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                            .into_kind_data(),
                        mutable: false,
                        value: Some(ftd::interpreter::PropertyValue::Value {
                            value: ftd::interpreter::Value::OrType {
                                name: ftd::interpreter::FTD_LENGTH.to_string(),
                                variant: ftd::interpreter::FTD_LENGTH_PX.to_string(),
                                full_variant: ftd::interpreter::FTD_LENGTH_PX.to_string(),
                                value: Box::new
                                    (ftd::interpreter::PropertyValue::Value {
                                        value: ftd::interpreter::Value::Integer {
                                            value: 0
                                        },
                                        is_mutable: false,
                                        line_number: 0
                                    }),
                            },
                            is_mutable: false,
                            line_number: 0,
                        }),
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "blur".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                            .into_kind_data(),
                        mutable: false,
                        value: Some(ftd::interpreter::PropertyValue::Value {
                            value: ftd::interpreter::Value::OrType {
                                name: ftd::interpreter::FTD_LENGTH.to_string(),
                                variant: ftd::interpreter::FTD_LENGTH_PX.to_string(),
                                full_variant: ftd::interpreter::FTD_LENGTH_PX.to_string(),
                                value: Box::new
                                    (ftd::interpreter::PropertyValue::Value {
                                        value: ftd::interpreter::Value::Integer {
                                            value: 0
                                        },
                                        is_mutable: false,
                                        line_number: 0
                                    }),
                            },
                            is_mutable: false,
                            line_number: 0,
                        }),
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "spread".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                            .into_kind_data(),
                        mutable: false,
                        value: Some(ftd::interpreter::PropertyValue::Value {
                            value: ftd::interpreter::Value::OrType {
                                name: ftd::interpreter::FTD_LENGTH.to_string(),
                                variant: ftd::interpreter::FTD_LENGTH_PX.to_string(),
                                full_variant: ftd::interpreter::FTD_LENGTH_PX.to_string(),
                                value: Box::new
                                    (ftd::interpreter::PropertyValue::Value {
                                        value: ftd::interpreter::Value::Integer {
                                            value: 0
                                        },
                                        is_mutable: false,
                                        line_number: 0
                                    }),
                            },
                            is_mutable: false,
                            line_number: 0,
                        }),
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "color".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        access_modifier: Default::default(),
                        value: Some(ftd::interpreter::PropertyValue::Value {
                            value: ftd::interpreter::Value::Record {
                                name: ftd::interpreter::FTD_COLOR.to_string(),
                                fields: std::iter::IntoIterator::into_iter([
                                    (
                                        "light".to_string(),
                                        ftd::interpreter::PropertyValue::Value {
                                            value: ftd::interpreter::Value::String { text: "black".to_string() },
                                            is_mutable: false,
                                            line_number: 0,
                                        }
                                    ),
                                    (
                                        "dark".to_string(),
                                        ftd::interpreter::PropertyValue::Value {
                                            value: ftd::interpreter::Value::String { text: "white".to_string() },
                                            is_mutable: false,
                                            line_number: 0,
                                        }
                                    ),
                                ]).collect()
                            },
                            is_mutable: false,
                            line_number: 0,
                        }),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "inset".to_string(),
                        kind: ftd::interpreter::Kind::boolean()
                            .into_kind_data(),
                        mutable: false,
                        access_modifier: Default::default(),
                        value: Some(ftd::interpreter::PropertyValue::Value {
                            value: ftd::interpreter::Value::Boolean { value: false },
                            is_mutable: false,
                            line_number: 0,
                        }),
                        line_number: 0,
                    },
                ]).collect(),
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_BACKDROP_FILTER.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_BACKDROP_FILTER.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKDROP_FILTER_BLUR,
                        ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                            .into_kind_data(),
                        false,
                        None,
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKDROP_FILTER_BRIGHTNESS,
                        ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                            .into_kind_data(),
                        false,
                        None,
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKDROP_FILTER_CONTRAST,
                        ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                            .into_kind_data(),
                        false,
                        None,
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKDROP_FILTER_GRAYSCALE,
                        ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                            .into_kind_data(),
                        false,
                        None,
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKDROP_FILTER_INVERT,
                        ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                            .into_kind_data(),
                        false,
                        None,
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKDROP_FILTER_OPACITY,
                        ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                            .into_kind_data(),
                        false,
                        None,
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKDROP_FILTER_SEPIA,
                        ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                            .into_kind_data(),
                        false,
                        None,
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKDROP_FILTER_SATURATE,
                        ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                            .into_kind_data(),
                        false,
                        None,
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKDROP_FILTER_MULTI,
                        ftd::interpreter::Kind::record(ftd::interpreter::FTD_BACKDROP_MULTI)
                            .into_kind_data(),
                        false,
                        None,
                        0,
                    )),
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_BACKDROP_MULTI.to_string(),
            ftd::interpreter::Thing::Record(ftd::interpreter::Record {
                name: ftd::interpreter::FTD_BACKDROP_MULTI.to_string(),
                fields: std::iter::IntoIterator::into_iter([
                    ftd::interpreter::Field {
                        name: "blur".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                            .into_kind_data().into_optional(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "brightness".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                            .into_kind_data().into_optional(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "contrast".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                            .into_kind_data().into_optional(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "grayscale".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                            .into_kind_data().into_optional(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "invert".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                            .into_kind_data().into_optional(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "opacity".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                            .into_kind_data().into_optional(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "sepia".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                            .into_kind_data().into_optional(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "saturate".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                            .into_kind_data().into_optional(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ]).collect(),
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_LENGTH_PAIR.to_string(),
            ftd::interpreter::Thing::Record(ftd::interpreter::Record {
                name: ftd::interpreter::FTD_LENGTH_PAIR.to_string(),
                fields: std::iter::IntoIterator::into_iter([
                    ftd::interpreter::Field {
                        name: "x".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "y".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ]).collect(),
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_BG_IMAGE.to_string(),
            ftd::interpreter::Thing::Record(ftd::interpreter::Record {
                name: ftd::interpreter::FTD_BG_IMAGE.to_string(),
                fields: std::iter::IntoIterator::into_iter([
                    ftd::interpreter::Field {
                        name: "src".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_IMAGE_SRC)
                            .into_kind_data().caption(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "repeat".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_BACKGROUND_REPEAT)
                            .into_kind_data().into_optional(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "size".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_BACKGROUND_SIZE)
                            .into_kind_data().into_optional(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "position".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_BACKGROUND_POSITION)
                            .into_kind_data().into_optional(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ]).collect(),
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_LINEAR_GRADIENT_COLOR.to_string(),
            ftd::interpreter::Thing::Record(ftd::interpreter::Record {
                name: ftd::interpreter::FTD_LINEAR_GRADIENT_COLOR.to_string(),
                fields: std::iter::IntoIterator::into_iter([
                    ftd::interpreter::Field {
                        name: "color".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data().caption(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "start".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                            .into_kind_data().into_optional(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "end".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                            .into_kind_data().into_optional(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "stop-position".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                            .into_kind_data().into_optional(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ]).collect(),
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_LINEAR_GRADIENT_DIRECTIONS.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_LINEAR_GRADIENT_DIRECTIONS.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_LINEAR_GRADIENT_DIRECTIONS_ANGLE,
                        ftd::interpreter::Kind::decimal()
                            .into_kind_data(),
                        false,
                        None,
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_LINEAR_GRADIENT_DIRECTIONS_TURN,
                        ftd::interpreter::Kind::decimal()
                            .into_kind_data(),
                        false,
                        None,
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_LINEAR_GRADIENT_DIRECTIONS_LEFT,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("to left")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_LINEAR_GRADIENT_DIRECTIONS_RIGHT,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("to right")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_LINEAR_GRADIENT_DIRECTIONS_TOP,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("to top")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_LINEAR_GRADIENT_DIRECTIONS_BOTTOM,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("to bottom")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_LINEAR_GRADIENT_DIRECTIONS_TOP_LEFT,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("to top left")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_LINEAR_GRADIENT_DIRECTIONS_BOTTOM_LEFT,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("to bottom left")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_LINEAR_GRADIENT_DIRECTIONS_TOP_RIGHT,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("to top right")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_LINEAR_GRADIENT_DIRECTIONS_BOTTOM_RIGHT,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("to bottom right")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_LINEAR_GRADIENT.to_string(),
            ftd::interpreter::Thing::Record(ftd::interpreter::Record {
                name: ftd::interpreter::FTD_LINEAR_GRADIENT.to_string(),
                fields: std::iter::IntoIterator::into_iter([
                    ftd::interpreter::Field {
                        name: "direction".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LINEAR_GRADIENT_DIRECTIONS)
                            .into_kind_data().into_optional(),
                        mutable: false,
                        value: Some(ftd::interpreter::PropertyValue::Value {
                            value: ftd::interpreter::Value::OrType {
                                name: ftd::interpreter::FTD_LINEAR_GRADIENT_DIRECTIONS.to_string(),
                                variant: ftd::interpreter::FTD_LINEAR_GRADIENT_DIRECTIONS_BOTTOM
                                    .to_string(),
                                full_variant: ftd::interpreter::FTD_LINEAR_GRADIENT_DIRECTIONS_BOTTOM.to_string(),
                                value: Box::new
                                    (ftd::interpreter::PropertyValue::Value {
                                        value: ftd::interpreter::Value::String {
                                            text: "bottom".to_string(),
                                        },
                                        is_mutable: false,
                                        line_number: 0
                                    }),
                            },
                            is_mutable: false,
                            line_number: 0,
                        }),
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "colors".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_LINEAR_GRADIENT_COLOR)
                            .into_list().into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ]).collect(),
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_BACKGROUND.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_BACKGROUND.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::Regular(
                    ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKGROUND_SOLID,
                        ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        false,
                        None,
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKGROUND_IMAGE,
                        ftd::interpreter::Kind::record(ftd::interpreter::FTD_BG_IMAGE)
                            .into_kind_data(),
                        false,
                        None,
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKGROUND_LINEAR_GRADIENT,
                        ftd::interpreter::Kind::record(ftd::interpreter::FTD_LINEAR_GRADIENT)
                            .into_kind_data(),
                        false,
                        None,
                        0,
                    )),
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_BACKGROUND_REPEAT.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_BACKGROUND_REPEAT.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKGROUND_REPEAT_BOTH_REPEAT,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("repeat")
                                 .into_property_value(false, 0),),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKGROUND_REPEAT_X_REPEAT,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("repeat-x")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKGROUND_REPEAT_Y_REPEAT,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("repeat-y")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKGROUND_REPEAT_NO_REPEAT,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("no-repeat")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKGROUND_REPEAT_SPACE,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("space")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKGROUND_REPEAT_ROUND,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("round")
                            .into_property_value(false, 0)),
                        0,
                    )),
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_BACKGROUND_SIZE.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_BACKGROUND_SIZE.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKGROUND_SIZE_AUTO,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("auto")
                                 .into_property_value(false, 0),),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKGROUND_SIZE_COVER,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("cover")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKGROUND_SIZE_CONTAIN,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("contain")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::AnonymousRecord(ftd::interpreter::Record {
                        name: ftd::interpreter::FTD_BACKGROUND_SIZE_LENGTH.to_string(),
                        fields: std::iter::IntoIterator::into_iter([
                            ftd::interpreter::Field {
                                name: "x".to_string(),
                                kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                                    .into_kind_data(),
                                mutable: false,
                                value: None,
                                access_modifier: Default::default(),
                                line_number: 0,
                            },
                            ftd::interpreter::Field {
                                name: "y".to_string(),
                                kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                                    .into_kind_data(),
                                mutable: false,
                                value: None,
                                access_modifier: Default::default(),
                                line_number: 0,
                            },
                        ]).collect(),
                        line_number: 0,
                    }),
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_BACKGROUND_POSITION.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_BACKGROUND_POSITION.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKGROUND_POSITION_LEFT,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("left")
                                 .into_property_value(false, 0),),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKGROUND_POSITION_CENTER,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("center")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKGROUND_POSITION_RIGHT,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("right")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKGROUND_POSITION_LEFT_TOP,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("left-top")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKGROUND_POSITION_LEFT_CENTER,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("left-center")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKGROUND_POSITION_LEFT_BOTTOM,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("left-bottom")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKGROUND_POSITION_CENTER_TOP,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("center-top")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKGROUND_POSITION_CENTER_CENTER,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("center-center")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKGROUND_POSITION_CENTER_BOTTOM,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("center-bottom")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKGROUND_POSITION_RIGHT_TOP,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("right-top")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKGROUND_POSITION_RIGHT_CENTER,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("right-center")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BACKGROUND_POSITION_RIGHT_BOTTOM,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("right-bottom")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::AnonymousRecord(ftd::interpreter::Record {
                        name: ftd::interpreter::FTD_BACKGROUND_POSITION_LENGTH.to_string(),
                        fields: std::iter::IntoIterator::into_iter([
                            ftd::interpreter::Field {
                                name: "x".to_string(),
                                kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                                    .into_kind_data(),
                                mutable: false,
                                value: None,
                                access_modifier: Default::default(),
                                line_number: 0,
                            },
                            ftd::interpreter::Field {
                                name: "y".to_string(),
                                kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                                    .into_kind_data(),
                                mutable: false,
                                value: None,
                                access_modifier: Default::default(),
                                line_number: 0,
                            },
                        ]).collect(),
                        line_number: 0,
                    }),
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_ALIGN.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_ALIGN.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_ALIGN_TOP_LEFT,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string(
                                ftd::interpreter::FTD_ALIGN_TOP_LEFT,
                            )
                            .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_ALIGN_TOP_CENTER,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string(
                                ftd::interpreter::FTD_ALIGN_TOP_CENTER,
                            )
                            .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_ALIGN_TOP_RIGHT,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string(
                                ftd::interpreter::FTD_ALIGN_TOP_RIGHT,
                            )
                            .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_ALIGN_LEFT,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string(ftd::interpreter::FTD_ALIGN_LEFT)
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_ALIGN_CENTER,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string(
                                ftd::interpreter::FTD_ALIGN_CENTER,
                            )
                            .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_ALIGN_RIGHT,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string(
                                ftd::interpreter::FTD_ALIGN_RIGHT,
                            )
                            .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_ALIGN_BOTTOM_LEFT,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string(
                                ftd::interpreter::FTD_ALIGN_BOTTOM_LEFT,
                            )
                            .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_ALIGN_BOTTOM_CENTER,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string(
                                ftd::interpreter::FTD_ALIGN_BOTTOM_CENTER,
                            )
                            .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_ALIGN_BOTTOM_RIGHT,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string(
                                ftd::interpreter::FTD_ALIGN_BOTTOM_RIGHT,
                            )
                            .into_property_value(false, 0),
                        ),
                        0,
                    )),
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_SPACING.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_SPACING.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_SPACING_FIXED,
                        ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                            .into_kind_data(),
                        false,
                        None,
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_SPACING_SPACE_BETWEEN,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("space-between")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_SPACING_SPACE_EVENLY,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("space-evenly")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_SPACING_SPACE_AROUND,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("space-around")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_IMAGE_FIT.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_IMAGE_FIT.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_IMAGE_FIT_NONE,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("none")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_IMAGE_FIT_COVER,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("cover")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_IMAGE_FIT_CONTAIN,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("contain")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_IMAGE_FIT_FILL,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("fill")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_IMAGE_FIT_SCALE_DOWN,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("scale-down")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),

                ],
                line_number: 0,
            }),
        ),
        (
             ftd::interpreter::FTD_IMAGE_FETCH_PRIORITY.to_string(),
             ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                 name: ftd::interpreter::FTD_IMAGE_FETCH_PRIORITY.to_string(),
                 variants: vec![
                     ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                         ftd::interpreter::FTD_IMAGE_FETCH_PRIORITY_AUTO,
                         ftd::interpreter::Kind::string().into_kind_data(),
                         false,
                         Some(
                              ftd::interpreter::Value::new_string("auto")
                                  .into_property_value(false, 0),
                         ),
                                0,
                         )),
                     ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                         ftd::interpreter::FTD_IMAGE_FETCH_PRIORITY_LOW,
                         ftd::interpreter::Kind::string().into_kind_data(),
                         false,
                         Some(
                              ftd::interpreter::Value::new_string("low")
                                  .into_property_value(false, 0),
                         ),
                                0,
                         )),
                     ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                         ftd::interpreter::FTD_IMAGE_FETCH_PRIORITY_HIGH,
                         ftd::interpreter::Kind::string().into_kind_data(),
                         false,
                         Some(
                              ftd::interpreter::Value::new_string("high")
                                  .into_property_value(false, 0),
                         ),
                                0,
                            )),
                        ],
                        line_number: 0,
                    }),
        ),
        (
            ftd::interpreter::FTD_ANCHOR.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_ANCHOR.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_ANCHOR_ID,
                        ftd::interpreter::Kind::string()
                            .into_kind_data(),
                        false,
                        None,
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_ANCHOR_PARENT,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("absolute")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_ANCHOR_WINDOW,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("fixed")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_OVERFLOW.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_OVERFLOW.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_OVERFLOW_SCROLL,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("scroll")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_OVERFLOW_VISIBLE,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("visible")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_OVERFLOW_HIDDEN,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("hidden")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_OVERFLOW_AUTO,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("auto")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_RESIZE.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_RESIZE.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_RESIZE_HORIZONTAL,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("horizontal")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_RESIZE_VERTICAL,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("vertical")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_RESIZE_BOTH,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("both")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_CURSOR.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_CURSOR.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_DEFAULT,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("default")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_NONE,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("none")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_CONTEXT_MENU,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("context-menu")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_HELP,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("help")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_POINTER,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("pointer")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_PROGRESS,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("progress")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_WAIT,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("wait")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_CELL,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("cell")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_CROSSHAIR,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("crosshair")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_TEXT,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("text")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_VERTICAL_TEXT,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("vertical-text")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_ALIAS,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("alias")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_COPY,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("copy")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_MOVE,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("move")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_NO_DROP,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("no-drop")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_NOT_ALLOWED,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("not-allowed")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_GRAB,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("grab")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_GRABBING,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("grabbing")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_E_RESIZE,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("e-resize")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_N_RESIZE,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("n-resize")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_NE_RESIZE,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("ne-resize")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_NW_RESIZE,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("nw-resize")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_S_RESIZE,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("s-resize")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_SE_RESIZE,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("se-resize")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_SW_RESIZE,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("sw-resize")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_W_RESIZE,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("w-resize")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_EW_RESIZE,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("ew-resize")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_NS_RESIZE,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("ns-resize")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_NESW_RESIZE,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("nesw-resize")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_NWSE_RESIZE,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("nwse-resize")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_COL_RESIZE,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("col-resize")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_ROW_RESIZE,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("row-resize")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_ALL_SCROLL,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("all-scroll")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_ZOOM_IN,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("zoom-in")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_CURSOR_ZOOM_OUT,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("zoom-out")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_ALIGN_SELF.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_ALIGN_SELF.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_ALIGN_SELF_START,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("start")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_ALIGN_SELF_CENTER,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("center")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_ALIGN_SELF_END,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("end")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_TEXT_ALIGN.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_TEXT_ALIGN.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_ALIGN_START,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("start")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_ALIGN_CENTER,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("center")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_ALIGN_END,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("end")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_ALIGN_JUSTIFY,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("justify")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_LINK_REL.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_LINK_REL.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_LINK_REL_NO_FOLLOW,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("no-follow")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_LINK_REL_SPONSORED,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("sponsored")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_LINK_REL_UGC,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("ugc")
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_RESIZING.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_RESIZING.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_RESIZING_HUG_CONTENT,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string(
                                ftd::interpreter::FTD_RESIZING_HUG_CONTENT,
                            )
                            .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_RESIZING_AUTO,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string(
                                ftd::interpreter::FTD_RESIZING_AUTO,
                            )
                                .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_RESIZING_FILL_CONTAINER,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string(
                                ftd::interpreter::FTD_RESIZING_FILL_CONTAINER,
                            )
                            .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_RESIZING_FIXED,
                        ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                            .into_kind_data(),
                        false,
                        None,
                        0,
                    )),
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_WHITESPACE.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_WHITESPACE.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_WHITESPACE_NORMAL,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("normal")
                                  .into_property_value(false, 0),),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_WHITESPACE_NOWRAP,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("nowrap")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_WHITESPACE_PRE,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("pre")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_WHITESPACE_PREWRAP,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("pre-wrap")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_WHITESPACE_PRELINE,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("pre-line")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_WHITESPACE_BREAKSPACES,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("break-spaces")
                            .into_property_value(false, 0)),
                        0,
                    )),
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_DISPLAY.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_DISPLAY.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_DISPLAY_BLOCK,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("block")
                                 .into_property_value(false, 0),),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_DISPLAY_INLINE,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("inline")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_DISPLAY_INLINE_BLOCK,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("inline-block")
                            .into_property_value(false, 0)),
                        0,
                    )),
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_LENGTH.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_LENGTH.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_LENGTH_PX,
                        ftd::interpreter::Kind::integer()
                            .into_kind_data()
                            .caption(),
                        false,
                        None,
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_LENGTH_PERCENT,
                        ftd::interpreter::Kind::decimal()
                            .into_kind_data()
                            .caption(),
                        false,
                        None,
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_LENGTH_CALC,
                        ftd::interpreter::Kind::string().into_kind_data().caption(),
                        false,
                        None,
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_LENGTH_VH,
                        ftd::interpreter::Kind::decimal()
                            .into_kind_data()
                            .caption(),
                        false,
                        None,
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_LENGTH_VW,
                        ftd::interpreter::Kind::decimal()
                            .into_kind_data()
                            .caption(),
                        false,
                        None,
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_LENGTH_VMIN,
                        ftd::interpreter::Kind::decimal()
                            .into_kind_data()
                            .caption(),
                        false,
                        None,
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_LENGTH_VMAX,
                        ftd::interpreter::Kind::decimal()
                            .into_kind_data()
                            .caption(),
                        false,
                        None,
                        0,
                    )),

                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_LENGTH_DVH,
                        ftd::interpreter::Kind::decimal()
                            .into_kind_data()
                            .caption(),
                        false,
                        None,
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_LENGTH_LVH,
                        ftd::interpreter::Kind::decimal()
                            .into_kind_data()
                            .caption(),
                        false,
                        None,
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_LENGTH_SVH,
                        ftd::interpreter::Kind::decimal()
                            .into_kind_data()
                            .caption(),
                        false,
                        None,
                        0,
                    )),

                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_LENGTH_EM,
                        ftd::interpreter::Kind::decimal()
                            .into_kind_data()
                            .caption(),
                        false,
                        None,
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_LENGTH_REM,
                        ftd::interpreter::Kind::decimal()
                            .into_kind_data()
                            .caption(),
                        false,
                        None,
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_LENGTH_RESPONSIVE,
                        ftd::interpreter::Kind::record(ftd::interpreter::FTD_RESPONSIVE_LENGTH)
                            .into_kind_data()
                            .caption(),
                        false,
                        None,
                        0,
                    )),
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_RESPONSIVE_LENGTH.to_string(),
            ftd::interpreter::Thing::Record(ftd::interpreter::Record {
                name: ftd::interpreter::FTD_RESPONSIVE_LENGTH.to_string(),
                fields: std::iter::IntoIterator::into_iter([
                    ftd::interpreter::Field {
                        name: "desktop".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                            .into_kind_data()
                            .caption(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "mobile".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                            .into_kind_data(),
                        mutable: false,
                        access_modifier: Default::default(),
                        value: Some(ftd::interpreter::PropertyValue::Reference {
                            name: ftd::interpreter::FTD_RESPONSIVE_LENGTH_DESKTOP.to_string(),
                            kind: ftd::interpreter::Kind::string().into_kind_data(),
                            source: ftd::interpreter::PropertyValueSource::Local(
                                ftd::interpreter::FTD_RESPONSIVE_LENGTH.to_string(),
                            ),
                            is_mutable: false,
                            line_number: 0,
                        }),
                        line_number: 0,
                    },
                ])
                .collect(),
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_FONT_SIZE.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_FONT_SIZE_PX,
                        ftd::interpreter::Kind::integer()
                            .into_kind_data()
                            .caption(),
                        false,
                        None,
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_FONT_SIZE_EM,
                        ftd::interpreter::Kind::decimal()
                            .into_kind_data()
                            .caption(),
                        false,
                        None,
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_FONT_SIZE_REM,
                        ftd::interpreter::Kind::decimal()
                            .into_kind_data()
                            .caption(),
                        false,
                        None,
                        0,
                    )),
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_REGION.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_REGION.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_REGION_H1,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(   ftd::interpreter::Value::new_string("h1")
                                    .into_property_value(false, 0),),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_REGION_H2,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("h2")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_REGION_H3,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("h3")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_REGION_H4,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("h4")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_REGION_H5,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("h5")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_REGION_H6,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("h6")
                            .into_property_value(false, 0)),
                        0,
                    )),
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_TEXT_INPUT_TYPE.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_TEXT_INPUT_TYPE.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_INPUT_TYPE_TEXT,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(   ftd::interpreter::Value::new_string("text")
                                    .into_property_value(false, 0),),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_INPUT_TYPE_EMAIL,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("email")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_INPUT_TYPE_PASSWORD,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("password")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_INPUT_TYPE_URL,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("url")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_INPUT_TYPE_DATETIME,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("datetime-local")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_INPUT_TYPE_DATE,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("date")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_INPUT_TYPE_TIME,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("time")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_INPUT_TYPE_MONTH,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("month")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_INPUT_TYPE_WEEK,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("week")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_INPUT_TYPE_COLOR,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("color")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_INPUT_TYPE_FILE,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("file")
                            .into_property_value(false, 0)),
                        0,
                    )),
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_LOADING.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_LOADING.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_LOADING_EAGER,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(   ftd::interpreter::Value::new_string("eager")
                                    .into_property_value(false, 0),),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_LOADING_LAZY,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("lazy")
                            .into_property_value(false, 0)),
                        0,
                    )),
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_BORDER_STYLE.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_BORDER_STYLE.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BORDER_STYLE_DASHED,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(   ftd::interpreter::Value::new_string("dashed")
                                    .into_property_value(false, 0),),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BORDER_STYLE_DOTTED,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(   ftd::interpreter::Value::new_string("dotted")
                                    .into_property_value(false, 0),),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BORDER_STYLE_DOUBLE,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(   ftd::interpreter::Value::new_string("double")
                                    .into_property_value(false, 0),),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BORDER_STYLE_GROOVE,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(   ftd::interpreter::Value::new_string("groove")
                                    .into_property_value(false, 0),),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BORDER_STYLE_INSET,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(   ftd::interpreter::Value::new_string("inset")
                                    .into_property_value(false, 0),),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BORDER_STYLE_OUTSET,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(   ftd::interpreter::Value::new_string("outset")
                                    .into_property_value(false, 0),),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BORDER_STYLE_RIDGE,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(   ftd::interpreter::Value::new_string("ridge")
                                    .into_property_value(false, 0),),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_BORDER_STYLE_SOLID,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(   ftd::interpreter::Value::new_string("solid")
                                    .into_property_value(false, 0),),
                        0,
                    )),
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_TEXT_STYLE.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_TEXT_STYLE.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_STYLE_UNDERLINE,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("underline").into_property_value(false, 0),),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_STYLE_STRIKE,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("strike").into_property_value(false, 0),),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_STYLE_ITALIC,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("italic").into_property_value(false, 0),),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_STYLE_WEIGHT_HEAVY,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("heavy").into_property_value(false, 0),),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_STYLE_WEIGHT_EXTRA_BOLD,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("extra-bold").into_property_value(false, 0),),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_STYLE_WEIGHT_BOLD,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("bold").into_property_value(false, 0),),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_STYLE_WEIGHT_SEMI_BOLD,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("semi-bold").into_property_value(false, 0),),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_STYLE_WEIGHT_MEDIUM,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("medium").into_property_value(false, 0),),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_STYLE_WEIGHT_REGULAR,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("regular").into_property_value(false, 0),),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_STYLE_WEIGHT_LIGHT,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("light").into_property_value(false, 0),),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_STYLE_WEIGHT_EXTRA_LIGHT,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("extra-light").into_property_value(false, 0),),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_STYLE_WEIGHT_HAIRLINE,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("hairline").into_property_value(false, 0),),
                        0,
                    )),
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_TEXT_TRANSFORM.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_TEXT_TRANSFORM.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_TRANSFORM_NONE,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(   ftd::interpreter::Value::new_string("none")
                                    .into_property_value(false, 0),),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_TRANSFORM_CAPITALIZE,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("capitalize")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_TRANSFORM_UPPERCASE,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("uppercase")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_TRANSFORM_LOWERCASE,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("lowercase")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_TRANSFORM_INITIAL,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("initial")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_TEXT_TRANSFORM_INHERIT,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("inherit")
                            .into_property_value(false, 0)),
                        0,
                    )),
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_TYPE.to_string(),
            ftd::interpreter::Thing::Record(ftd::interpreter::Record {
                name: ftd::interpreter::FTD_TYPE.to_string(),
                fields: std::iter::IntoIterator::into_iter([
                    ftd::interpreter::Field {
                        name: "size".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_FONT_SIZE)
                            .into_optional()
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "line-height".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_FONT_SIZE)
                            .into_optional()
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "letter-spacing".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_FONT_SIZE)
                            .into_optional()
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "weight".to_string(),
                        kind: ftd::interpreter::Kind::integer()
                            .into_optional()
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "font-family".to_string(),
                        kind: ftd::interpreter::Kind::string()
                            .into_list()
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ])
                .collect(),
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_RESPONSIVE_TYPE.to_string(),
            ftd::interpreter::Thing::Record(ftd::interpreter::Record {
                name: ftd::interpreter::FTD_RESPONSIVE_TYPE.to_string(),
                fields: std::iter::IntoIterator::into_iter([
                    ftd::interpreter::Field {
                        name: "desktop".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_TYPE)
                            .into_kind_data().caption(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "mobile".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_TYPE)
                            .into_kind_data(),
                        mutable: false,
                        access_modifier: Default::default(),
                        value: Some(ftd::interpreter::PropertyValue::Reference {
                            name: ftd::interpreter::FTD_RESPONSIVE_TYPE_DESKTOP.to_string(),
                            kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_TYPE)
                                .into_kind_data(),
                            source: ftd::interpreter::PropertyValueSource::Local(
                                ftd::interpreter::FTD_RESPONSIVE_TYPE.to_string(),
                            ),
                            is_mutable: false,
                            line_number: 0,
                        }),
                        line_number: 0,
                    },
                ])
                .collect(),
                line_number: 0,
            }),
        ),
        (
            "ftd#dark-mode".to_string(),
            ftd::interpreter::Thing::Variable(ftd::interpreter::Variable {
                name: "ftd#dark-mode".to_string(),
                kind: ftd::interpreter::Kind::boolean().into_kind_data(),
                mutable: true,
                value: ftd::interpreter::PropertyValue::Value {
                    value: ftd::interpreter::Value::Boolean { value: false },
                    is_mutable: true,
                    line_number: 0,
                },
                conditional_value: vec![],
                line_number: 0,
                is_static: false,
            }),
        ),
        (
            "ftd#empty".to_string(),
            ftd::interpreter::Thing::Variable(ftd::interpreter::Variable {
                name: "ftd#empty".to_string(),
                kind: ftd::interpreter::Kind::string().into_kind_data(),
                mutable: false,
                value: ftd::interpreter::PropertyValue::Value {
                    value: ftd::interpreter::Value::String { text: "".to_string() },
                    is_mutable: false,
                    line_number: 0,
                },
                conditional_value: vec![],
                line_number: 0,
                is_static: false,
            }),
        ),
        (
            "ftd#space".to_string(),
            ftd::interpreter::Thing::Variable(ftd::interpreter::Variable {
                name: "ftd#space".to_string(),
                kind: ftd::interpreter::Kind::string().into_kind_data(),
                mutable: false,
                value: ftd::interpreter::PropertyValue::Value {
                    value: ftd::interpreter::Value::String { text: " ".to_string() },
                    is_mutable: false,
                    line_number: 0,
                },
                conditional_value: vec![],
                line_number: 0,
                is_static: false,
            }),
        ),
        (
            "ftd#nbsp".to_string(),
            ftd::interpreter::Thing::Variable(ftd::interpreter::Variable {
                name: "ftd#nbsp".to_string(),
                kind: ftd::interpreter::Kind::string().into_kind_data(),
                mutable: false,
                value: ftd::interpreter::PropertyValue::Value {
                    value: ftd::interpreter::Value::String { text: "&nbsp;".to_string() },
                    is_mutable: false,
                    line_number: 0,
                },
                conditional_value: vec![],
                line_number: 0,
                is_static: false,
            }),
        ),
        (
            "ftd#non-breaking-space".to_string(),
            ftd::interpreter::Thing::Variable(ftd::interpreter::Variable {
                name: "ftd#non-breaking-space".to_string(),
                kind: ftd::interpreter::Kind::string().into_kind_data(),
                mutable: false,
                value: ftd::interpreter::PropertyValue::Value {
                    value: ftd::interpreter::Value::String { text: "&nbsp;".to_string() },
                    is_mutable: false,
                    line_number: 0,
                },
                conditional_value: vec![],
                line_number: 0,
                is_static: false,
            }),
        ),
        (
            "ftd#system-dark-mode".to_string(),
            ftd::interpreter::Thing::Variable(ftd::interpreter::Variable {
                name: "ftd#system-dark-mode".to_string(),
                kind: ftd::interpreter::Kind::boolean().into_kind_data(),
                mutable: true,
                value: ftd::interpreter::PropertyValue::Value {
                    value: ftd::interpreter::Value::Boolean { value: false },
                    is_mutable: true,
                    line_number: 0,
                },
                conditional_value: vec![],
                line_number: 0,
                is_static: false,
            }),
        ),
        (
            "ftd#follow-system-dark-mode".to_string(),
            ftd::interpreter::Thing::Variable(ftd::interpreter::Variable {
                name: "ftd#follow-system-dark-mode".to_string(),
                kind: ftd::interpreter::Kind::boolean().into_kind_data(),
                mutable: true,
                value: ftd::interpreter::PropertyValue::Value {
                    value: ftd::interpreter::Value::Boolean { value: true },
                    is_mutable: true,
                    line_number: 0,
                },
                conditional_value: vec![],
                line_number: 0,
                is_static: false,
            }),
        ),
        (
            "ftd#redirect".to_string(),
            ftd::interpreter::Thing::Component(ftd::interpreter::ComponentDefinition {
                name: "ftd#redirect".to_string(),
                arguments: vec![
                    ftd::interpreter::Argument::default(
                    "url",
                    ftd::interpreter::Kind::string()
                        .into_kind_data().caption_or_body(),
                    ),
                    ftd::interpreter::Argument::default_with_value(
                        "code",
                        ftd::interpreter::Kind::integer()
                            .into_kind_data(),
                        ftd::interpreter::PropertyValue::Value {
                            value: ftd::interpreter::Value::Integer { value: 308 },
                            is_mutable: false,
                            line_number: 0,
                        }
                    ),
                ],
                definition: ftd::interpreter::Component::from_name("ftd.kernel"),
                css: None,
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_BACKGROUND_COLOR.to_string(),
            ftd::interpreter::Thing::Record(ftd::interpreter::Record {
                name: ftd::interpreter::FTD_BACKGROUND_COLOR.to_string(),
                fields: vec![
                    ftd::interpreter::Field {
                        name: "base".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "step-1".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "step-2".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "overlay".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "code".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_CTA_COLOR.to_string(),
            ftd::interpreter::Thing::Record(ftd::interpreter::Record {
                name: ftd::interpreter::FTD_CTA_COLOR.to_string(),
                fields: vec![
                    ftd::interpreter::Field {
                        name: "base".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "hover".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "pressed".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "disabled".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "focused".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "border".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "border-disabled".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "text".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "text-disabled".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_PST.to_string(),
            ftd::interpreter::Thing::Record(ftd::interpreter::Record {
                name: ftd::interpreter::FTD_PST.to_string(),
                fields: vec![
                    ftd::interpreter::Field {
                        name: "primary".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "secondary".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "tertiary".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_BTB.to_string(),
            ftd::interpreter::Thing::Record(ftd::interpreter::Record {
                name: ftd::interpreter::FTD_BTB.to_string(),
                fields: vec![
                    ftd::interpreter::Field {
                        name: "base".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "text".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "border".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_CUSTOM_COLORS.to_string(),
            ftd::interpreter::Thing::Record(ftd::interpreter::Record {
                name: ftd::interpreter::FTD_CUSTOM_COLORS.to_string(),
                fields: vec![
                    ftd::interpreter::Field {
                        name: "one".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "two".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "three".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "four".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "five".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "six".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "seven".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "eight".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "nine".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "ten".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_COLOR_SCHEME.to_string(),
            ftd::interpreter::Thing::Record(ftd::interpreter::Record {
                name: ftd::interpreter::FTD_COLOR_SCHEME.to_string(),
                fields: vec![
                    ftd::interpreter::Field {
                        name: "background".to_string(),
                        kind: ftd::interpreter::Kind::record("ftd#background-colors")
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "border".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "border-strong".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "text".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "text-strong".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        line_number: 0,
                        access_modifier: Default::default(),
                    },
                    ftd::interpreter::Field {
                        name: "shadow".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        line_number: 0,
                        access_modifier: Default::default(),
                    },
                    ftd::interpreter::Field {
                        name: "scrim".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        line_number: 0,
                        access_modifier: Default::default(),
                    },
                    ftd::interpreter::Field {
                        name: "cta-primary".to_string(),
                        kind: ftd::interpreter::Kind::record("ftd#cta-colors").into_kind_data(),
                        mutable: false,
                        value: None,
                        line_number: 0,
                        access_modifier: Default::default(),
                    },
                    ftd::interpreter::Field {
                        name: "cta-secondary".to_string(),
                        kind: ftd::interpreter::Kind::record("ftd#cta-colors").into_kind_data(),
                        mutable: false,
                        value: None,
                        line_number: 0,
                        access_modifier: Default::default(),
                    },
                    ftd::interpreter::Field {
                        name: "cta-tertiary".to_string(),
                        kind: ftd::interpreter::Kind::record("ftd#cta-colors").into_kind_data(),
                        mutable: false,
                        value: None,
                        line_number: 0,
                        access_modifier: Default::default(),
                    },
                    ftd::interpreter::Field {
                        name: "cta-danger".to_string(),
                        kind: ftd::interpreter::Kind::record("ftd#cta-colors").into_kind_data(),
                        mutable: false,
                        value: None,
                        line_number: 0,
                        access_modifier: Default::default(),
                    },
                    ftd::interpreter::Field {
                        name: "accent".to_string(),
                        kind: ftd::interpreter::Kind::record("ftd#pst").into_kind_data(),
                        mutable: false,
                        value: None,
                        line_number: 0,
                        access_modifier: Default::default(),
                    },
                    ftd::interpreter::Field {
                        name: "error".to_string(),
                        kind: ftd::interpreter::Kind::record("ftd#btb").into_kind_data(),
                        mutable: false,
                        value: None,
                        line_number: 0,
                        access_modifier: Default::default(),
                    },
                    ftd::interpreter::Field {
                        name: "success".to_string(),
                        kind: ftd::interpreter::Kind::record("ftd#btb").into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "info".to_string(),
                        kind: ftd::interpreter::Kind::record("ftd#btb").into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "warning".to_string(),
                        kind: ftd::interpreter::Kind::record("ftd#btb").into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "custom".to_string(),
                        kind: ftd::interpreter::Kind::record("ftd#custom-colors").into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_TYPE_DATA.to_string(),
            ftd::interpreter::Thing::Record(ftd::interpreter::Record {
                name: ftd::interpreter::FTD_TYPE_DATA.to_string(),
                fields: vec![ftd::interpreter::Field {
                    name: "heading-large".to_string(),
                    kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_RESPONSIVE_TYPE)
                        .into_kind_data(),
                    mutable: false,
                    value: None,
                    access_modifier: Default::default(),
                    line_number: 0
                }, ftd::interpreter::Field {
                    name: "heading-medium".to_string(),
                    kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_RESPONSIVE_TYPE)
                        .into_kind_data(),
                    mutable: false,
                    value: None,
                    access_modifier: Default::default(),
                    line_number: 0
                }, ftd::interpreter::Field {
                    name: "heading-small".to_string(),
                    kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_RESPONSIVE_TYPE)
                        .into_kind_data(),
                    mutable: false,
                    value: None,
                    access_modifier: Default::default(),
                    line_number: 0
                },ftd::interpreter::Field {
                    name: "heading-hero".to_string(),
                    kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_RESPONSIVE_TYPE)
                        .into_kind_data(),
                    mutable: false,
                    value: None,
                    access_modifier: Default::default(),
                    line_number: 0
                },ftd::interpreter::Field {
                    name: "heading-tiny".to_string(),
                    kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_RESPONSIVE_TYPE)
                        .into_kind_data(),
                    mutable: false,
                    value: None,
                    access_modifier: Default::default(),
                    line_number: 0
                },ftd::interpreter::Field {
                    name: "copy-small".to_string(),
                    kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_RESPONSIVE_TYPE)
                        .into_kind_data(),
                    mutable: false,
                    value: None,
                    access_modifier: Default::default(),
                    line_number: 0
                },ftd::interpreter::Field {
                    name: "copy-regular".to_string(),
                    kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_RESPONSIVE_TYPE)
                        .into_kind_data(),
                    mutable: false,
                    value: None,
                    access_modifier: Default::default(),
                    line_number: 0
                },ftd::interpreter::Field {
                    name: "copy-large".to_string(),
                    kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_RESPONSIVE_TYPE)
                        .into_kind_data(),
                    mutable: false,
                    value: None,
                    access_modifier: Default::default(),
                    line_number: 0
                },ftd::interpreter::Field {
                    name: "fine-print".to_string(),
                    kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_RESPONSIVE_TYPE)
                        .into_kind_data(),
                    mutable: false,
                    value: None,
                    access_modifier: Default::default(),
                    line_number: 0
                },ftd::interpreter::Field {
                    name: "blockquote".to_string(),
                    kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_RESPONSIVE_TYPE)
                        .into_kind_data(),
                    mutable: false,
                    value: None,
                    access_modifier: Default::default(),
                    line_number: 0
                },ftd::interpreter::Field {
                    name: "source-code".to_string(),
                    kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_RESPONSIVE_TYPE)
                        .into_kind_data(),
                    mutable: false,
                    value: None,
                    access_modifier: Default::default(),
                    line_number: 0
                },ftd::interpreter::Field {
                    name: "button-small".to_string(),
                    kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_RESPONSIVE_TYPE)
                        .into_kind_data(),
                    mutable: false,
                    value: None,
                    access_modifier: Default::default(),
                    line_number: 0
                },ftd::interpreter::Field {
                    name: "button-medium".to_string(),
                    kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_RESPONSIVE_TYPE)
                        .into_kind_data(),
                    mutable: false,
                    value: None,
                    access_modifier: Default::default(),
                    line_number: 0,
                },ftd::interpreter::Field {
                    name: "button-large".to_string(),
                    kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_RESPONSIVE_TYPE)
                        .into_kind_data(),
                    mutable: false,
                    value: None,
                    access_modifier: Default::default(),
                    line_number: 0,
                },ftd::interpreter::Field {
                    name: "link".to_string(),
                    kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_RESPONSIVE_TYPE)
                        .into_kind_data(),
                    mutable: false,
                    value: None,
                    access_modifier: Default::default(),
                    line_number: 0,
                },ftd::interpreter::Field {
                    name: "label-large".to_string(),
                    kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_RESPONSIVE_TYPE)
                        .into_kind_data(),
                    mutable: false,
                    value: None,
                    access_modifier: Default::default(),
                    line_number: 0
                },ftd::interpreter::Field {
                    name: "label-small".to_string(),
                    kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_RESPONSIVE_TYPE)
                        .into_kind_data(),
                    mutable: false,
                    value: None,
                    access_modifier: Default::default(),
                    line_number: 0
                },],
                line_number: 0
            })
        ),
        (
            "ftd#font-display".to_string(),
             ftd::interpreter::Thing::Variable(ftd::interpreter::Variable {
                 name: "ftd#font-display".to_string(),
                 kind: ftd::interpreter::Kind::string().into_kind_data(),
                 mutable: true,
                 value: ftd::interpreter::PropertyValue::Value {
                     value: ftd::interpreter::Value::new_string("sans-serif"),
                     is_mutable: true,
                     line_number: 0
                 },
                 conditional_value: vec![],
                 line_number: 0,
                 is_static: false
             })
        ),
        (
            "ftd#font-copy".to_string(),
            ftd::interpreter::Thing::Variable(ftd::interpreter::Variable {
                name: "ftd#font-copy".to_string(),
                kind: ftd::interpreter::Kind::string().into_kind_data(),
                mutable: true,
                value: ftd::interpreter::PropertyValue::Value {
                    value: ftd::interpreter::Value::new_string("sans-serif"),
                    is_mutable: true,
                    line_number: 0
                },
                conditional_value: vec![],
                line_number: 0,
                is_static: false
            })
        ),
        (
            "ftd#font-code".to_string(),
            ftd::interpreter::Thing::Variable(ftd::interpreter::Variable {
                name: "ftd#font-code".to_string(),
                kind: ftd::interpreter::Kind::string().into_kind_data(),
                mutable: true,
                value: ftd::interpreter::PropertyValue::Value {
                    value: ftd::interpreter::Value::new_string("sans-serif"),
                    is_mutable: true,
                    line_number: 0
                },
                conditional_value: vec![],
                line_number: 0,
                is_static: false
            })
        ),
        (
            "ftd#default-types".to_string(),
            ftd::interpreter::Thing::Variable(ftd::interpreter::Variable {
                name: "ftd#default-types".to_string(),
                kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_TYPE_DATA).into_kind_data(),
                mutable: true,
                value: ftd::interpreter::PropertyValue::Value {
                    value: ftd::interpreter::Value::Record {
                        name: ftd::interpreter::FTD_TYPE_DATA.to_string(),
                        fields: std::iter::IntoIterator::into_iter([
                            // HEADING TYPES -------------------------------------------
                            (
                                "heading-hero".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_RESPONSIVE_TYPE.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([
                                            (
                                                "desktop".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-display".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 80
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 104
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ), (
                                                "mobile".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-display".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 48
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 64
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            )
                                        ]).collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            (
                                "heading-large".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_RESPONSIVE_TYPE.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([
                                            (
                                                "desktop".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-display".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 50
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 65
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ), (
                                                "mobile".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-display".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 36
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 54
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            )
                                        ]).collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            (
                                "heading-medium".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_RESPONSIVE_TYPE.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([
                                            (
                                                "desktop".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-display".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 38
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 57
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ), (
                                                "mobile".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-display".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 26
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 40
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            )
                                        ]).collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            (
                                "heading-small".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_RESPONSIVE_TYPE.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([
                                            (
                                                "desktop".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-display".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 24
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 31
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ), (
                                                "mobile".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-display".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 22
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 29
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            )
                                        ]).collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            (
                                "heading-tiny".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_RESPONSIVE_TYPE.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([
                                            (
                                                "desktop".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-display".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 20
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 26
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ), (
                                                "mobile".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-display".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 18
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 24
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            )
                                        ]).collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            // COPY TYPES -------------------------------------------
                            (
                                "copy-large".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_RESPONSIVE_TYPE.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([
                                            (
                                                "desktop".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-copy".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 22
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 34
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ), (
                                                "mobile".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-copy".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 18
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 28
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            )
                                        ]).collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            (
                                "copy-regular".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_RESPONSIVE_TYPE.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([
                                            (
                                                "desktop".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-copy".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 18
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 30
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ), (
                                                "mobile".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-copy".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 16
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 24
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            )
                                        ]).collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            (
                                "copy-small".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_RESPONSIVE_TYPE.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([
                                            (
                                                "desktop".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-copy".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 14
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 24
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ), (
                                                "mobile".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-copy".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 12
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 16
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            )
                                        ]).collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            // SPECIALIZED TEXT TYPES ---------------------------------
                            (
                                "fine-print".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_RESPONSIVE_TYPE.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([
                                            (
                                                "desktop".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-code".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 12
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 16
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ), (
                                                "mobile".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-code".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 12
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 16
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            )
                                        ]).collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            (
                                "blockquote".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_RESPONSIVE_TYPE.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([
                                            (
                                                "desktop".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-code".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 16
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 21
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ), (
                                                "mobile".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-code".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 16
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 21
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            )
                                        ]).collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            (
                                "source-code".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_RESPONSIVE_TYPE.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([
                                            (
                                                "desktop".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-code".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 18
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 30
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ), (
                                                "mobile".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-code".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 16
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 21
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            )
                                        ]).collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            // LABEL TYPES -------------------------------------
                            (
                                "label-large".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_RESPONSIVE_TYPE.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([
                                            (
                                                "desktop".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-display".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 14
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 19
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ), (
                                                "mobile".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-display".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 14
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 19
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            )
                                        ]).collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            (
                                "label-small".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_RESPONSIVE_TYPE.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([
                                            (
                                                "desktop".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-display".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 12
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 16
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ), (
                                                "mobile".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-display".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 12
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 16
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            )
                                        ]).collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            // BUTTON TYPES -------------------------------------
                            (
                                "button-large".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_RESPONSIVE_TYPE.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([
                                            (
                                                "desktop".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-display".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 18
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 24
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ), (
                                                "mobile".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-display".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 18
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 24
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            )
                                        ]).collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            (
                                "button-medium".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_RESPONSIVE_TYPE.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([
                                            (
                                                "desktop".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-display".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 16
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 21
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ), (
                                                "mobile".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-display".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 16
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 21
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            )
                                        ]).collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            (
                                "button-small".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_RESPONSIVE_TYPE.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([
                                            (
                                                "desktop".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-display".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 14
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 19
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ), (
                                                "mobile".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-display".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 14
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 19
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            )
                                        ]).collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            (
                                "link".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_RESPONSIVE_TYPE.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([
                                            (
                                                "desktop".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-display".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 14
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 19
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ), (
                                                "mobile".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_TYPE.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "font-family".to_string(),
                                                                ftd::interpreter::PropertyValue::Reference {
                                                                    name: "ftd#font-display".to_string(),
                                                                    kind:
                                                                    ftd::interpreter::Kind::string().into_kind_data(),
                                                                    source:
                                                                    ftd::interpreter::PropertyValueSource::Global,
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "size".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 14
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "line-height".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::OrType {
                                                                        name: ftd::interpreter::FTD_FONT_SIZE.to_string(),
                                                                        variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        full_variant: ftd::interpreter::FTD_FONT_SIZE_PX.to_string(),
                                                                        value: Box::new
                                                                            (ftd::interpreter::PropertyValue::Value {
                                                                                value: ftd::interpreter::Value::Integer {
                                                                                    value: 19
                                                                                },
                                                                                is_mutable: false,
                                                                                line_number: 0
                                                                            })
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                            (
                                                                "weight".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value:
                                                                    ftd::interpreter::Value::Integer {
                                                                        value: 400
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ),
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            )
                                        ]).collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                        ]).collect()
                    },
                    is_mutable: false,
                    line_number: 0
                },
                conditional_value: vec![],
                line_number: 0,
                is_static: false
            })
        ),
        (
            "ftd#default-colors".to_string(),
            ftd::interpreter::Thing::Variable(ftd::interpreter::Variable {
                name: "ftd#default-colors".to_string(),
                kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR_SCHEME)
                    .into_kind_data(),
                mutable: true,
                value: ftd::interpreter::PropertyValue::Value {
                    value: ftd::interpreter::Value::Record {
                        name: ftd::interpreter::FTD_COLOR_SCHEME.to_string(),
                        fields: std::iter::IntoIterator::into_iter([
                            (
                                "background".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_BACKGROUND_COLOR.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([
                                            (
                                                "base".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value:
                                                                ftd::interpreter::Value::String {
                                                                    text: "#e7e7e4".to_string(),
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            }
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value:
                                                                ftd::interpreter::Value::String {
                                                                    text: "#18181b".to_string(),
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            }
                                                        )])
                                                            .collect(),
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                }
                                            ),
                                            (
                                                "step-1".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value:
                                                                ftd::interpreter::Value::String {
                                                                    text: "#f3f3f3".to_string(),
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            }
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value:
                                                                ftd::interpreter::Value::String {
                                                                    text: "#141414".to_string(),
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            }
                                                        )])
                                                            .collect(),
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                }
                                            ),
                                            (
                                                "step-2".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value:
                                                                ftd::interpreter::Value::String {
                                                                    text: "#c9cece".to_string(),
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            }
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value:
                                                                ftd::interpreter::Value::String {
                                                                    text: "#585656".to_string(),
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            }
                                                        )])
                                                            .collect(),
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                }
                                            ),
                                            (
                                                "overlay".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value:
                                                                ftd::interpreter::Value::String {
                                                                    text: "rgba(0, 0, 0, 0.8)"
                                                                        .to_string(),
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            }
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value:
                                                                ftd::interpreter::Value::String {
                                                                    text: "rgba(0, 0, 0, 0.8)"
                                                                        .to_string(),
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            }
                                                        )])
                                                            .collect(),
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                }
                                            ),
                                            (
                                                "code".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value:
                                                                ftd::interpreter::Value::String {
                                                                    text: "#F5F5F5".to_string(),
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            }
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value:
                                                                ftd::interpreter::Value::String {
                                                                    text: "#21222C".to_string(),
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            }
                                                        )])
                                                            .collect(),
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                }
                                            ),
                                        ])
                                            .collect(),
                                    },
                                    is_mutable: false,
                                    line_number: 0,
                                }
                            ),
                            (
                                "border".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([(
                                            "light".to_string(),
                                            ftd::interpreter::PropertyValue::Value {
                                                value:
                                                ftd::interpreter::Value::String {
                                                    text: "#434547".to_string(),
                                                },
                                                is_mutable: false,
                                                line_number: 0,
                                            }
                                        ), (
                                            "dark".to_string(),
                                            ftd::interpreter::PropertyValue::Value {
                                                value:
                                                ftd::interpreter::Value::String {
                                                    text: "#434547".to_string(),
                                                },
                                                is_mutable: false,
                                                line_number: 0,
                                            }
                                        )])
                                            .collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            (
                                "border-strong".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([(
                                            "light".to_string(),
                                            ftd::interpreter::PropertyValue::Value {
                                                value:
                                                ftd::interpreter::Value::String {
                                                    text: "#919192".to_string(),
                                                },
                                                is_mutable: false,
                                                line_number: 0,
                                            }
                                        ), (
                                            "dark".to_string(),
                                            ftd::interpreter::PropertyValue::Value {
                                                value:
                                                ftd::interpreter::Value::String {
                                                    text: "#919192".to_string(),
                                                },
                                                is_mutable: false,
                                                line_number: 0,
                                            }
                                        )])
                                            .collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            (
                                "text".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([(
                                            "light".to_string(),
                                            ftd::interpreter::PropertyValue::Value {
                                                value:
                                                ftd::interpreter::Value::String {
                                                    text: "#584b42".to_string(),
                                                },
                                                is_mutable: false,
                                                line_number: 0,
                                            }
                                        ), (
                                            "dark".to_string(),
                                            ftd::interpreter::PropertyValue::Value {
                                                value:
                                                ftd::interpreter::Value::String {
                                                    text: "#a8a29e".to_string(),
                                                },
                                                is_mutable: false,
                                                line_number: 0,
                                            }
                                        )])
                                            .collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            (
                                "text-strong".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([(
                                            "light".to_string(),
                                            ftd::interpreter::PropertyValue::Value {
                                                value:
                                                ftd::interpreter::Value::String {
                                                    text: "#141414".to_string(),
                                                },
                                                is_mutable: false,
                                                line_number: 0,
                                            }
                                        ), (
                                            "dark".to_string(),
                                            ftd::interpreter::PropertyValue::Value {
                                                value:
                                                ftd::interpreter::Value::String {
                                                    text: "#ffffff".to_string(),
                                                },
                                                is_mutable: false,
                                                line_number: 0,
                                            }
                                        )])
                                            .collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            (
                                "shadow".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([(
                                            "light".to_string().to_string(),
                                            ftd::interpreter::PropertyValue::Value {
                                                value:
                                                ftd::interpreter::Value::String {
                                                    text: "#007f9b".to_string(),
                                                },
                                                is_mutable: false,
                                                line_number: 0,
                                            },
                                        ), (
                                            "dark".to_string(),
                                            ftd::interpreter::PropertyValue::Value {
                                                value:
                                                ftd::interpreter::Value::String {
                                                    text: "#007f9b".to_string(),
                                                },
                                                is_mutable: false,
                                                line_number: 0,
                                            },
                                        )])
                                            .collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            (
                                "scrim".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([(
                                            "light".to_string(),
                                            ftd::interpreter::PropertyValue::Value {
                                                value:
                                                ftd::interpreter::Value::String {
                                                    text: "#007f9b".to_string(),
                                                },
                                                is_mutable: false,
                                                line_number: 0,
                                            },
                                        ), (
                                            "dark".to_string(),
                                            ftd::interpreter::PropertyValue::Value {
                                                value:
                                                ftd::interpreter::Value::String {
                                                    text: "#007f9b".to_string(),
                                                },
                                                is_mutable: false,
                                                line_number: 0,
                                            },
                                        )])
                                            .collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            (
                                "cta-primary".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_CTA_COLOR.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([
                                            (
                                                "base".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#2dd4bf".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#2dd4bf".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "hover".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#2c9f90".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#2c9f90".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "pressed".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#2cc9b5".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#2cc9b5".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "disabled".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "rgba(44, 201, 181, 0.1)".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "rgba(44, 201, 181, 0.1)".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "focused".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#2cbfac".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#2cbfac".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "border".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#2b8074".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#2b8074".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "text".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#feffff".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#feffff".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "border-disabled".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#65b693".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string().to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#65b693".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "text-disabled".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#65b693".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#65b693".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            )
                                        ]).collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            (
                                "cta-secondary".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_CTA_COLOR.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([
                                            (
                                                "base".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#4fb2df".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#4fb2df".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "hover".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#40afe1".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#40afe1".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "pressed".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#4fb2df".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#4fb2df".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "disabled".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "rgba(79, 178, 223, 0.1)".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "rgba(79, 178, 223, 0.1)".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "focused".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#4fb1df".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#4fb1df".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "border".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#209fdb".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#209fdb".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "text".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#584b42".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#ffffff".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "border-disabled".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#65b693".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#65b693".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "text-disabled".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#65b693".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#65b693".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            )
                                        ]).collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            (
                                "cta-tertiary".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_CTA_COLOR.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([
                                            (
                                                "base".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#556375".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#556375".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "hover".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#c7cbd1".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#c7cbd1".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "pressed".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#3b4047".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#3b4047".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "disabled".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "rgba(85, 99, 117, 0.1)".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "rgba(85, 99, 117, 0.1)".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "focused".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#e0e2e6".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#e0e2e6".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "border".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#e2e4e7".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#e2e4e7".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "text".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#ffffff".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#ffffff".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "border-disabled".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#65b693".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#65b693".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "text-disabled".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#65b693".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#65b693".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            )
                                        ]).collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            (
                                "cta-danger".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_CTA_COLOR.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([
                                            (
                                                "base".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#1C1B1F".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#1C1B1F".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "hover".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#1C1B1F".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#1C1B1F".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "pressed".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#1C1B1F".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#1C1B1F".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "disabled".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#1C1B1F".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#1C1B1F".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "focused".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#1C1B1F".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#1C1B1F".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "border".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#1C1B1F".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#1C1B1F".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "text".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "light".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#1C1B1F".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0,
                                                                }
                                                            ), (
                                                                "dark".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#1C1B1F".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0,
                                                                }
                                                            )
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "border-disabled".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#feffff".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#feffff".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            ),
                                            (
                                                "text-disabled".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([(
                                                            "light".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#feffff".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        ), (
                                                            "dark".to_string(),
                                                            ftd::interpreter::PropertyValue::Value {
                                                                value: ftd::interpreter::Value::String {
                                                                    text: "#feffff".to_string()
                                                                },
                                                                is_mutable: false,
                                                                line_number: 0,
                                                            },
                                                        )]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0,
                                                },
                                            )
                                        ]).collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            (
                                "accent".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_PST.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([
                                            (
                                                "primary".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "light".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#2dd4bf".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ), (
                                                                "dark".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#2dd4bf".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            )
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ),
                                            (
                                                "secondary".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "light".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#4fb2df".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ), (
                                                                "dark".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#4fb2df".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            )
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ),
                                            (
                                                "tertiary".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "light".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#c5cbd7".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ), (
                                                                "dark".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#c5cbd7".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            )
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            )
                                        ]).collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            (
                                "error".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_BTB.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([
                                            (
                                                "base".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "light".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#f5bdbb".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ), (
                                                                "dark".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#311b1f".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            )
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ),
                                            (
                                                "text".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "light".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#c62a21".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ), (
                                                                "dark".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#c62a21".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            )
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ),
                                            (
                                                "border".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "light".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#df2b2b".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ), (
                                                                "dark".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#df2b2b".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            )
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            )
                                        ]).collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            (
                                "success".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_BTB.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([
                                            (
                                                "base".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "light".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#e3f0c4".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ), (
                                                                "dark".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#405508ad".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            )
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ),
                                            (
                                                "text".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "light".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#467b28".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ), (
                                                                "dark".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#479f16".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            )
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ),
                                            (
                                                "border".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "light".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#3d741f".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ), (
                                                                "dark".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#3d741f".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            )
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            )
                                        ]).collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            (
                                "info".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_BTB.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([
                                            (
                                                "base".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "light".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#c4edfd".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ), (
                                                                "dark".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#15223a".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            )
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ),
                                            (
                                                "text".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "light".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#205694".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ), (
                                                                "dark".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#1f6feb".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            )
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ),
                                            (
                                                "border".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "light".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#205694".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ), (
                                                                "dark".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#205694".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            )
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ),
                                        ]).collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            (
                                "warning".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_BTB.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([
                                            (
                                                "base".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "light".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#fbefba".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ), (
                                                                "dark".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#544607a3".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            )
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ),
                                            (
                                                "text".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "light".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#966220".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ), (
                                                                "dark".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#d07f19".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            )
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ),
                                            (
                                                "border".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "light".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#966220".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ), (
                                                                "dark".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#966220".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            )
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ),
                                        ]).collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                            (
                                "custom".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Record {
                                        name: ftd::interpreter::FTD_CUSTOM_COLORS.to_string(),
                                        fields: std::iter::IntoIterator::into_iter([
                                            (
                                                "one".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "light".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#ed753a".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ), (
                                                                "dark".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#ed753a".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            )
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ),
                                            (
                                                "two".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "light".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#f3db5f".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ), (
                                                                "dark".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#f3db5f".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            )
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ),
                                            (
                                                "three".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "light".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#8fdcf8".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ), (
                                                                "dark".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#8fdcf8".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            )
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ),
                                            (
                                                "four".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "light".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#7a65c7".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ), (
                                                                "dark".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#7a65c7".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            )
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ),
                                            (
                                                "five".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "light".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#eb57be".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ), (
                                                                "dark".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#eb57be".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            )
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ),
                                            (
                                                "six".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "light".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#ef8dd6".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ), (
                                                                "dark".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#ef8dd6".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            )
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ),
                                            (
                                                "seven".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "light".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#7564be".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ), (
                                                                "dark".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#7564be".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            )
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ),
                                            (
                                                "eight".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "light".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#d554b3".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ), (
                                                                "dark".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#d554b3".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            )
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ),
                                            (
                                                "nine".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "light".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#ec8943".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ), (
                                                                "dark".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#ec8943".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            )
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ),
                                            (
                                                "ten".to_string(),
                                                ftd::interpreter::PropertyValue::Value {
                                                    value: ftd::interpreter::Value::Record {
                                                        name: ftd::interpreter::FTD_COLOR.to_string(),
                                                        fields: std::iter::IntoIterator::into_iter([
                                                            (
                                                                "light".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#da7a4a".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            ), (
                                                                "dark".to_string(),
                                                                ftd::interpreter::PropertyValue::Value {
                                                                    value: ftd::interpreter::Value::String {
                                                                        text: "#da7a4a".to_string()
                                                                    },
                                                                    is_mutable: false,
                                                                    line_number: 0
                                                                }
                                                            )
                                                        ]).collect()
                                                    },
                                                    is_mutable: false,
                                                    line_number: 0
                                                }
                                            ),
                                        ]).collect()
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            ),
                        ])
                            .collect(),
                    },
                    is_mutable: false,
                    line_number: 0,
                },
                conditional_value: vec![],
                line_number: 0,
                is_static: false,
            }),
        ),
        (
            ftd::interpreter::FTD_BREAKPOINT_WIDTH_DATA.to_string(),
            ftd::interpreter::Thing::Record(ftd::interpreter::Record {
                name: ftd::interpreter::FTD_BREAKPOINT_WIDTH_DATA.to_string(),
                fields: vec![ftd::interpreter::Field {
                    name: "mobile".to_string(),
                    kind: ftd::interpreter::Kind::integer().into_kind_data().caption(),
                    mutable: false,
                    value: None,
                    access_modifier: Default::default(),
                    line_number: 0
                }],
                line_number: 0
            })
        ),
        (
            ftd::interpreter::FTD_BREAKPOINT_WIDTH.to_string(),
            ftd::interpreter::Thing::Variable(ftd::interpreter::Variable {
                name: ftd::interpreter::FTD_BREAKPOINT_WIDTH.to_string(),
                kind: ftd::interpreter::Kind::record
                    (ftd::interpreter::FTD_BREAKPOINT_WIDTH_DATA).into_kind_data(),
                mutable: true,
                value: ftd::interpreter::PropertyValue::Value {
                    value: ftd::interpreter::Value::Record {
                        name: ftd::interpreter::FTD_BREAKPOINT_WIDTH_DATA.to_string(),
                        fields: std::iter::IntoIterator::into_iter([
                            (
                                "mobile".to_string(),
                                ftd::interpreter::PropertyValue::Value {
                                    value: ftd::interpreter::Value::Integer {
                                        value: 768
                                    },
                                    is_mutable: false,
                                    line_number: 0
                                }
                            )
                        ]).collect()
                    },
                    is_mutable: true,
                    line_number: 0
                },
                conditional_value: vec![],
                line_number: 0,
                is_static: false
            })
        ),
        (
            ftd::interpreter::FTD_DEVICE_DATA.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_DEVICE_DATA.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_DEVICE_DATA_MOBILE,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("mobile")
                                    .into_property_value(false, 0),),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_DEVICE_DATA_DESKTOP,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("desktop")
                                 .into_property_value(false, 0),),
                        0,
                    )),
                ],
                line_number: 0
            })
        ),
        (
            ftd::interpreter::FTD_DEVICE.to_string(),
            ftd::interpreter::Thing::Variable(ftd::interpreter::Variable {
                name: ftd::interpreter::FTD_DEVICE.to_string(),
                kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_DEVICE_DATA)
                    .into_kind_data(),
                mutable: true,
                value: ftd::interpreter::PropertyValue::Value {
                    value: ftd::interpreter::Value::OrType {
                        name: ftd::interpreter::FTD_DEVICE_DATA.to_string(),
                        variant: ftd::interpreter::FTD_DEVICE_DATA_MOBILE.to_string(),
                        full_variant: ftd::interpreter::FTD_DEVICE_DATA_MOBILE.to_string(),
                        value: Box::new(ftd::interpreter::Value::new_string("mobile")
                            .into_property_value(false, 0))
                    },
                    is_mutable: true,
                    line_number: 0
                },
                conditional_value: vec![],
                line_number: 0,
                is_static: false
            })
        ),
        (
            ftd::interpreter::FTD_MASK_IMAGE_DATA.to_string(),
            ftd::interpreter::Thing::Record(ftd::interpreter::Record {
                name: ftd::interpreter::FTD_MASK_IMAGE_DATA.to_string(),
                fields: std::iter::IntoIterator::into_iter([
                    ftd::interpreter::Field {
                        name: "src".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_IMAGE_SRC)
                            .into_kind_data().caption().into_optional(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "linear-gradient".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_LINEAR_GRADIENT)
                            .into_kind_data()
                            .into_optional(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "color".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                            .into_kind_data()
                            .into_optional(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ]).collect(),
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_MASK_SIZE.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_MASK_SIZE.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_MASK_SIZE_FIXED,
                        ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                            .into_kind_data(),
                        false,
                        None,
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_MASK_SIZE_AUTO,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string(
                                ftd::interpreter::FTD_MASK_SIZE_AUTO,
                            )
                            .into_property_value(false, 0),
                        ),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_MASK_SIZE_COVER,
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string(
                                ftd::interpreter::FTD_MASK_SIZE_CONTAIN,
                            )
                            .into_property_value(false, 0),
                        ),
                        0,
                    )),
                ],
                line_number: 0,
            }),
        ),

        (
            ftd::interpreter::FTD_MASK_REPEAT.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_MASK_REPEAT.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_MASK_REPEAT_BOTH_REPEAT,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("repeat")
                                 .into_property_value(false, 0),),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_MASK_REPEAT_X_REPEAT,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("repeat-x")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_MASK_REPEAT_Y_REPEAT,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("repeat-y")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_MASK_REPEAT_NO_REPEAT,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("no-repeat")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_MASK_REPEAT_SPACE,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("space")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_MASK_REPEAT_ROUND,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("round")
                            .into_property_value(false, 0)),
                        0,
                    )),
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_MASK_POSITION.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_MASK_POSITION.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_MASK_POSITION_LEFT,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("left")
                                 .into_property_value(false, 0),),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_MASK_POSITION_CENTER,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("center")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_MASK_POSITION_RIGHT,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("right")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_MASK_POSITION_LEFT_TOP,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("left-top")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_MASK_POSITION_LEFT_CENTER,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("left-center")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_MASK_POSITION_LEFT_BOTTOM,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("left-bottom")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_MASK_POSITION_CENTER_TOP,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("center-top")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_MASK_POSITION_CENTER_CENTER,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("center-center")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_MASK_POSITION_CENTER_BOTTOM,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("center-bottom")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_MASK_POSITION_RIGHT_TOP,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("right-top")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_MASK_POSITION_RIGHT_CENTER,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("right-center")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Constant(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_MASK_POSITION_RIGHT_BOTTOM,
                        ftd::interpreter::Kind::string()
                            .into_kind_data()
                            .caption(),
                        false,
                        Some(ftd::interpreter::Value::new_string("right-bottom")
                            .into_property_value(false, 0)),
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::AnonymousRecord(ftd::interpreter::Record {
                        name: ftd::interpreter::FTD_MASK_POSITION_LENGTH.to_string(),
                        fields: std::iter::IntoIterator::into_iter([
                            ftd::interpreter::Field {
                                name: "x".to_string(),
                                kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                                    .into_kind_data(),
                                mutable: false,
                                value: None,
                                access_modifier: Default::default(),
                                line_number: 0,
                            },
                            ftd::interpreter::Field {
                                name: "y".to_string(),
                                kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                                    .into_kind_data(),
                                mutable: false,
                                value: None,
                                access_modifier: Default::default(),
                                line_number: 0,
                            },
                        ]).collect(),
                        line_number: 0,
                    }),
                ],
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_MASK_MULTI_DATA.to_string(),
            ftd::interpreter::Thing::Record(ftd::interpreter::Record {
                name: ftd::interpreter::FTD_MASK_MULTI_DATA.to_string(),
                fields: std::iter::IntoIterator::into_iter([
                    ftd::interpreter::Field {
                        name: "image".to_string(),
                        kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_MASK_IMAGE_DATA)
                            .into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "size".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_MASK_SIZE)
                            .into_kind_data()
                            .into_optional(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "size-x".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_MASK_SIZE)
                            .into_kind_data()
                            .into_optional(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "size-y".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_MASK_SIZE)
                            .into_kind_data()
                            .into_optional(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "repeat".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_MASK_REPEAT)
                            .into_kind_data()
                            .into_optional(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "position".to_string(),
                        kind: ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_MASK_POSITION)
                            .into_kind_data()
                            .into_optional(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ]).collect(),
                line_number: 0,
            }),
        ),
        (
            ftd::interpreter::FTD_MASK.to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: ftd::interpreter::FTD_MASK.to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_MASK_IMAGE,
                        ftd::interpreter::Kind::record(ftd::interpreter::FTD_MASK_IMAGE_DATA)
                            .into_kind_data(),
                        false,
                        None,
                        0,
                    )),
                    ftd::interpreter::OrTypeVariant::Regular(ftd::interpreter::Field::new(
                        ftd::interpreter::FTD_MASK_MULTI,
                        ftd::interpreter::Kind::record(ftd::interpreter::FTD_MASK_MULTI_DATA)
                            .into_kind_data(),
                        false,
                        None,
                        0,
                    )),
                ],
                line_number: 0,
            }),
        ),
        (
            "ftd#http-options".to_string(),
            ftd::interpreter::Thing::Record(ftd::interpreter::Record {
                name: "ftd#http-options".to_string(),
                fields: std::iter::IntoIterator::into_iter([
                    ftd::interpreter::Field {
                        name: "method".to_string(),
                        kind: ftd::interpreter::Kind::or_type("ftd#http-method")
                            .into_kind_data(),
                        mutable: false,
                        value: Some(
                            ftd::interpreter::PropertyValue::Value {
                                value: ftd::interpreter::Value::OrType {
                                    name: "ftd#http-method".to_string(),
                                    variant: "ftd#http-method.GET".to_string(),
                                    full_variant: "ftd#http-method.GET".to_string(),
                                    value: Box::new(ftd::interpreter::Value::new_string("GET")
                                        .into_property_value(false, 0))
                                }

                                , is_mutable: false, line_number: 0 }
                        ),
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "redirect".to_string(),
                        kind: ftd::interpreter::Kind::or_type("ftd#http-redirect").into_kind_data(),
                        mutable: false,
                        value: Some(
                            ftd::interpreter::PropertyValue::Value {
                                value: ftd::interpreter::Value::OrType {
                                    name: "ftd#http-redirect".to_string(),
                                    variant: "ftd#http-redirect.manual".to_string(),
                                    full_variant: "ftd#http-redirect.manual".to_string(),
                                    value: Box::new(ftd::interpreter::Value::new_string("follow")
                                        .into_property_value(false, 0))
                                }

                                , is_mutable: false, line_number: 0 }
                        ),
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "fastn-module".to_string(),
                        kind: ftd::interpreter::Kind::string().into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ]).collect(),
                line_number: 0,
            }),
        ),
        (
            "ftd#string-field".to_string(),
            ftd::interpreter::Thing::Record(ftd::interpreter::Record {
                name: "ftd#string-field".to_string(),
                fields: std::iter::IntoIterator::into_iter([
                    ftd::interpreter::Field {
                        name: "name".to_string(),
                        kind: ftd::interpreter::Kind::string().into_kind_data().caption(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "value".to_string(),
                        kind: ftd::interpreter::Kind::string().into_kind_data(),
                        mutable: true,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "error".to_string(),
                        kind: ftd::interpreter::Kind::string().into_optional().into_kind_data(),
                        mutable: true,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ]).collect(),
                line_number: 0,
            }),
        ),        (
            "ftd#http-method".to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: "ftd#http-method".to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        "ftd#http-method.GET",
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("GET")
                                .into_property_value(false, 0),
                        ),
                        0
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        "ftd#http-method.POST",
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("POST")
                                .into_property_value(false, 0),
                        ),
                        0
                    )),
                ],
                line_number: 0,
            }),
        ),
        (
            "ftd#http-redirect".to_string(),
            ftd::interpreter::Thing::OrType(ftd::interpreter::OrType {
                name: "ftd#http-redirect".to_string(),
                variants: vec![
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        "ftd#http-redirect.follow",
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("follow")
                                .into_property_value(false, 0),
                        ),
                        0
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        "ftd#http-redirect.manual",
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("manual")
                                .into_property_value(false, 0),
                        ),
                        0
                    )),
                    ftd::interpreter::OrTypeVariant::new_constant(ftd::interpreter::Field::new(
                        "ftd#http-redirect.error",
                        ftd::interpreter::Kind::string().into_kind_data(),
                        false,
                        Some(
                            ftd::interpreter::Value::new_string("error")
                                .into_property_value(false, 0),
                        ),
                        0
                    )),
                ],
                line_number: 0,
            }),
        ),
    ];

    things.into_iter().collect()
}

pub fn default_test_bag() -> indexmap::IndexMap<String, ftd::interpreter::Thing> {
    let test_things = vec![
        (
            ftd::interpreter::FASTN_GET_QUERY_PARAMS.to_string(),
            ftd::interpreter::Thing::Record(ftd::interpreter::Record {
                name: ftd::interpreter::FASTN_GET_QUERY_PARAMS.to_string(),
                fields: std::iter::IntoIterator::into_iter([
                    ftd::interpreter::Field {
                        name: "key".to_string(),
                        kind: ftd::interpreter::Kind::string().into_kind_data().caption(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                    ftd::interpreter::Field {
                        name: "value".to_string(),
                        kind: ftd::interpreter::Kind::string().into_kind_data(),
                        mutable: false,
                        value: None,
                        access_modifier: Default::default(),
                        line_number: 0,
                    },
                ])
                .collect(),
                line_number: 0,
            }),
        ),
        (
            "fastn#get".to_string(),
            ftd::interpreter::Thing::Component(fastn_get_function()),
        ),
        (
            "fastn#post".to_string(),
            ftd::interpreter::Thing::Component(fastn_post_function()),
        ),
        (
            "fastn#redirect".to_string(),
            ftd::interpreter::Thing::Component(fastn_redirect_function()),
        ),
        (
            "fastn#test".to_string(),
            ftd::interpreter::Thing::Component(fastn_test_function()),
        ),
    ];
    test_things.into_iter().collect()
}

pub fn fastn_get_function() -> ftd::interpreter::ComponentDefinition {
    ftd::interpreter::ComponentDefinition {
        name: "fastn#get".to_string(),
        arguments: [vec![
            ftd::interpreter::Argument::default(
                "title",
                ftd::interpreter::Kind::string().into_kind_data().caption(),
            ),
            ftd::interpreter::Argument::default(
                "url",
                ftd::interpreter::Kind::string().into_kind_data(),
            ),
            ftd::interpreter::Argument::default(
                "test",
                ftd::interpreter::Kind::string()
                    .into_kind_data()
                    .into_optional(),
            ),
            ftd::interpreter::Argument::default(
                "http-status",
                ftd::interpreter::Kind::string()
                    .into_kind_data()
                    .into_optional(),
            ),
            ftd::interpreter::Argument::default(
                "http-location",
                ftd::interpreter::Kind::string()
                    .into_kind_data()
                    .into_optional(),
            ),
            ftd::interpreter::Argument::default(
                "http-redirect",
                ftd::interpreter::Kind::string()
                    .into_kind_data()
                    .into_optional(),
            ),
            ftd::interpreter::Argument::default(
                "id",
                ftd::interpreter::Kind::string()
                    .into_kind_data()
                    .into_optional(),
            ),
            ftd::interpreter::Argument::default(
                "query-params",
                ftd::interpreter::Kind::record(ftd::interpreter::FASTN_GET_QUERY_PARAMS)
                    .into_list()
                    .into_kind_data(),
            ),
        ]]
        .concat()
        .into_iter()
        .collect(),
        definition: ftd::interpreter::Component::from_name("ftd.kernel"),
        css: None,
        line_number: 0,
    }
}

pub fn fastn_post_function() -> ftd::interpreter::ComponentDefinition {
    ftd::interpreter::ComponentDefinition {
        name: "fastn#post".to_string(),
        arguments: [vec![
            ftd::interpreter::Argument::default(
                "title",
                ftd::interpreter::Kind::string().into_kind_data().caption(),
            ),
            ftd::interpreter::Argument::default(
                "url",
                ftd::interpreter::Kind::string().into_kind_data(),
            ),
            ftd::interpreter::Argument::default(
                "body",
                ftd::interpreter::Kind::string().into_kind_data().body(),
            ),
            ftd::interpreter::Argument::default(
                "test",
                ftd::interpreter::Kind::string()
                    .into_kind_data()
                    .into_optional(),
            ),
            ftd::interpreter::Argument::default(
                "http-status",
                ftd::interpreter::Kind::string()
                    .into_kind_data()
                    .into_optional(),
            ),
            ftd::interpreter::Argument::default(
                "http-location",
                ftd::interpreter::Kind::string()
                    .into_kind_data()
                    .into_optional(),
            ),
            ftd::interpreter::Argument::default(
                "http-redirect",
                ftd::interpreter::Kind::string()
                    .into_kind_data()
                    .into_optional(),
            ),
            ftd::interpreter::Argument::default(
                "id",
                ftd::interpreter::Kind::string()
                    .into_kind_data()
                    .into_optional(),
            ),
        ]]
        .concat()
        .into_iter()
        .collect(),
        definition: ftd::interpreter::Component::from_name("ftd.kernel"),
        css: None,
        line_number: 0,
    }
}

pub fn fastn_redirect_function() -> ftd::interpreter::ComponentDefinition {
    ftd::interpreter::ComponentDefinition {
        name: "fastn#redirect".to_string(),
        arguments: vec![ftd::interpreter::Argument::default(
            "http-redirect",
            ftd::interpreter::Kind::string().into_kind_data().caption(),
        )],
        definition: ftd::interpreter::Component::from_name("ftd.kernel"),
        css: None,
        line_number: 0,
    }
}

pub fn fastn_test_function() -> ftd::interpreter::ComponentDefinition {
    ftd::interpreter::ComponentDefinition {
        name: "fastn#test".to_string(),
        arguments: [vec![
            ftd::interpreter::Argument::default(
                "title",
                ftd::interpreter::Kind::string()
                    .into_kind_data()
                    .caption()
                    .into_optional(),
            ),
            ftd::interpreter::Argument::default(
                "fixtures",
                ftd::interpreter::Kind::string()
                    .into_list()
                    .into_kind_data(),
            ),
        ]]
        .concat()
        .into_iter()
        .collect(),
        definition: ftd::interpreter::Component::from_name("ftd.kernel"),
        css: None,
        line_number: 0,
    }
}

pub static DEFAULT_BAG: once_cell::sync::OnceCell<
    indexmap::IndexMap<String, ftd::interpreter::things::Thing>,
> = once_cell::sync::OnceCell::new();

pub fn get_default_bag() -> &'static indexmap::IndexMap<String, ftd::interpreter::things::Thing> {
    DEFAULT_BAG.get_or_init(ftd::interpreter::things::default::default_bag)
}

pub fn image_function() -> ftd::interpreter::ComponentDefinition {
    ftd::interpreter::ComponentDefinition {
        name: "ftd#image".to_string(),
        arguments: [
            common_arguments(),
            vec![
                ftd::interpreter::Argument::default(
                    "src",
                    ftd::interpreter::Kind::record(ftd::interpreter::FTD_IMAGE_SRC)
                        .into_kind_data()
                        .caption(),
                ),
                ftd::interpreter::Argument::default(
                    "fit",
                    ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_IMAGE_FIT)
                        .into_kind_data()
                        .into_optional(),
                ),
                ftd::interpreter::Argument::default(
                    "alt",
                    ftd::interpreter::Kind::string()
                        .into_kind_data()
                        .into_optional(),
                ),
                ftd::interpreter::Argument::default(
                    "fetch-priority",
                    ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_IMAGE_FETCH_PRIORITY)
                        .into_kind_data()
                        .into_optional(),
                ),
            ],
        ]
        .concat()
        .into_iter()
        .collect(),
        definition: ftd::interpreter::Component::from_name("ftd.kernel"),
        css: None,
        line_number: 0,
    }
}

pub fn video_function() -> ftd::interpreter::ComponentDefinition {
    ftd::interpreter::ComponentDefinition {
        name: "ftd#video".to_string(),
        arguments: [
            common_arguments(),
            vec![
                ftd::interpreter::Argument::default(
                    "src",
                    ftd::interpreter::Kind::record(ftd::interpreter::FTD_VIDEO_SRC)
                        .into_kind_data()
                        .caption(),
                ),
                ftd::interpreter::Argument::default(
                    "fit",
                    ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_IMAGE_FIT)
                        .into_kind_data()
                        .into_optional(),
                ),
                ftd::interpreter::Argument::default(
                    "controls",
                    ftd::interpreter::Kind::boolean()
                        .into_optional()
                        .into_kind_data(),
                ),
                ftd::interpreter::Argument::default(
                    "loop",
                    ftd::interpreter::Kind::boolean()
                        .into_optional()
                        .into_kind_data(),
                ),
                ftd::interpreter::Argument::default(
                    "autoplay",
                    ftd::interpreter::Kind::boolean()
                        .into_optional()
                        .into_kind_data(),
                ),
                ftd::interpreter::Argument::default(
                    "muted",
                    ftd::interpreter::Kind::boolean()
                        .into_optional()
                        .into_kind_data(),
                ),
                ftd::interpreter::Argument::default(
                    "poster",
                    ftd::interpreter::Kind::record(ftd::interpreter::FTD_IMAGE_SRC)
                        .into_optional()
                        .into_kind_data(),
                ),
            ],
        ]
        .concat()
        .into_iter()
        .collect(),
        definition: ftd::interpreter::Component::from_name("ftd.kernel"),
        css: None,
        line_number: 0,
    }
}

pub fn boolean_function() -> ftd::interpreter::ComponentDefinition {
    ftd::interpreter::ComponentDefinition {
        name: "ftd#boolean".to_string(),
        arguments: [
            text_arguments(),
            common_arguments(),
            vec![
                ftd::interpreter::Argument::default(
                    "value",
                    ftd::interpreter::Kind::boolean()
                        .into_kind_data()
                        .caption_or_body(),
                ),
                ftd::interpreter::Argument::default(
                    "style",
                    ftd::interpreter::Kind::string()
                        .into_optional()
                        .into_kind_data(),
                ),
                ftd::interpreter::Argument::default(
                    "format",
                    ftd::interpreter::Kind::string()
                        .into_optional()
                        .into_kind_data(),
                ),
                ftd::interpreter::Argument::default(
                    "text-align",
                    ftd::interpreter::Kind::string()
                        .into_optional()
                        .into_kind_data(),
                ),
            ],
        ]
        .concat()
        .into_iter()
        .collect(),
        definition: ftd::interpreter::Component::from_name("ftd.kernel"),
        css: None,
        line_number: 0,
    }
}

pub fn checkbox_function() -> ftd::interpreter::ComponentDefinition {
    ftd::interpreter::ComponentDefinition {
        name: "ftd#checkbox".to_string(),
        arguments: [
            common_arguments(),
            vec![
                ftd::interpreter::Argument::default(
                    "checked",
                    ftd::interpreter::Kind::boolean()
                        .into_optional()
                        .into_kind_data(),
                ),
                ftd::interpreter::Argument::default(
                    "enabled",
                    ftd::interpreter::Kind::boolean()
                        .into_optional()
                        .into_kind_data(),
                ),
            ],
        ]
        .concat()
        .into_iter()
        .collect(),
        definition: ftd::interpreter::Component::from_name("ftd.kernel"),
        css: None,
        line_number: 0,
    }
}

pub fn text_input_function() -> ftd::interpreter::ComponentDefinition {
    ftd::interpreter::ComponentDefinition {
        name: "ftd#text-input".to_string(),
        arguments: [
            text_arguments(),
            common_arguments(),
            vec![
                ftd::interpreter::Argument::default(
                    "placeholder",
                    ftd::interpreter::Kind::string()
                        .into_optional()
                        .into_kind_data(),
                ),
                ftd::interpreter::Argument::default(
                    "value",
                    ftd::interpreter::Kind::string()
                        .into_optional()
                        .into_kind_data(),
                ),
                ftd::interpreter::Argument::default(
                    "default-value",
                    ftd::interpreter::Kind::string()
                        .into_optional()
                        .into_kind_data(),
                ),
                ftd::interpreter::Argument::default(
                    "multiline",
                    ftd::interpreter::Kind::boolean()
                        .into_optional()
                        .into_kind_data(),
                ),
                ftd::interpreter::Argument::default(
                    "enabled",
                    ftd::interpreter::Kind::boolean()
                        .into_optional()
                        .into_kind_data(),
                ),
                ftd::interpreter::Argument::default(
                    "max-length",
                    ftd::interpreter::Kind::integer()
                        .into_optional()
                        .into_kind_data(),
                ),
                ftd::interpreter::Argument::default(
                    "type",
                    ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_TEXT_INPUT_TYPE)
                        .into_optional()
                        .into_kind_data(),
                ),
            ],
        ]
        .concat()
        .into_iter()
        .collect(),
        definition: ftd::interpreter::Component::from_name("ftd.kernel"),
        css: None,
        line_number: 0,
    }
}

pub fn integer_function() -> ftd::interpreter::ComponentDefinition {
    ftd::interpreter::ComponentDefinition {
        name: "ftd#integer".to_string(),
        arguments: [
            text_arguments(),
            common_arguments(),
            vec![
                ftd::interpreter::Argument::default(
                    "value",
                    ftd::interpreter::Kind::integer()
                        .into_kind_data()
                        .caption_or_body(),
                ),
                ftd::interpreter::Argument::default(
                    "style",
                    ftd::interpreter::Kind::string()
                        .into_optional()
                        .into_kind_data(),
                ),
                ftd::interpreter::Argument::default(
                    "format",
                    ftd::interpreter::Kind::string()
                        .into_optional()
                        .into_kind_data(),
                ),
                ftd::interpreter::Argument::default(
                    "text-align",
                    ftd::interpreter::Kind::string()
                        .into_optional()
                        .into_kind_data(),
                ),
            ],
        ]
        .concat()
        .into_iter()
        .collect(),
        definition: ftd::interpreter::Component::from_name("ftd.kernel"),
        css: None,
        line_number: 0,
    }
}

pub fn decimal_function() -> ftd::interpreter::ComponentDefinition {
    ftd::interpreter::ComponentDefinition {
        name: "ftd#decimal".to_string(),
        arguments: [
            text_arguments(),
            common_arguments(),
            vec![
                ftd::interpreter::Argument::default(
                    "value",
                    ftd::interpreter::Kind::decimal()
                        .into_kind_data()
                        .caption_or_body(),
                ),
                ftd::interpreter::Argument::default(
                    "style",
                    ftd::interpreter::Kind::string()
                        .into_optional()
                        .into_kind_data(),
                ),
                ftd::interpreter::Argument::default(
                    "format",
                    ftd::interpreter::Kind::string()
                        .into_optional()
                        .into_kind_data(),
                ),
            ],
        ]
        .concat()
        .into_iter()
        .collect(),
        definition: ftd::interpreter::Component::from_name("ftd.kernel"),
        css: None,
        line_number: 0,
    }
}

pub fn markup_function() -> ftd::interpreter::ComponentDefinition {
    ftd::interpreter::ComponentDefinition {
        name: "ftd#text".to_string(),
        arguments: [
            text_arguments(),
            common_arguments(),
            vec![ftd::interpreter::Argument::default(
                "text",
                ftd::interpreter::Kind::string()
                    .into_kind_data()
                    .caption_or_body(),
            )],
        ]
        .concat()
        .into_iter()
        .collect(),
        definition: ftd::interpreter::Component::from_name("ftd.kernel"),
        css: None,
        line_number: 0,
    }
}

pub fn row_function() -> ftd::interpreter::ComponentDefinition {
    ftd::interpreter::ComponentDefinition {
        name: "ftd#row".to_string(),
        arguments: [
            container_root_arguments(),
            container_arguments(),
            common_arguments(),
        ]
        .concat()
        .into_iter()
        .collect(),
        definition: ftd::interpreter::Component::from_name("ftd.kernel"),
        css: None,
        line_number: 0,
    }
}

pub fn rive_function() -> ftd::interpreter::ComponentDefinition {
    use itertools::Itertools;

    ftd::interpreter::ComponentDefinition {
        name: "ftd#rive".to_string(),
        arguments: [
            common_arguments()
                .into_iter()
                .filter(|v| v.name.ne("id"))
                .collect_vec(),
            vec![
                ftd::interpreter::Argument::default(
                    "id",
                    ftd::interpreter::Kind::string().into_kind_data().caption(),
                ),
                ftd::interpreter::Argument::default(
                    "src",
                    ftd::interpreter::Kind::string().into_kind_data(),
                ),
                ftd::interpreter::Argument::default(
                    "canvas-width",
                    ftd::interpreter::Kind::integer()
                        .into_optional()
                        .into_kind_data(),
                ),
                ftd::interpreter::Argument::default(
                    "canvas-height",
                    ftd::interpreter::Kind::integer()
                        .into_optional()
                        .into_kind_data(),
                ),
                ftd::interpreter::Argument::default(
                    "state-machine",
                    ftd::interpreter::Kind::string()
                        .into_list()
                        .into_kind_data(),
                ),
                ftd::interpreter::Argument {
                    name: "autoplay".to_string(),
                    kind: ftd::interpreter::Kind::boolean().into_kind_data(),
                    mutable: false,
                    value: Some(ftd::interpreter::PropertyValue::Value {
                        value: ftd::interpreter::Value::Boolean { value: true },
                        is_mutable: false,
                        line_number: 0,
                    }),
                    access_modifier: Default::default(),
                    line_number: 0,
                },
                ftd::interpreter::Argument::default(
                    "artboard",
                    ftd::interpreter::Kind::string()
                        .into_optional()
                        .into_kind_data(),
                ),
            ],
        ]
        .concat()
        .into_iter()
        .collect(),
        definition: ftd::interpreter::Component::from_name("ftd.kernel"),
        css: None,
        line_number: 0,
    }
}

pub fn container_function() -> ftd::interpreter::ComponentDefinition {
    ftd::interpreter::ComponentDefinition {
        name: "ftd#container".to_string(),
        arguments: [
            container_root_arguments(),
            common_arguments(),
            vec![ftd::interpreter::Argument::default(
                "display",
                ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_DISPLAY)
                    .into_optional()
                    .into_kind_data(),
            )],
        ]
        .concat()
        .into_iter()
        .collect(),
        definition: ftd::interpreter::Component::from_name("ftd.kernel"),
        css: None,
        line_number: 0,
    }
}

pub fn desktop_function() -> ftd::interpreter::ComponentDefinition {
    ftd::interpreter::ComponentDefinition {
        name: "ftd#desktop".to_string(),
        arguments: [container_root_arguments()].concat().into_iter().collect(),
        definition: ftd::interpreter::Component::from_name("ftd.kernel"),
        css: None,
        line_number: 0,
    }
}

pub fn mobile_function() -> ftd::interpreter::ComponentDefinition {
    ftd::interpreter::ComponentDefinition {
        name: "ftd#mobile".to_string(),
        arguments: [container_root_arguments()].concat().into_iter().collect(),
        definition: ftd::interpreter::Component::from_name("ftd.kernel"),
        css: None,
        line_number: 0,
    }
}

pub fn code_function() -> ftd::interpreter::ComponentDefinition {
    ftd::interpreter::ComponentDefinition {
        name: "ftd#code".to_string(),
        arguments: [
            text_arguments(),
            common_arguments(),
            vec![
                ftd::interpreter::Argument::default(
                    "text",
                    ftd::interpreter::Kind::string()
                        .into_kind_data()
                        .caption_or_body(),
                ),
                // TODO: Added `txt` as default
                ftd::interpreter::Argument::default(
                    "lang",
                    ftd::interpreter::Kind::string()
                        .into_optional()
                        .into_kind_data(),
                ),
                // TODO: Added `CODE_DEFAULT_THEME` as default
                ftd::interpreter::Argument::default(
                    "theme",
                    ftd::interpreter::Kind::string()
                        .into_optional()
                        .into_kind_data(),
                ),
                ftd::interpreter::Argument::default_with_value(
                    "show-line-number",
                    ftd::interpreter::Kind::boolean().into_kind_data(),
                    ftd::interpreter::Value::Boolean { value: false }.into_property_value(false, 0),
                ),
            ],
        ]
        .concat()
        .into_iter()
        .collect(),
        definition: ftd::interpreter::Component::from_name("ftd.kernel"),
        css: None,
        line_number: 0,
    }
}

pub fn iframe_function() -> ftd::interpreter::ComponentDefinition {
    ftd::interpreter::ComponentDefinition {
        name: "ftd#iframe".to_string(),
        arguments: [
            common_arguments(),
            vec![
                ftd::interpreter::Argument::default(
                    "src",
                    ftd::interpreter::Kind::string()
                        .into_optional()
                        .into_kind_data()
                        .caption(),
                ),
                ftd::interpreter::Argument::default(
                    "youtube",
                    ftd::interpreter::Kind::string()
                        .into_optional()
                        .into_kind_data(),
                ),
                ftd::interpreter::Argument::default(
                    "srcdoc",
                    ftd::interpreter::Kind::string()
                        .into_optional()
                        .into_kind_data()
                        .body(),
                ),
                ftd::interpreter::Argument::default(
                    "loading",
                    ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LOADING)
                        .into_optional()
                        .into_kind_data(),
                ),
            ],
        ]
        .concat()
        .into_iter()
        .collect(),
        definition: ftd::interpreter::Component::from_name("ftd.kernel"),
        css: None,
        line_number: 0,
    }
}

pub fn column_function() -> ftd::interpreter::ComponentDefinition {
    ftd::interpreter::ComponentDefinition {
        name: "ftd#column".to_string(),
        arguments: [
            container_root_arguments(),
            container_arguments(),
            common_arguments(),
        ]
        .concat()
        .into_iter()
        .collect(),
        definition: ftd::interpreter::Component::from_name("ftd.kernel"),
        css: None,
        line_number: 0,
    }
}

pub fn document_function() -> ftd::interpreter::ComponentDefinition {
    ftd::interpreter::ComponentDefinition {
        name: "ftd#document".to_string(),
        arguments: [vec![
            ftd::interpreter::Argument::default(
                "favicon",
                ftd::interpreter::Kind::record(ftd::interpreter::FTD_RAW_IMAGE_SRC)
                    .into_optional()
                    .into_kind_data(),
            ),
            ftd::interpreter::Argument::default(
                "breakpoint",
                ftd::interpreter::Kind::record(ftd::interpreter::FTD_BREAKPOINT_WIDTH_DATA)
                    .into_optional()
                    .into_kind_data(),
            ),
            ftd::interpreter::Argument::default(
                "facebook-domain-verification",
                ftd::interpreter::Kind::string()
                    .into_optional()
                    .into_kind_data(),
            ),
            ftd::interpreter::Argument::default(
                "title",
                ftd::interpreter::Kind::string()
                    .into_optional()
                    .into_kind_data()
                    .caption_or_body(),
            ),
            ftd::interpreter::Argument {
                name: "og-title".to_string(),
                kind: ftd::interpreter::Kind::string()
                    .into_optional()
                    .into_kind_data(),
                mutable: false,
                value: Some(ftd::interpreter::PropertyValue::Reference {
                    name: "ftd#document.title".to_string(),
                    kind: ftd::interpreter::Kind::string()
                        .into_optional()
                        .into_kind_data(),
                    source: ftd::interpreter::PropertyValueSource::Local("document".to_string()),
                    is_mutable: false,
                    line_number: 0,
                }),
                access_modifier: Default::default(),
                line_number: 0,
            },
            ftd::interpreter::Argument {
                name: "twitter-title".to_string(),
                kind: ftd::interpreter::Kind::string()
                    .into_optional()
                    .into_kind_data(),
                mutable: false,
                value: Some(ftd::interpreter::PropertyValue::Reference {
                    name: "ftd#document.title".to_string(),
                    kind: ftd::interpreter::Kind::string()
                        .into_optional()
                        .into_kind_data(),
                    source: ftd::interpreter::PropertyValueSource::Local("document".to_string()),
                    is_mutable: false,
                    line_number: 0,
                }),
                access_modifier: Default::default(),
                line_number: 0,
            },
            ftd::interpreter::Argument::default(
                "description",
                ftd::interpreter::Kind::string()
                    .into_optional()
                    .into_kind_data(),
            ),
            ftd::interpreter::Argument {
                name: "og-description".to_string(),
                kind: ftd::interpreter::Kind::string()
                    .into_optional()
                    .into_kind_data(),
                mutable: false,
                value: Some(ftd::interpreter::PropertyValue::Reference {
                    name: "ftd#document.description".to_string(),
                    kind: ftd::interpreter::Kind::string()
                        .into_optional()
                        .into_kind_data(),
                    source: ftd::interpreter::PropertyValueSource::Local("document".to_string()),
                    is_mutable: false,
                    line_number: 0,
                }),
                access_modifier: Default::default(),
                line_number: 0,
            },
            ftd::interpreter::Argument {
                name: "twitter-description".to_string(),
                kind: ftd::interpreter::Kind::string()
                    .into_optional()
                    .into_kind_data(),
                mutable: false,
                value: Some(ftd::interpreter::PropertyValue::Reference {
                    name: "ftd#document.description".to_string(),
                    kind: ftd::interpreter::Kind::string()
                        .into_optional()
                        .into_kind_data(),
                    source: ftd::interpreter::PropertyValueSource::Local("document".to_string()),
                    is_mutable: false,
                    line_number: 0,
                }),
                access_modifier: Default::default(),
                line_number: 0,
            },
            ftd::interpreter::Argument::default(
                "og-image",
                ftd::interpreter::Kind::record(ftd::interpreter::FTD_RAW_IMAGE_SRC)
                    .into_optional()
                    .into_kind_data(),
            ),
            ftd::interpreter::Argument {
                name: "twitter-image".to_string(),
                kind: ftd::interpreter::Kind::record(ftd::interpreter::FTD_RAW_IMAGE_SRC)
                    .into_optional()
                    .into_kind_data(),
                mutable: false,
                value: Some(ftd::interpreter::PropertyValue::Reference {
                    name: "ftd#document.og-image".to_string(),
                    kind: ftd::interpreter::Kind::string().into_kind_data(),
                    source: ftd::interpreter::PropertyValueSource::Local("document".to_string()),
                    is_mutable: false,
                    line_number: 0,
                }),
                access_modifier: Default::default(),
                line_number: 0,
            },
            ftd::interpreter::Argument::default(
                "theme-color",
                ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                    .into_optional()
                    .into_kind_data(),
            ),
            ftd::interpreter::Argument::default(
                "children",
                ftd::interpreter::Kind::subsection_ui()
                    .into_list()
                    .into_kind_data(),
            ),
            ftd::interpreter::Argument::default(
                "colors",
                ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR_SCHEME)
                    .into_optional()
                    .into_kind_data(),
            ),
            ftd::interpreter::Argument::default(
                "types",
                ftd::interpreter::Kind::record(ftd::interpreter::FTD_TYPE_DATA)
                    .into_optional()
                    .into_kind_data(),
            ),
        ]]
        .concat()
        .into_iter()
        .collect(),
        definition: ftd::interpreter::Component::from_name("ftd.kernel"),
        css: None,
        line_number: 0,
    }
}

fn container_root_arguments() -> Vec<ftd::interpreter::Argument> {
    vec![
        ftd::interpreter::Argument::default(
            "children",
            ftd::interpreter::Kind::subsection_ui()
                .into_list()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "colors",
            ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR_SCHEME)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "types",
            ftd::interpreter::Kind::record(ftd::interpreter::FTD_TYPE_DATA)
                .into_optional()
                .into_kind_data(),
        ),
    ]
}

fn container_arguments() -> Vec<ftd::interpreter::Argument> {
    vec![
        ftd::interpreter::Argument::default(
            "wrap",
            ftd::interpreter::Kind::boolean()
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "align-content",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_ALIGN)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "spacing",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_SPACING)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "backdrop-filter",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_BACKDROP_FILTER)
                .into_optional()
                .into_kind_data(),
        ),
    ]
}

fn common_arguments() -> Vec<ftd::interpreter::Argument> {
    vec![
        ftd::interpreter::Argument::default(
            "opacity",
            ftd::interpreter::Kind::decimal()
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "shadow",
            ftd::interpreter::Kind::record(ftd::interpreter::FTD_SHADOW)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "sticky",
            ftd::interpreter::Kind::boolean()
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "rel",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LINK_REL)
                .into_list()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "id",
            ftd::interpreter::Kind::string()
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "border-style",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_BORDER_STYLE)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "border-style-left",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_BORDER_STYLE)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "border-style-right",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_BORDER_STYLE)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "border-style-top",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_BORDER_STYLE)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "border-style-bottom",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_BORDER_STYLE)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "border-style-vertical",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_BORDER_STYLE)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "border-style-horizontal",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_BORDER_STYLE)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "z-index",
            ftd::interpreter::Kind::integer()
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "white-space",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_WHITESPACE)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "text-transform",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_TEXT_TRANSFORM)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "region",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_REGION)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "left",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "right",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "top",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "bottom",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "anchor",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_ANCHOR)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "role",
            ftd::interpreter::Kind::record(ftd::interpreter::FTD_RESPONSIVE_TYPE)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "cursor",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_CURSOR)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "classes",
            ftd::interpreter::Kind::string()
                .into_list()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "js",
            ftd::interpreter::Kind::string()
                .into_list()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "css",
            ftd::interpreter::Kind::string()
                .into_list()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "open-in-new-tab",
            ftd::interpreter::Kind::boolean()
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "resize",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_RESIZE)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "overflow",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_OVERFLOW)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "overflow-x",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_OVERFLOW)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "overflow-y",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_OVERFLOW)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "align-self",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_ALIGN_SELF)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "background",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_BACKGROUND)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "border-color",
            ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "color",
            ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "max-width",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_RESIZING)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "min-width",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_RESIZING)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "min-height",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_RESIZING)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "max-height",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_RESIZING)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "width",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_RESIZING)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "height",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_RESIZING)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "padding",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "padding-vertical",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "padding-horizontal",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "padding-left",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "padding-right",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "padding-top",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "padding-bottom",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "margin",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "margin-vertical",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "margin-horizontal",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "margin-left",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "margin-right",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "margin-top",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "margin-bottom",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "border-width",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "border-bottom-width",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "border-bottom-color",
            ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "border-top-width",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "border-top-color",
            ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "border-left-width",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "border-left-color",
            ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "border-right-width",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "border-right-color",
            ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "border-radius",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "border-top-left-radius",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "border-top-right-radius",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "border-bottom-left-radius",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "border-bottom-right-radius",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "link",
            ftd::interpreter::Kind::string()
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "selectable",
            ftd::interpreter::Kind::boolean()
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "mask",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_MASK)
                .into_optional()
                .into_kind_data(),
        ),
    ]
}

fn text_arguments() -> Vec<ftd::interpreter::Argument> {
    vec![
        ftd::interpreter::Argument::default(
            "display",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_DISPLAY)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "text-align",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_TEXT_ALIGN)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "line-clamp",
            ftd::interpreter::Kind::integer()
                .into_kind_data()
                .into_optional(),
        ),
        ftd::interpreter::Argument::default(
            "text-indent",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_LENGTH)
                .into_kind_data()
                .into_optional(),
        ),
        ftd::interpreter::Argument::default(
            "style",
            ftd::interpreter::Kind::or_type(ftd::interpreter::FTD_TEXT_STYLE)
                .into_list()
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "link-color",
            ftd::interpreter::Kind::record(ftd::interpreter::FTD_COLOR)
                .into_optional()
                .into_kind_data(),
        ),
        ftd::interpreter::Argument::default(
            "text-shadow",
            ftd::interpreter::Kind::record(ftd::interpreter::FTD_SHADOW)
                .into_optional()
                .into_kind_data(),
        ),
    ]
}

/*fn kernel_component() -> ftd::interpreter::ComponentDefinition {
    ftd::interpreter::ComponentDefinition {
        name: "ftd.kernel".to_string(),
        arguments: vec![],
        definition: ftd::interpreter::Component {
            name: "ftd.kernel".to_string(),
            properties: vec![],
            iteration: Box::new(None),
            condition: Box::new(None),
            events: vec![],
            children: vec![],
            line_number: 0,
        },
        line_number: 0,
    }
}*/
