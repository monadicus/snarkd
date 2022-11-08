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

  from_error prost_decode_error {
    args: (),
    error_msgs: [
        "Failed decoding Protobuf Message.",
    ],
    suggestions: [],
  }

  from_error invalid_ir {
    args: (),
    error_msgs: [
        "Invalid IR.",
    ],
    suggestions: [],
  }

  from_error failed_to_read_ir_file {
    args: (),
    error_msgs: [
        "Failed to read the IR file.",
    ],
    suggestions: [],
  }

  from_error failed_to_open_ir_file {
    args: (file),
    error_msgs: [
        "Failed to read ir file `{file}`.",
    ],
    suggestions: [],
  }

  from_error failed_to_write_ir_file {
    args: (mode),
    error_msgs: [
        "Failed to write the IR {mode} file.",
    ],
    suggestions: [],
  }

  unwrapped invalid_commit_method {
    args: (),
    error_msgs: [
        "Invalid commit method.",
    ],
    suggestions: [],
  }

  unwrapped invalid_hash_method {
    args: (),
    error_msgs: [
        "Invalid hash method.",
    ],
    suggestions: [],
  }
}
