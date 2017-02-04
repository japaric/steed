// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(issue = "22")]
use cmp;
#[cfg(issue = "22")]
use ffi::CString;
use fmt;
use io::{self, Error, ErrorKind};
use libc::{self, c_int, size_t};
use mem;
use net::{SocketAddr, Shutdown, Ipv4Addr, Ipv6Addr};
#[cfg(issue = "22")]
use ptr;
use sys::{cvt, cvt_r};
use sys::net::{Socket, init};
#[cfg(issue = "22")]
use sys::net::cvt_gai;
use sys_common::{AsInner, FromInner, IntoInner};
use time::Duration;

#[allow(non_camel_case_types)]
struct sockaddr_storage {
    ss_family: libc::sa_family_t,
    __ss_align: size_t,
    #[cfg(target_pointer_width = "32")]
    __ss_pad2: [u8; 128 - 2 * 4],
    #[cfg(target_pointer_width = "64")]
    __ss_pad2: [u8; 128 - 2 * 8],
}

#[cfg(any(target_os = "dragonfly", target_os = "freebsd",
          target_os = "ios", target_os = "macos",
          target_os = "openbsd", target_os = "netbsd",
          target_os = "solaris", target_os = "haiku"))]
use sys::net::netc::IPV6_JOIN_GROUP as IPV6_ADD_MEMBERSHIP;

#[cfg(any(target_os = "dragonfly", target_os = "freebsd",
          target_os = "ios", target_os = "macos",
          target_os = "openbsd", target_os = "netbsd",
          target_os = "solaris", target_os = "haiku"))]
use sys::net::netc::IPV6_LEAVE_GROUP as IPV6_DROP_MEMBERSHIP;

#[cfg(not(any(target_os = "linux", target_os = "android",
              target_os = "dragonfly", target_os = "freebsd",
              target_os = "openbsd", target_os = "netbsd",
              target_os = "haiku", target_os = "bitrig")))]
const MSG_NOSIGNAL: c_int = 0x0;

////////////////////////////////////////////////////////////////////////////////
// sockaddr and misc bindings
////////////////////////////////////////////////////////////////////////////////

pub fn setsockopt<T>(sock: &Socket, opt: c_int, val: c_int,
                     payload: T) -> io::Result<()> {
    unsafe {
        let payload = &payload as *const T as *const _;
        cvt(libc::setsockopt(*sock.as_inner(), opt, val, payload,
                          mem::size_of::<T>() as libc::socklen_t))?;
        Ok(())
    }
}

pub fn getsockopt<T: Copy>(sock: &Socket, opt: c_int,
                       val: c_int) -> io::Result<T> {
    unsafe {
        let mut slot: T = mem::zeroed();
        let mut len = mem::size_of::<T>() as libc::socklen_t;
        cvt(libc::getsockopt(*sock.as_inner(), opt, val,
                          &mut slot as *mut _ as *mut _,
                          &mut len))?;
        assert_eq!(len as usize, mem::size_of::<T>());
        Ok(slot)
    }
}

fn sockname<F>(f: F) -> io::Result<SocketAddr>
    where F: FnOnce(*mut libc::sockaddr, *mut libc::socklen_t) -> isize
{
    unsafe {
        let mut storage: sockaddr_storage = mem::zeroed();
        let mut len = mem::size_of_val(&storage) as libc::socklen_t;
        cvt(f(&mut storage as *mut _ as *mut _, &mut len))?;
        sockaddr_to_addr(&storage, len as usize)
    }
}

