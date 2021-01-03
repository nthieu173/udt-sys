#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(target_family = "unix")]
pub use libc::sockaddr;

#[cfg(target_family = "windows")]
pub use winapi::shared::ws2def::SOCKADDR as sockaddr;

use std::{
    ops,
    os::raw::{c_int, c_ushort},
};

pub type sa_family_t = c_ushort;

pub type SYSSOCKET = c_int;
pub type UDPSOCKET = SYSSOCKET;
pub type UDTSOCKET = c_int;

impl EPOLLOpt {
    pub const UDT_EPOLL_IN: EPOLLOpt = EPOLLOpt(1);
}
impl EPOLLOpt {
    pub const UDT_EPOLL_OUT: EPOLLOpt = EPOLLOpt(4);
}
impl EPOLLOpt {
    pub const UDT_EPOLL_ERR: EPOLLOpt = EPOLLOpt(8);
}

impl ops::BitOr<EPOLLOpt> for EPOLLOpt {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        EPOLLOpt(self.0 | other.0)
    }
}
impl ops::BitOrAssign for EPOLLOpt {
    #[inline]
    fn bitor_assign(&mut self, rhs: EPOLLOpt) {
        self.0 |= rhs.0;
    }
}
impl ops::BitAnd<EPOLLOpt> for EPOLLOpt {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        EPOLLOpt(self.0 & other.0)
    }
}
impl ops::BitAndAssign for EPOLLOpt {
    #[inline]
    fn bitand_assign(&mut self, rhs: EPOLLOpt) {
        self.0 &= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct EPOLLOpt(pub u32);
impl UDTSTATUS {
    pub const INIT: UDTSTATUS = UDTSTATUS(1);
}
impl UDTSTATUS {
    pub const OPENED: UDTSTATUS = UDTSTATUS(2);
}
impl UDTSTATUS {
    pub const LISTENING: UDTSTATUS = UDTSTATUS(3);
}
impl UDTSTATUS {
    pub const CONNECTING: UDTSTATUS = UDTSTATUS(4);
}
impl UDTSTATUS {
    pub const CONNECTED: UDTSTATUS = UDTSTATUS(5);
}
impl UDTSTATUS {
    pub const BROKEN: UDTSTATUS = UDTSTATUS(6);
}
impl UDTSTATUS {
    pub const CLOSING: UDTSTATUS = UDTSTATUS(7);
}
impl UDTSTATUS {
    pub const CLOSED: UDTSTATUS = UDTSTATUS(8);
}
impl UDTSTATUS {
    pub const NONEXIST: UDTSTATUS = UDTSTATUS(9);
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct UDTSTATUS(pub u32);
impl UDTOpt {
    pub const UDT_MSS: UDTOpt = UDTOpt(0);
}
impl UDTOpt {
    pub const UDT_SNDSYN: UDTOpt = UDTOpt(1);
}
impl UDTOpt {
    pub const UDT_RCVSYN: UDTOpt = UDTOpt(2);
}
impl UDTOpt {
    pub const UDT_CC: UDTOpt = UDTOpt(3);
}
impl UDTOpt {
    pub const UDT_FC: UDTOpt = UDTOpt(4);
}
impl UDTOpt {
    pub const UDT_SNDBUF: UDTOpt = UDTOpt(5);
}
impl UDTOpt {
    pub const UDT_RCVBUF: UDTOpt = UDTOpt(6);
}
impl UDTOpt {
    pub const UDT_LINGER: UDTOpt = UDTOpt(7);
}
impl UDTOpt {
    pub const UDP_SNDBUF: UDTOpt = UDTOpt(8);
}
impl UDTOpt {
    pub const UDP_RCVBUF: UDTOpt = UDTOpt(9);
}
impl UDTOpt {
    pub const UDT_MAXMSG: UDTOpt = UDTOpt(10);
}
impl UDTOpt {
    pub const UDT_MSGTTL: UDTOpt = UDTOpt(11);
}
impl UDTOpt {
    pub const UDT_RENDEZVOUS: UDTOpt = UDTOpt(12);
}
impl UDTOpt {
    pub const UDT_SNDTIMEO: UDTOpt = UDTOpt(13);
}
impl UDTOpt {
    pub const UDT_RCVTIMEO: UDTOpt = UDTOpt(14);
}
impl UDTOpt {
    pub const UDT_REUSEADDR: UDTOpt = UDTOpt(15);
}
impl UDTOpt {
    pub const UDT_MAXBW: UDTOpt = UDTOpt(16);
}
impl UDTOpt {
    pub const UDT_STATE: UDTOpt = UDTOpt(17);
}
impl UDTOpt {
    pub const UDT_EVENT: UDTOpt = UDTOpt(18);
}
impl UDTOpt {
    pub const UDT_SNDDATA: UDTOpt = UDTOpt(19);
}
impl UDTOpt {
    pub const UDT_RCVDATA: UDTOpt = UDTOpt(20);
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct UDTOpt(pub u32);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CPerfMon {
    pub msTimeStamp: i64,
    pub pktSentTotal: i64,
    pub pktRecvTotal: i64,
    pub pktSndLossTotal: c_int,
    pub pktRcvLossTotal: c_int,
    pub pktRetransTotal: c_int,
    pub pktSentACKTotal: c_int,
    pub pktRecvACKTotal: c_int,
    pub pktSentNAKTotal: c_int,
    pub pktRecvNAKTotal: c_int,
    pub usSndDurationTotal: i64,
    pub pktSent: i64,
    pub pktRecv: i64,
    pub pktSndLoss: c_int,
    pub pktRcvLoss: c_int,
    pub pktRetrans: c_int,
    pub pktSentACK: c_int,
    pub pktRecvACK: c_int,
    pub pktSentNAK: c_int,
    pub pktRecvNAK: c_int,
    pub mbpsSendRate: f64,
    pub mbpsRecvRate: f64,
    pub usSndDuration: i64,
    pub usPktSndPeriod: f64,
    pub pktFlowWindow: c_int,
    pub pktCongestionWindow: c_int,
    pub pktFlightSize: c_int,
    pub msRTT: f64,
    pub mbpsBandwidth: f64,
    pub byteAvailSndBuf: c_int,
    pub byteAvailRcvBuf: c_int,
}

pub use self::UDTOpt as SOCKOPT;
pub type TRACEINFO = CPerfMon;
extern "C" {
    pub static UDT_SUCCESS: c_int;
    pub static UDT_ECONNSETUP: c_int;
    pub static UDT_ENOSERVER: c_int;
    pub static UDT_ECONNREJ: c_int;
    pub static UDT_ESOCKFAIL: c_int;
    pub static UDT_ESECFAIL: c_int;
    pub static UDT_ECONNFAIL: c_int;
    pub static UDT_ECONNLOST: c_int;
    pub static UDT_ENOCONN: c_int;
    pub static UDT_ERESOURCE: c_int;
    pub static UDT_ETHREAD: c_int;
    pub static UDT_ENOBUF: c_int;
    pub static UDT_EFILE: c_int;
    pub static UDT_EINVRDOFF: c_int;
    pub static UDT_ERDPERM: c_int;
    pub static UDT_EINVWROFF: c_int;
    pub static UDT_EWRPERM: c_int;
    pub static UDT_EINVOP: c_int;
    pub static UDT_EBOUNDSOCK: c_int;
    pub static UDT_ECONNSOCK: c_int;
    pub static UDT_EINVPARAM: c_int;
    pub static UDT_EINVSOCK: c_int;
    pub static UDT_EUNBOUNDSOCK: c_int;
    pub static UDT_ENOLISTEN: c_int;
    pub static UDT_ERDVNOSERV: c_int;
    pub static UDT_ERDVUNBOUND: c_int;
    pub static UDT_ESTREAMILL: c_int;
    pub static UDT_EDGRAMILL: c_int;
    pub static UDT_EDUPLISTEN: c_int;
    pub static UDT_ELARGEMSG: c_int;
    pub static UDT_EINVPOLLID: c_int;
    pub static UDT_EASYNCFAIL: c_int;
    pub static UDT_EASYNCSND: c_int;
    pub static UDT_EASYNCRCV: c_int;
    pub static UDT_ETIMEOUT: c_int;
    pub static UDT_EPEERERR: c_int;
    pub static UDT_EUNKNOWN: c_int;
    pub static UDT_INVALID_SOCK: UDTSOCKET;
    pub static UDT_ERROR: c_int;
    pub fn udt_startup() -> c_int;
    pub fn udt_cleanup() -> c_int;
    pub fn udt_socket(af: c_int, type_: c_int, protocol: c_int) -> UDTSOCKET;
    pub fn udt_bind(u: UDTSOCKET, name: *const sockaddr, namelen: c_int) -> c_int;
    pub fn udt_bind2(u: UDTSOCKET, udpsock: UDPSOCKET) -> c_int;
    pub fn udt_listen(u: UDTSOCKET, backlog: c_int) -> c_int;
    pub fn udt_accept(u: UDTSOCKET, addr: *mut sockaddr, addrlen: *mut c_int) -> UDTSOCKET;
    pub fn udt_connect(u: UDTSOCKET, name: *const sockaddr, namelen: c_int) -> c_int;
    pub fn udt_close(u: UDTSOCKET) -> c_int;
    pub fn udt_getpeername(u: UDTSOCKET, name: *mut sockaddr, namelen: *mut c_int) -> c_int;
    pub fn udt_getsockname(u: UDTSOCKET, name: *mut sockaddr, namelen: *mut c_int) -> c_int;
    pub fn udt_getsockopt(
        u: UDTSOCKET,
        level: c_int,
        optname: SOCKOPT,
        optval: *mut ::std::os::raw::c_void,
        optlen: *mut c_int,
    ) -> c_int;
    pub fn udt_setsockopt(
        u: UDTSOCKET,
        level: c_int,
        optname: SOCKOPT,
        optval: *const ::std::os::raw::c_void,
        optlen: c_int,
    ) -> c_int;
    pub fn udt_send(
        u: UDTSOCKET,
        buf: *const ::std::os::raw::c_char,
        len: c_int,
        flags: c_int,
    ) -> c_int;
    pub fn udt_recv(
        u: UDTSOCKET,
        buf: *mut ::std::os::raw::c_char,
        len: c_int,
        flags: c_int,
    ) -> c_int;
    pub fn udt_sendmsg(
        u: UDTSOCKET,
        buf: *const ::std::os::raw::c_char,
        len: c_int,
        ttl: c_int,
        inorder: bool,
    ) -> c_int;
    pub fn udt_recvmsg(u: UDTSOCKET, buf: *mut ::std::os::raw::c_char, len: c_int) -> c_int;
    pub fn udt_sendfile2(
        u: UDTSOCKET,
        path: *const ::std::os::raw::c_char,
        offset: *mut i64,
        size: i64,
        block: c_int,
    ) -> i64;
    pub fn udt_recvfile2(
        u: UDTSOCKET,
        path: *const ::std::os::raw::c_char,
        offset: *mut i64,
        size: i64,
        block: c_int,
    ) -> i64;
    pub fn udt_epoll_create() -> c_int;
    pub fn udt_epoll_add_usock(eid: c_int, u: UDTSOCKET, events: *const c_int) -> c_int;
    pub fn udt_epoll_add_ssock(eid: c_int, s: SYSSOCKET, events: *const c_int) -> c_int;
    pub fn udt_epoll_remove_usock(eid: c_int, u: UDTSOCKET) -> c_int;
    pub fn udt_epoll_remove_ssock(eid: c_int, s: SYSSOCKET) -> c_int;
    pub fn udt_epoll_wait2(
        eid: c_int,
        readfds: *mut UDTSOCKET,
        rnum: *mut c_int,
        writefds: *mut UDTSOCKET,
        wnum: *mut c_int,
        msTimeOut: i64,
        lrfds: *mut SYSSOCKET,
        lrnum: *mut c_int,
        lwfds: *mut SYSSOCKET,
        lwnum: *mut c_int,
    ) -> c_int;
    pub fn udt_epoll_release(eid: c_int) -> c_int;
    pub fn udt_getlasterror_code() -> c_int;
    pub fn udt_getlasterror_desc() -> *const ::std::os::raw::c_char;
    pub fn udt_perfmon(u: UDTSOCKET, perf: *mut TRACEINFO, clear: bool) -> c_int;
    pub fn udt_getsockstate(u: UDTSOCKET) -> UDTSTATUS;
}

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
            let result = crate::udt_bind(
                sock,
                &test_addr as *const crate::sockaddr,
                std::mem::size_of_val(&test_addr) as i32,
            );
            assert_ne!(result, crate::UDT_ERROR);
            let result = crate::udt_close(sock);
            assert_ne!(result, crate::UDT_ERROR);
            crate::udt_cleanup();
        };
    }
}
