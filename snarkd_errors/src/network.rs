use super::CreateErrorType;

CreateErrorType! {
  NetworkError

  from read_packet_error {
    args: (),
    error_msgs: [
        "Could not read packet length.",
    ],
    suggestions: [],
  }
}
