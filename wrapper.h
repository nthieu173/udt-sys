#ifndef __UDT_WRAP_H__
#define __UDT_WRAP_H__

#include <udt.h>

typedef UDTOpt SOCKOPT;
typedef CPerfMon TRACEINFO;

extern "C"
{
  extern const int UDT_SUCCESS = UDT::ERRORINFO::SUCCESS;
  extern const int UDT_ECONNSETUP = UDT::ERRORINFO::ECONNSETUP;
  extern const int UDT_ENOSERVER = UDT::ERRORINFO::ENOSERVER;
  extern const int UDT_ECONNREJ = UDT::ERRORINFO::ECONNREJ;
  extern const int UDT_ESOCKFAIL = UDT::ERRORINFO::ESOCKFAIL;
  extern const int UDT_ESECFAIL = UDT::ERRORINFO::ESECFAIL;
  extern const int UDT_ECONNFAIL = UDT::ERRORINFO::ECONNFAIL;
  extern const int UDT_ECONNLOST = UDT::ERRORINFO::ECONNLOST;
  extern const int UDT_ENOCONN = UDT::ERRORINFO::ENOCONN;
  extern const int UDT_ERESOURCE = UDT::ERRORINFO::ERESOURCE;
  extern const int UDT_ETHREAD = UDT::ERRORINFO::ETHREAD;
  extern const int UDT_ENOBUF = UDT::ERRORINFO::ENOBUF;
  extern const int UDT_EFILE = UDT::ERRORINFO::EFILE;
  extern const int UDT_EINVRDOFF = UDT::ERRORINFO::EINVRDOFF;
  extern const int UDT_ERDPERM = UDT::ERRORINFO::ERDPERM;
  extern const int UDT_EINVWROFF = UDT::ERRORINFO::EINVWROFF;
  extern const int UDT_EWRPERM = UDT::ERRORINFO::EWRPERM;
  extern const int UDT_EINVOP = UDT::ERRORINFO::EINVOP;
  extern const int UDT_EBOUNDSOCK = UDT::ERRORINFO::EBOUNDSOCK;
  extern const int UDT_ECONNSOCK = UDT::ERRORINFO::ECONNSOCK;
  extern const int UDT_EINVPARAM = UDT::ERRORINFO::EINVPARAM;
  extern const int UDT_EINVSOCK = UDT::ERRORINFO::EINVSOCK;
  extern const int UDT_EUNBOUNDSOCK = UDT::ERRORINFO::EUNBOUNDSOCK;
  extern const int UDT_ENOLISTEN = UDT::ERRORINFO::ENOLISTEN;
  extern const int UDT_ERDVNOSERV = UDT::ERRORINFO::ERDVNOSERV;
  extern const int UDT_ERDVUNBOUND = UDT::ERRORINFO::ERDVUNBOUND;
  extern const int UDT_ESTREAMILL = UDT::ERRORINFO::ESTREAMILL;
  extern const int UDT_EDGRAMILL = UDT::ERRORINFO::EDGRAMILL;
  extern const int UDT_EDUPLISTEN = UDT::ERRORINFO::EDUPLISTEN;
  extern const int UDT_ELARGEMSG = UDT::ERRORINFO::ELARGEMSG;
  extern const int UDT_EINVPOLLID = UDT::ERRORINFO::EINVPOLLID;
  extern const int UDT_EASYNCFAIL = UDT::ERRORINFO::EASYNCFAIL;
  extern const int UDT_EASYNCSND = UDT::ERRORINFO::EASYNCSND;
  extern const int UDT_EASYNCRCV = UDT::ERRORINFO::EASYNCRCV;
  extern const int UDT_ETIMEOUT = UDT::ERRORINFO::ETIMEOUT;
  extern const int UDT_EPEERERR = UDT::ERRORINFO::EPEERERR;
  extern const int UDT_EUNKNOWN = UDT::ERRORINFO::EUNKNOWN;

  extern const UDTSOCKET UDT_INVALID_SOCK = UDT::INVALID_SOCK;
  extern const int UDT_ERROR = UDT::ERROR;

  int udt_startup();
  int udt_cleanup();
  UDTSOCKET udt_socket(int af, int type, int protocol);
  int udt_bind(UDTSOCKET u, const struct sockaddr *name, int namelen);
  int udt_bind2(UDTSOCKET u, UDPSOCKET udpsock);
  int udt_listen(UDTSOCKET u, int backlog);
  UDTSOCKET udt_accept(UDTSOCKET u, struct sockaddr *addr, int *addrlen);
  int udt_connect(UDTSOCKET u, const struct sockaddr *name, int namelen);
  int udt_close(UDTSOCKET u);
  int udt_getpeername(UDTSOCKET u, struct sockaddr *name, int *namelen);
  int udt_getsockname(UDTSOCKET u, struct sockaddr *name, int *namelen);
  int udt_getsockopt(UDTSOCKET u, int level, SOCKOPT optname, void *optval, int *optlen);
  int udt_setsockopt(UDTSOCKET u, int level, SOCKOPT optname, const void *optval, int optlen);
  int udt_send(UDTSOCKET u, const char *buf, int len, int flags);
  int udt_recv(UDTSOCKET u, char *buf, int len, int flags);
  int udt_sendmsg(UDTSOCKET u, const char *buf, int len, int ttl, bool inorder);
  int udt_recvmsg(UDTSOCKET u, char *buf, int len);
  int64_t udt_sendfile2(UDTSOCKET u, const char *path, int64_t *offset, int64_t size, int block);
  int64_t udt_recvfile2(UDTSOCKET u, const char *path, int64_t *offset, int64_t size, int block);
  int udt_epoll_create();
  int udt_epoll_add_usock(int eid, UDTSOCKET u, const int *events);
  int udt_epoll_add_ssock(int eid, SYSSOCKET s, const int *events);
  int udt_epoll_remove_usock(int eid, UDTSOCKET u);
  int udt_epoll_remove_ssock(int eid, SYSSOCKET s);
  int udt_epoll_wait2(int eid, UDTSOCKET *readfds, int *rnum, UDTSOCKET *writefds, int *wnum,
                      int64_t msTimeOut, SYSSOCKET *lrfds, int *lrnum, SYSSOCKET *lwfds, int *lwnum);
  int udt_epoll_release(int eid);
  int udt_getlasterror_code();
  const char *udt_getlasterror_desc();
  int udt_perfmon(UDTSOCKET u, TRACEINFO *perf, bool clear);
  UDTSTATUS udt_getsockstate(UDTSOCKET u);
}

#endif