fn sockaddr_to_addr(storage: &sockaddr_storage,
                    len: usize) -> io::Result<SocketAddr> {
    match storage.ss_family as c_int {
        libc::AF_INET => {
            assert!(len as usize >= mem::size_of::<libc::sockaddr_in>());
            Ok(SocketAddr::V4(FromInner::from_inner(unsafe {
                *(storage as *const _ as *const libc::sockaddr_in)
            })))
        }
        libc::AF_INET6 => {
            assert!(len as usize >= mem::size_of::<libc::sockaddr_in6>());
            Ok(SocketAddr::V6(FromInner::from_inner(unsafe {
                *(storage as *const _ as *const libc::sockaddr_in6)
            })))
        }
        _ => {
            Err(Error::new(ErrorKind::InvalidInput, "invalid argument"))
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// get_host_addresses
////////////////////////////////////////////////////////////////////////////////

#[cfg(issue = "22")]
pub struct LookupHost {
    original: *mut libc::addrinfo,
    cur: *mut libc::addrinfo,
}

#[cfg(issue = "22")]
impl Iterator for LookupHost {
    type Item = SocketAddr;
    fn next(&mut self) -> Option<SocketAddr> {
        loop {
            unsafe {
                let cur = match self.cur.as_ref() {
                    None => return None,
                    Some(c) => c,
                };
                self.cur = cur.ai_next;
                match sockaddr_to_addr(mem::transmute(cur.ai_addr),
                                       cur.ai_addrlen as usize)
                {
                    Ok(addr) => return Some(addr),
                    Err(_) => continue,
                }
            }
        }
    }
}

#[cfg(issue = "22")] unsafe impl Sync for LookupHost {}
#[cfg(issue = "22")] unsafe impl Send for LookupHost {}

#[cfg(issue = "22")]
impl Drop for LookupHost {
    fn drop(&mut self) {
        unsafe { libc::freeaddrinfo(self.original) }
    }
}

#[cfg(issue = "22")]
pub fn lookup_host(_host: &str) -> io::Result<LookupHost> {
    init();

    let c_host = CString::new(host)?;
    let hints = libc::addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: libc::SOCK_STREAM,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: ptr::null_mut(),
        ai_canonname: ptr::null_mut(),
        ai_next: ptr::null_mut()
    };
    let mut res = ptr::null_mut();
    unsafe {
        cvt_gai(libc::getaddrinfo(c_host.as_ptr(), ptr::null(), &hints,
                               &mut res))?;
        Ok(LookupHost { original: res, cur: res })
    }
}

////////////////////////////////////////////////////////////////////////////////
// TCP streams
////////////////////////////////////////////////////////////////////////////////

pub struct TcpStream {
    inner: Socket,
}

impl TcpStream {
    pub fn connect(addr: &SocketAddr) -> io::Result<TcpStream> {
        init();

        let sock = Socket::new(addr, libc::SOCK_STREAM)?;

        let (addrp, len) = addr.into_inner();
        cvt_r(|| unsafe { libc::connect(*sock.as_inner(), addrp, len) })?;
        Ok(TcpStream { inner: sock })
    }

    #[cfg_attr(not(issue = "22"), allow(dead_code))]
    pub fn socket(&self) -> &Socket { &self.inner }

    #[cfg_attr(not(issue = "22"), allow(dead_code))]
    pub fn into_socket(self) -> Socket { self.inner }

    pub fn set_read_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.inner.set_timeout(dur, libc::SO_RCVTIMEO)
    }

    pub fn set_write_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.inner.set_timeout(dur, libc::SO_SNDTIMEO)
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        self.inner.timeout(libc::SO_RCVTIMEO)
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        self.inner.timeout(libc::SO_SNDTIMEO)
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }

    pub fn read_to_end(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.inner.read_to_end(buf)
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        let ret = cvt(unsafe {
            libc::send(*self.inner.as_inner(),
                    buf.as_ptr() as *const _,
                    buf.len(),
                    libc::MSG_NOSIGNAL)
        })?;
        Ok(ret as usize)
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        sockname(|buf, len| unsafe {
            libc::getpeername(*self.inner.as_inner(), buf, len)
        })
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        sockname(|buf, len| unsafe {
            libc::getsockname(*self.inner.as_inner(), buf, len)
        })
    }

    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        self.inner.shutdown(how)
    }

    pub fn duplicate(&self) -> io::Result<TcpStream> {
        self.inner.duplicate().map(|s| TcpStream { inner: s })
    }

    pub fn set_nodelay(&self, nodelay: bool) -> io::Result<()> {
        self.inner.set_nodelay(nodelay)
    }

    pub fn nodelay(&self) -> io::Result<bool> {
        self.inner.nodelay()
    }

    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        setsockopt(&self.inner, libc::IPPROTO_IP, libc::IP_TTL, ttl as c_int)
    }

    pub fn ttl(&self) -> io::Result<u32> {
        let raw: c_int = getsockopt(&self.inner, libc::IPPROTO_IP, libc::IP_TTL)?;
        Ok(raw as u32)
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.inner.take_error()
    }

    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.inner.set_nonblocking(nonblocking)
    }
}

impl FromInner<Socket> for TcpStream {
    fn from_inner(socket: Socket) -> TcpStream {
        TcpStream { inner: socket }
    }
}

impl fmt::Debug for TcpStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = f.debug_struct("TcpStream");

        if let Ok(addr) = self.socket_addr() {
            res.field("addr", &addr);
        }

        if let Ok(peer) = self.peer_addr() {
            res.field("peer", &peer);
        }

        let name = if cfg!(windows) {"socket"} else {"fd"};
        res.field(name, &self.inner.as_inner())
            .finish()
    }
}

////////////////////////////////////////////////////////////////////////////////
// TCP listeners
////////////////////////////////////////////////////////////////////////////////

pub struct TcpListener {
    inner: Socket,
}

impl TcpListener {
    pub fn bind(addr: &SocketAddr) -> io::Result<TcpListener> {
        init();

        let sock = Socket::new(addr, libc::SOCK_STREAM)?;

        // On platforms with Berkeley-derived sockets, this allows
        // to quickly rebind a socket, without needing to wait for
        // the OS to clean up the previous one.
        if !cfg!(windows) {
            setsockopt(&sock, libc::SOL_SOCKET, libc::SO_REUSEADDR,
                       1 as c_int)?;
        }

        // Bind our new socket
        let (addrp, len) = addr.into_inner();
        cvt(unsafe { libc::bind(*sock.as_inner(), addrp, len) })?;

        // Start listening
        cvt(unsafe { libc::listen(*sock.as_inner(), 128) })?;
        Ok(TcpListener { inner: sock })
    }

    #[cfg_attr(not(issue = "22"), allow(dead_code))]
    pub fn socket(&self) -> &Socket { &self.inner }

    #[cfg_attr(not(issue = "22"), allow(dead_code))]
    pub fn into_socket(self) -> Socket { self.inner }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        sockname(|buf, len| unsafe {
            libc::getsockname(*self.inner.as_inner(), buf, len)
        })
    }

    pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        let mut storage: sockaddr_storage = unsafe { mem::zeroed() };
        let mut len = mem::size_of_val(&storage) as libc::socklen_t;
        let sock = self.inner.accept(&mut storage as *mut _ as *mut _,
                                     &mut len)?;
        let addr = sockaddr_to_addr(&storage, len as usize)?;
        Ok((TcpStream { inner: sock, }, addr))
    }

    pub fn duplicate(&self) -> io::Result<TcpListener> {
        self.inner.duplicate().map(|s| TcpListener { inner: s })
    }

    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        setsockopt(&self.inner, libc::IPPROTO_IP, libc::IP_TTL, ttl as c_int)
    }

    pub fn ttl(&self) -> io::Result<u32> {
        let raw: c_int = getsockopt(&self.inner, libc::IPPROTO_IP, libc::IP_TTL)?;
        Ok(raw as u32)
    }

    pub fn set_only_v6(&self, only_v6: bool) -> io::Result<()> {
        setsockopt(&self.inner, libc::IPPROTO_IPV6, libc::IPV6_V6ONLY, only_v6 as c_int)
    }

    pub fn only_v6(&self) -> io::Result<bool> {
        let raw: c_int = getsockopt(&self.inner, libc::IPPROTO_IPV6, libc::IPV6_V6ONLY)?;
        Ok(raw != 0)
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.inner.take_error()
    }

    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.inner.set_nonblocking(nonblocking)
    }
}

impl FromInner<Socket> for TcpListener {
    fn from_inner(socket: Socket) -> TcpListener {
        TcpListener { inner: socket }
    }
}

impl fmt::Debug for TcpListener {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = f.debug_struct("TcpListener");

        if let Ok(addr) = self.socket_addr() {
            res.field("addr", &addr);
        }

