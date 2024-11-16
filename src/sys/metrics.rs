use crate::{
    bindings::{
        Cronet_MetricsPtr, Cronet_Metrics_Create, Cronet_Metrics_Destroy,
        Cronet_Metrics_connect_end_get, Cronet_Metrics_connect_end_move,
        Cronet_Metrics_connect_end_set, Cronet_Metrics_connect_start_get,
        Cronet_Metrics_connect_start_move, Cronet_Metrics_connect_start_set,
        Cronet_Metrics_dns_end_get, Cronet_Metrics_dns_end_move, Cronet_Metrics_dns_end_set,
        Cronet_Metrics_dns_start_get, Cronet_Metrics_dns_start_move, Cronet_Metrics_dns_start_set,
        Cronet_Metrics_push_end_get, Cronet_Metrics_push_end_move, Cronet_Metrics_push_end_set,
        Cronet_Metrics_push_start_get, Cronet_Metrics_push_start_move,
        Cronet_Metrics_push_start_set, Cronet_Metrics_received_byte_count_get,
        Cronet_Metrics_received_byte_count_set, Cronet_Metrics_request_end_get,
        Cronet_Metrics_request_end_move, Cronet_Metrics_request_end_set,
        Cronet_Metrics_request_start_get, Cronet_Metrics_request_start_move,
        Cronet_Metrics_request_start_set, Cronet_Metrics_response_start_get,
        Cronet_Metrics_response_start_move, Cronet_Metrics_response_start_set,
        Cronet_Metrics_sending_end_get, Cronet_Metrics_sending_end_move,
        Cronet_Metrics_sending_end_set, Cronet_Metrics_sending_start_get,
        Cronet_Metrics_sending_start_move, Cronet_Metrics_sending_start_set,
        Cronet_Metrics_sent_byte_count_get, Cronet_Metrics_sent_byte_count_set,
        Cronet_Metrics_socket_reused_get, Cronet_Metrics_socket_reused_set,
        Cronet_Metrics_ssl_end_get, Cronet_Metrics_ssl_end_move, Cronet_Metrics_ssl_end_set,
        Cronet_Metrics_ssl_start_get, Cronet_Metrics_ssl_start_move, Cronet_Metrics_ssl_start_set,
    },
    util::define_impl,
};

use super::{date_time::DateTime, Borrowed};

impl<'a> Metrics {
    pub(crate) fn as_ptr(&self) -> Cronet_MetricsPtr {
        self.ptr
    }

    pub(crate) fn into_raw(self) -> Cronet_MetricsPtr {
        self.ptr
    }

    pub(crate) unsafe fn borrow_from_ptr(ptr: Cronet_MetricsPtr) -> Option<&'a mut Metrics> {
        if ptr.is_null() {
            return None;
        }
        let borrowed = Metrics { ptr };
        let ptr = Box::into_raw(Box::new(borrowed));
        Some(&mut *ptr)
    }

    pub fn borrow_from<X>(ptr: Cronet_MetricsPtr, lifetime: &'a X) -> Borrowed<'a, Metrics> {
        let borrowed = Metrics { ptr };
        let ptr = Box::into_raw(Box::new(borrowed));
        Borrowed::new(ptr, lifetime)
    }
}

