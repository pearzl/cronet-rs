use crate::bindings::{
    Cronet_MetricsPtr, Cronet_Metrics_Create, Cronet_Metrics_Destroy, Cronet_Metrics_connect_end_get, Cronet_Metrics_connect_end_move, Cronet_Metrics_connect_end_set, Cronet_Metrics_connect_start_get, Cronet_Metrics_connect_start_move, Cronet_Metrics_connect_start_set, Cronet_Metrics_dns_end_get, Cronet_Metrics_dns_end_move, Cronet_Metrics_dns_end_set, Cronet_Metrics_dns_start_get, Cronet_Metrics_dns_start_move, Cronet_Metrics_dns_start_set, Cronet_Metrics_push_end_get, Cronet_Metrics_push_end_move, Cronet_Metrics_push_end_set, Cronet_Metrics_push_start_get, Cronet_Metrics_push_start_move, Cronet_Metrics_push_start_set, Cronet_Metrics_received_byte_count_get, Cronet_Metrics_received_byte_count_set, Cronet_Metrics_request_end_get, Cronet_Metrics_request_end_move, Cronet_Metrics_request_end_set, Cronet_Metrics_request_start_get, Cronet_Metrics_request_start_move, Cronet_Metrics_request_start_set, Cronet_Metrics_response_start_get, Cronet_Metrics_response_start_move, Cronet_Metrics_response_start_set, Cronet_Metrics_sending_end_get, Cronet_Metrics_sending_end_move, Cronet_Metrics_sending_end_set, Cronet_Metrics_sending_start_get, Cronet_Metrics_sending_start_move, Cronet_Metrics_sending_start_set, Cronet_Metrics_sent_byte_count_get, Cronet_Metrics_sent_byte_count_set, Cronet_Metrics_socket_reused_get, Cronet_Metrics_socket_reused_set, Cronet_Metrics_ssl_end_get, Cronet_Metrics_ssl_end_move, Cronet_Metrics_ssl_end_set, Cronet_Metrics_ssl_start_get, Cronet_Metrics_ssl_start_move, Cronet_Metrics_ssl_start_set
};

use super::date_time::DateTime;

pub struct Metrics {
    pub ptr: Cronet_MetricsPtr,
}

impl Drop for Metrics {
    fn drop(&mut self) {
        unsafe { Cronet_Metrics_Destroy(self.ptr) }
    }
}

impl Metrics {
    pub fn create() -> Self {
        unsafe {
            let ptr = Cronet_Metrics_Create();
            Self { ptr }
        }
    }

    pub fn request_start_set(&self, request_start: DateTime) {
        unsafe {
            Cronet_Metrics_request_start_set(self.ptr, request_start.ptr);
        }
    }

    pub fn request_start_move(&self, request_start: DateTime) {
        unsafe { Cronet_Metrics_request_start_move(self.ptr, request_start.ptr) }
    }

    pub fn dns_start_set(&self, dns_start: DateTime) {
        unsafe { Cronet_Metrics_dns_start_set(self.ptr, dns_start.ptr) }
    }

    pub fn dns_start_move(&self, dns_start: DateTime) {
        unsafe { Cronet_Metrics_dns_start_move(self.ptr, dns_start.ptr) }
    }

    pub fn dns_end_set(&self, dns_end: DateTime) {
        unsafe { Cronet_Metrics_dns_end_set(self.ptr, dns_end.ptr) }
    }

    pub fn dns_end_move(&self, dns_end: DateTime) {
        unsafe { Cronet_Metrics_dns_end_move(self.ptr, dns_end.ptr) }
    }

    pub fn connect_start_set(&self, connect_start: DateTime) {
        unsafe { Cronet_Metrics_connect_start_set(self.ptr, connect_start.ptr) }
    }

    pub fn connect_start_move(&self, connect_start: DateTime) {
        unsafe { Cronet_Metrics_connect_start_move(self.ptr, connect_start.ptr) }
    }

    pub fn connect_end_set(&self, connect_end: DateTime) {
        unsafe { Cronet_Metrics_connect_end_set(self.ptr, connect_end.ptr) }
    }

    pub fn connect_end_move(&self, connect_end: DateTime) {
        unsafe { Cronet_Metrics_connect_end_move(self.ptr, connect_end.ptr) }
    }

    pub fn ssl_start_set(&self, ssl_start: DateTime) {
        unsafe { Cronet_Metrics_ssl_start_set(self.ptr, ssl_start.ptr) }
    }

    pub fn ssl_start_move(&self, ssl_start: DateTime) {
        unsafe { Cronet_Metrics_ssl_start_move(self.ptr, ssl_start.ptr) }
    }

    pub fn ssl_end_set(&self, ssl_end: DateTime) {
        unsafe { Cronet_Metrics_ssl_end_set(self.ptr, ssl_end.ptr) }
    }

    pub fn ssl_end_move(&self, ssl_end: DateTime) {
        unsafe { Cronet_Metrics_ssl_end_move(self.ptr, ssl_end.ptr) }
    }

    pub fn sending_start_set(&self, sending_start: DateTime) {
        unsafe { Cronet_Metrics_sending_start_set(self.ptr, sending_start.ptr) }
    }

    pub fn sending_start_move(&self, sending_start: DateTime) {
        unsafe { Cronet_Metrics_sending_start_move(self.ptr, sending_start.ptr) }
    }

    pub fn sending_end_set(&self, sending_start: DateTime) {
        unsafe { Cronet_Metrics_sending_end_set(self.ptr, sending_start.ptr) }
    }

    pub fn sending_end_move(&self, sending_start: DateTime) {
        unsafe { Cronet_Metrics_sending_end_move(self.ptr, sending_start.ptr) }
    }


    pub fn push_start_set(&self, push_start: DateTime) {
        unsafe { Cronet_Metrics_push_start_set(self.ptr, push_start.ptr) }
    }

    pub fn push_start_moe(&self, push_start: DateTime) {
        unsafe { Cronet_Metrics_push_start_move(self.ptr, push_start.ptr) }
    }

    pub fn push_end_set(&self, push_end: DateTime) {
        unsafe { Cronet_Metrics_push_end_set(self.ptr, push_end.ptr) }
    }

    pub fn push_end_move(&self, push_end: DateTime) {
        unsafe { Cronet_Metrics_push_end_move(self.ptr, push_end.ptr) }
    }

    pub fn response_start_set(&self, response_start: DateTime) {
        unsafe { Cronet_Metrics_response_start_set(self.ptr, response_start.ptr) }
    }

    pub fn response_start_move(&self, response_start: DateTime) {
        unsafe { Cronet_Metrics_response_start_move(self.ptr, response_start.ptr) }
    }

    pub fn request_end_set(&self, request_end: DateTime) {
        unsafe { Cronet_Metrics_request_end_set(self.ptr, request_end.ptr) }
    }

    pub fn request_end_move(&self, request_end: DateTime) {
        unsafe { Cronet_Metrics_request_end_move(self.ptr, request_end.ptr) }
    }

    pub fn socket_reused_set(&self, socket_reused: bool) {
        unsafe { Cronet_Metrics_socket_reused_set(self.ptr, socket_reused) }
    }

    pub fn sent_byte_count_set(&self, sent_byte_count: i64) {
        unsafe { Cronet_Metrics_sent_byte_count_set(self.ptr, sent_byte_count) }
    }

    pub fn received_byte_count_set(&self, received_byte_count: i64) {
        unsafe { Cronet_Metrics_received_byte_count_set(self.ptr, received_byte_count) }
    }

    pub fn request_start_get(&self) -> DateTime {
        unsafe {
            let ptr = Cronet_Metrics_request_start_get(self.ptr);
            DateTime { ptr }
        }
    }

    pub fn dns_start_get(&self) -> DateTime {
        unsafe {
            let ptr = Cronet_Metrics_dns_start_get(self.ptr);
            DateTime { ptr }
        }
    }

    pub fn dns_end_get(&self) -> DateTime {
        unsafe {
            let ptr = Cronet_Metrics_dns_end_get(self.ptr);
            DateTime { ptr }
        }
    }

    pub fn connect_start_get(&self) -> DateTime {
        unsafe {
            let ptr = Cronet_Metrics_connect_start_get(self.ptr);
            DateTime { ptr }
        }
    }

    pub fn connect_end_get(&self) -> DateTime {
        unsafe {
            let ptr = Cronet_Metrics_connect_end_get(self.ptr);
            DateTime { ptr }
        }
    }

    pub fn ssl_start_get(&self) -> DateTime {
        unsafe {
            let ptr = Cronet_Metrics_ssl_start_get(self.ptr);
            DateTime { ptr }
        }
    }

    pub fn ssl_end_get(&self) -> DateTime {
        unsafe {
            let ptr = Cronet_Metrics_ssl_end_get(self.ptr);
            DateTime { ptr }
        }
    }

    pub fn sending_start_get(&self) -> DateTime {
        unsafe {
            let ptr = Cronet_Metrics_sending_start_get(self.ptr);
            DateTime { ptr }
        }
    }

    pub fn sending_end_get(&self) -> DateTime {
        unsafe {
            let ptr = Cronet_Metrics_sending_end_get(self.ptr);
            DateTime { ptr }
        }
    }

    pub fn push_start_get(&self) -> DateTime {
        unsafe {
            let ptr = Cronet_Metrics_push_start_get(self.ptr);
            DateTime { ptr }
        }
    }

    pub fn push_end_get(&self) -> DateTime {
        unsafe {
            let ptr = Cronet_Metrics_push_end_get(self.ptr);
            DateTime { ptr }
        }
    }

    pub fn response_start_get(&self) -> DateTime {
        unsafe {
            let ptr = Cronet_Metrics_response_start_get(self.ptr);
            DateTime { ptr }
        }
    }

    pub fn request_end_get(&self) -> DateTime {
        unsafe {
            let ptr = Cronet_Metrics_request_end_get(self.ptr);
            DateTime { ptr }
        }
    }

    pub fn socket_reused_get(&self) -> bool {
        unsafe { Cronet_Metrics_socket_reused_get(self.ptr) }
    }

    pub fn sent_bytes_count_get(&self) -> i64 {
        unsafe { Cronet_Metrics_sent_byte_count_get(self.ptr) }
    }

    pub fn received_byte_count_get(&self) -> i64 {
        unsafe { Cronet_Metrics_received_byte_count_get(self.ptr) }
    }
}