        let name = if cfg!(windows) {"socket"} else {"fd"};
        res.field(name, &self.inner.as_inner())
            .finish()
    }
}

////////////////////////////////////////////////////////////////////////////////
// UDP
////////////////////////////////////////////////////////////////////////////////

pub struct UdpSocket {
    inner: Socket,
}

impl UdpSocket {
    pub fn bind(addr: &SocketAddr) -> io::Result<UdpSocket> {
        init();

        let sock = Socket::new(addr, libc::SOCK_DGRAM)?;
        let (addrp, len) = addr.into_inner();
        cvt(unsafe { libc::bind(*sock.as_inner(), addrp, len) })?;
        Ok(UdpSocket { inner: sock })
    }

    #[cfg_attr(not(issue = "22"), allow(dead_code))]
    pub fn socket(&self) -> &Socket { &self.inner }

    #[cfg_attr(not(issue = "22"), allow(dead_code))]
    pub fn into_socket(self) -> Socket { self.inner }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        sockname(|buf, len| unsafe {
            libc::getsockname(*self.inner.as_inner(), buf, len)
        })
    }

    pub fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        let mut storage: sockaddr_storage = unsafe { mem::zeroed() };
        let mut addrlen = mem::size_of_val(&storage) as libc::socklen_t;

        let n = cvt(unsafe {
            libc::recvfrom(*self.inner.as_inner(),
                        buf.as_mut_ptr() as *mut _,
                        buf.len(), 0,
                        &mut storage as *mut _ as *mut _, &mut addrlen)
        })?;
        Ok((n as usize, sockaddr_to_addr(&storage, addrlen as usize)?))
    }

    pub fn send_to(&self, buf: &[u8], dst: &SocketAddr) -> io::Result<usize> {
        let (dstp, dstlen) = dst.into_inner();
        let ret = cvt(unsafe {
            libc::sendto(*self.inner.as_inner(),
                      buf.as_ptr() as *const _, buf.len(),
                      libc::MSG_NOSIGNAL, dstp, dstlen)
        })?;
        Ok(ret as usize)
    }

    pub fn duplicate(&self) -> io::Result<UdpSocket> {
        self.inner.duplicate().map(|s| UdpSocket { inner: s })
    }

    pub fn set_read_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.inner.set_timeout(dur, libc::SO_RCVTIMEO)
    }

    pub fn set_write_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.inner.set_timeout(dur, libc::SO_SNDTIMEO)
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        self.inner.timeout(libc::SO_RCVTIMEO)
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        self.inner.timeout(libc::SO_SNDTIMEO)
    }

    pub fn set_broadcast(&self, broadcast: bool) -> io::Result<()> {
        setsockopt(&self.inner, libc::SOL_SOCKET, libc::SO_BROADCAST, broadcast as c_int)
    }

    pub fn broadcast(&self) -> io::Result<bool> {
        let raw: c_int = getsockopt(&self.inner, libc::SOL_SOCKET, libc::SO_BROADCAST)?;
        Ok(raw != 0)
    }

    pub fn set_multicast_loop_v4(&self, multicast_loop_v4: bool) -> io::Result<()> {
        setsockopt(&self.inner, libc::IPPROTO_IP, libc::IP_MULTICAST_LOOP, multicast_loop_v4 as c_int)
    }

    pub fn multicast_loop_v4(&self) -> io::Result<bool> {
        let raw: c_int = getsockopt(&self.inner, libc::IPPROTO_IP, libc::IP_MULTICAST_LOOP)?;
        Ok(raw != 0)
    }

    pub fn set_multicast_ttl_v4(&self, multicast_ttl_v4: u32) -> io::Result<()> {
        setsockopt(&self.inner, libc::IPPROTO_IP, libc::IP_MULTICAST_TTL, multicast_ttl_v4 as c_int)
    }

    pub fn multicast_ttl_v4(&self) -> io::Result<u32> {
        let raw: c_int = getsockopt(&self.inner, libc::IPPROTO_IP, libc::IP_MULTICAST_TTL)?;
        Ok(raw as u32)
    }

    pub fn set_multicast_loop_v6(&self, multicast_loop_v6: bool) -> io::Result<()> {
        setsockopt(&self.inner, libc::IPPROTO_IPV6, libc::IPV6_MULTICAST_LOOP, multicast_loop_v6 as c_int)
    }

    pub fn multicast_loop_v6(&self) -> io::Result<bool> {
        let raw: c_int = getsockopt(&self.inner, libc::IPPROTO_IPV6, libc::IPV6_MULTICAST_LOOP)?;
        Ok(raw != 0)
    }

    pub fn join_multicast_v4(&self, multiaddr: &Ipv4Addr, interface: &Ipv4Addr)
                         -> io::Result<()> {
        let mreq = libc::ip_mreq {
            imr_multiaddr: *multiaddr.as_inner(),
            imr_interface: *interface.as_inner(),
        };
        setsockopt(&self.inner, libc::IPPROTO_IP, libc::IP_ADD_MEMBERSHIP, mreq)
    }

    pub fn join_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32)
                         -> io::Result<()> {
        let mreq = libc::ipv6_mreq {
            ipv6mr_multiaddr: *multiaddr.as_inner(),
            ipv6mr_interface: interface as i32,
        };
        setsockopt(&self.inner, libc::IPPROTO_IPV6, libc::IPV6_ADD_MEMBERSHIP, mreq)
    }

    pub fn leave_multicast_v4(&self, multiaddr: &Ipv4Addr, interface: &Ipv4Addr)
                          -> io::Result<()> {
        let mreq = libc::ip_mreq {
            imr_multiaddr: *multiaddr.as_inner(),
            imr_interface: *interface.as_inner(),
        };
        setsockopt(&self.inner, libc::IPPROTO_IP, libc::IP_DROP_MEMBERSHIP, mreq)
    }

    pub fn leave_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32)
                          -> io::Result<()> {
        let mreq = libc::ipv6_mreq {
            ipv6mr_multiaddr: *multiaddr.as_inner(),
            ipv6mr_interface: interface as i32,
        };
        setsockopt(&self.inner, libc::IPPROTO_IPV6, libc::IPV6_DROP_MEMBERSHIP, mreq)
    }

    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        setsockopt(&self.inner, libc::IPPROTO_IP, libc::IP_TTL, ttl as c_int)
    }

    pub fn ttl(&self) -> io::Result<u32> {
        let raw: c_int = getsockopt(&self.inner, libc::IPPROTO_IP, libc::IP_TTL)?;
        Ok(raw as u32)
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.inner.take_error()
    }

    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.inner.set_nonblocking(nonblocking)
    }

    pub fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }

    pub fn send(&self, buf: &[u8]) -> io::Result<usize> {
        let ret = cvt(unsafe {
            libc::send(*self.inner.as_inner(),
                    buf.as_ptr() as *const _,
                    buf.len(),
                    libc::MSG_NOSIGNAL)
        })?;
        Ok(ret as usize)
    }

    pub fn connect(&self, addr: &SocketAddr) -> io::Result<()> {
        let (addrp, len) = addr.into_inner();
        cvt_r(|| unsafe { libc::connect(*self.inner.as_inner(), addrp, len) }).map(|_| ())
    }
}

impl FromInner<Socket> for UdpSocket {
    fn from_inner(socket: Socket) -> UdpSocket {
        UdpSocket { inner: socket }
    }
}

impl fmt::Debug for UdpSocket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = f.debug_struct("UdpSocket");

        if let Ok(addr) = self.socket_addr() {
            res.field("addr", &addr);
        }

        let name = if cfg!(windows) {"socket"} else {"fd"};
        res.field(name, &self.inner.as_inner())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use collections::HashMap;

    #[test]
    fn no_lookup_host_duplicates() {
        let mut addrs = HashMap::new();
        let lh = match lookup_host("localhost") {
            Ok(lh) => lh,
            Err(e) => panic!("couldn't resolve `localhost': {}", e)
        };
        let _na = lh.map(|sa| *addrs.entry(sa).or_insert(0) += 1).count();
        assert!(addrs.values().filter(|&&v| v > 1).count() == 0);
    }
}