define_impl! {
    Metrics, Cronet_MetricsPtr, Cronet_Metrics_Destroy,

    fn request_start_set(&mut Self, request_start: &DateTime >> DateTime::as_ptr);
        Cronet_Metrics_request_start_set,
    fn request_start_move(&Self, request_start: DateTime >> DateTime::into_raw);
        Cronet_Metrics_request_start_move,
    fn request_start_get(&Self) -> Option<&mut DateTime> >> DateTime::borrow_from_ptr;
        Cronet_Metrics_request_start_get,

    fn dns_start_set(&mut Self, dns_start: &DateTime >> DateTime::as_ptr);
        Cronet_Metrics_dns_start_set,
    fn dns_start_move(&Self, dns_start: DateTime >> DateTime::into_raw);
        Cronet_Metrics_dns_start_move,
    fn dns_start_get(&Self) -> Option<&mut DateTime> >> DateTime::borrow_from_ptr;
        Cronet_Metrics_dns_start_get,

    fn dns_end_set(&mut Self, dns_end: &DateTime >> DateTime::as_ptr);
        Cronet_Metrics_dns_end_set,
    fn dns_end_move(&Self, dns_end: DateTime >> DateTime::into_raw);
        Cronet_Metrics_dns_end_move,
    fn dns_end_get(&Self) -> Option<&mut DateTime> >> DateTime::borrow_from_ptr;
        Cronet_Metrics_dns_end_get,

    fn connect_start_set(&mut Self, connect_start: &DateTime >> DateTime::as_ptr);
        Cronet_Metrics_connect_start_set,
    fn connect_start_move(&Self, connect_start: DateTime >> DateTime::into_raw);
        Cronet_Metrics_connect_start_move,
    fn connect_start_get(&Self) -> Option<&mut DateTime> >> DateTime::borrow_from_ptr;
        Cronet_Metrics_connect_start_get,

    fn connect_end_set(&mut Self, connect_end: &DateTime >> DateTime::as_ptr);
        Cronet_Metrics_connect_end_set,
    fn connect_end_move(&Self, connect_end: DateTime >> DateTime::into_raw);
        Cronet_Metrics_connect_end_move,
    fn connect_end_get(&Self) -> Option<&mut DateTime> >> DateTime::borrow_from_ptr;
        Cronet_Metrics_connect_end_get,

    fn ssl_start_set(&mut Self, ssl_start: &DateTime >> DateTime::as_ptr);
        Cronet_Metrics_ssl_start_set,
    fn ssl_start_move(&Self, ssl_start: DateTime >> DateTime::into_raw);
        Cronet_Metrics_ssl_start_move,
    fn ssl_start_get(&Self) -> Option<&mut DateTime> >> DateTime::borrow_from_ptr;
        Cronet_Metrics_ssl_start_get,

    fn ssl_end_set(&mut Self, ssl_end: &DateTime >> DateTime::as_ptr);
        Cronet_Metrics_ssl_end_set,
    fn ssl_end_move(&Self, ssl_end: DateTime >> DateTime::into_raw);
        Cronet_Metrics_ssl_end_move,
    fn ssl_end_get(&Self) -> Option<&mut DateTime> >> DateTime::borrow_from_ptr;
        Cronet_Metrics_ssl_end_get,


    fn sending_start_set(&mut Self, sending_start: &DateTime >> DateTime::as_ptr);
        Cronet_Metrics_sending_start_set,
    fn sending_start_move(&Self, sending_start: DateTime >> DateTime::into_raw);
        Cronet_Metrics_sending_start_move,
    fn sending_start_get(&Self) -> Option<&mut DateTime> >> DateTime::borrow_from_ptr;
        Cronet_Metrics_sending_start_get,

    fn sending_end_set(&mut Self, sending_start: &DateTime >> DateTime::as_ptr);
        Cronet_Metrics_sending_end_set,
    fn sending_end_move(&Self, sending_start: DateTime >> DateTime::into_raw);
        Cronet_Metrics_sending_end_move,
    fn sending_end_get(&Self) -> Option<&mut DateTime> >> DateTime::borrow_from_ptr;
        Cronet_Metrics_sending_end_get,

    fn push_start_set(&mut Self, push_start: &DateTime >> DateTime::as_ptr);
        Cronet_Metrics_push_start_set,
    fn push_start_move(&Self, push_start: DateTime >> DateTime::into_raw);
        Cronet_Metrics_push_start_move,
    fn push_start_get(&Self) -> Option<&mut DateTime> >> DateTime::borrow_from_ptr;
        Cronet_Metrics_push_start_get,

    fn push_end_set(&mut Self, push_end: &DateTime >> DateTime::as_ptr);
        Cronet_Metrics_push_end_set,
    fn push_end_move(&Self, push_end: DateTime >> DateTime::into_raw);
        Cronet_Metrics_push_end_move,
    fn push_end_get(&Self) -> Option<&mut DateTime> >> DateTime::borrow_from_ptr;
        Cronet_Metrics_push_end_get,

    fn response_start_set(&mut Self, response_start: &DateTime >> DateTime::as_ptr);
        Cronet_Metrics_response_start_set,
    fn response_start_move(&Self, response_start: DateTime >> DateTime::into_raw);
        Cronet_Metrics_response_start_move,
    fn response_start_get(&Self) -> Option<&mut DateTime> >> DateTime::borrow_from_ptr;
        Cronet_Metrics_response_start_get,

    fn request_end_set(&mut Self, request_end: &DateTime >> DateTime::as_ptr);
        Cronet_Metrics_request_end_set,
    fn request_end_move(&Self, request_end: DateTime >> DateTime::into_raw);
        Cronet_Metrics_request_end_move,
    fn request_end_get(&Self) -> Option<&mut DateTime> >> DateTime::borrow_from_ptr;
        Cronet_Metrics_request_end_get,

    fn socket_reused_set(&mut Self, socket_reused: bool);
        Cronet_Metrics_socket_reused_set,
    fn socket_reused_get(&Self) -> bool;
        Cronet_Metrics_socket_reused_get,

    fn sent_byte_count_set(&mut Self, sent_byte_count: i64);
        Cronet_Metrics_sent_byte_count_set,
    fn sent_bytes_count_get(&Self) -> i64;
        Cronet_Metrics_sent_byte_count_get,

    fn received_byte_count_set(&mut Self, received_byte_count: i64);
        Cronet_Metrics_received_byte_count_set,
    fn received_byte_count_get(&Self) -> i64;
        Cronet_Metrics_received_byte_count_get,
}

impl Metrics {
    pub(crate) fn create() -> Self {
        unsafe {
            let ptr = Cronet_Metrics_Create();
            Self { ptr }
        }
    }
}
