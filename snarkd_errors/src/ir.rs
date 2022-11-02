use super::CreateErrorType;

CreateErrorType! {
  IRError

  unwrapped unset {
    args: (item),
    error_msgs: [
        "{item} unset.",
    ],
    suggestions: [],
  }

  unwrapped invalid_num_of_bytes {
    args: (type_),
    error_msgs: [
        "Invalid number of bytes for type: `{type_}`.",
    ],
    suggestions: [],
  }

  from_error cast_int_error {
    args: (),
    error_msgs: [
        "Could not cast int to target type.",
    ],
    suggestions: [],
  }

  unwrapped tuple_group_set_side_unset {
    args: (side),
    error_msgs: [
        "The `{side}` of the TupleGroup is unset.",
    ],
    suggestions: [],
  }

  unwrapped invalid_visibility {
    args: (item),
    error_msgs: [
        "Invalid visibility for `{item}`.",
    ],
    suggestions: [],
  }

  unwrapped missing_operand {
    args: (item),
    error_msgs: [
        "Missing operand for `{item}`.",
    ],
    suggestions: [],
  }
}
