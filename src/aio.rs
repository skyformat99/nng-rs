//!Most applications will interact with nng synchronously; that is that functions such as nng_send()
//!will block the calling thread until the operation has completed.
//!iSynchronous operations which send messages may return before the message
//!has actually been received, or even transmitted. Instead, These functions return
//!as soon as the message was successfully queued for delivery.
//!Asynchronous operations behave differently. These operations are initiated by the calling thread,
//!but control returns immediately to the calling thread. When the operation is subsequently
//!completed (regardless of whether this was successful or not), then a user supplied function
//!(“callback”) is executed.
//!A context structure, an nng_aio, is allocated and associated with each asynchronous operation.
//!Only a single asynchronous operation may be associated with an nng_aio at any time.
//!The following functions are used in the asynchronous model:
