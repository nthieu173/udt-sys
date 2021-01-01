#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod test {
    #[test]
    fn test_startup_cleanup() {
        unsafe {
            crate::udt_startup();
            crate::udt_cleanup();
        }
    }
    #[test]
    fn test_socket_bind() {
        unsafe {
            let AF_INET = 2;
            let SOCK_STREAM = 1;
            crate::udt_startup();
            let sock = crate::udt_socket(AF_INET, SOCK_STREAM, 0);
            assert_ne!(sock, crate::UDT_INVALID_SOCK);
            let test_addr = crate::sockaddr {
                sa_family: AF_INET as u16,
                sa_data: [0; 14],
            };
            crate::udt_bind(
                sock,
                &test_addr as *const crate::sockaddr,
                std::mem::size_of_val(&test_addr) as i32,
            );
            assert_ne!(sock, crate::UDT_ERROR);
            crate::udt_cleanup();
        };
    }
}
