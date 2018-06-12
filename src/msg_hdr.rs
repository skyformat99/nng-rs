//!Message Handling Functions
//!Applications desiring to use the richest part of libnng will want to use the message API, where a
//!message structure is passed between functions. This API provides the most power support for
//!zero-copy.
//!Messages are divided into a header and body, where the body generally carries user-payload and
//!the header carries protocol specific header information. Most applications will only interact with
//!the body.
