#![allow(unknown_lints)]
#![allow(missing_docs)]

use serde_json;

error_chain! {
  foreign_links {
    Json(serde_json::Error);
  }
  errors {
    Decoder(e: String) {
        description("decoder error"),
        display("Decoder error: {}", e)
    }
  }
}