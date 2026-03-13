# always-online-stun Development Patterns

> Auto-generated skill from repository analysis

## Overview

The always-online-stun repository is a Rust-based STUN (Session Traversal Utilities for NAT) server validation system that continuously monitors and maintains lists of working STUN servers across different protocols and IP versions. The codebase focuses on network connectivity testing, NAT traversal capabilities, and automated health checking of STUN infrastructure.

## Coding Conventions

### File Naming
- Use `snake_case` for all file names
- Validation result files use descriptive names: `valid_hosts.txt`, `valid_ipv4s_tcp.txt`
- Separate files by protocol (UDP/TCP) and IP version (IPv4/IPv6)

### Import Style
```rust
// Mixed import patterns - adapt to context
use std::net::{IpAddr, SocketAddr};
use tokio::net::UdpSocket;
```

### Export Style
- Mixed export patterns based on module requirements
- Focus on clarity and functionality over strict consistency

### Commit Messages
- Use freeform commit messages averaging ~33 characters
- Focus on clarity and brevity
- Describe the validation updates or functionality changes

## Workflows

### STUN Server Validation Check
**Trigger:** When scheduled automated checks run to verify STUN server availability
**Command:** `/run-stun-validation`

1. **Initialize validation process** - Set up network testing infrastructure for multiple protocols
2. **Test UDP protocol servers** - Validate STUN servers over UDP for both IPv4 and IPv6
3. **Test TCP protocol servers** - Validate STUN servers over TCP for both IPv4 and IPv6  
4. **Perform NAT testing validation** - Check NAT traversal capabilities for each working server
5. **Update validation files** - Write results to appropriate validation list files:
   - `valid_hosts.txt` and `valid_hosts_tcp.txt` for hostname-based servers
   - `valid_ipv4s.txt` and `valid_ipv4s_tcp.txt` for IPv4 addresses
   - `valid_ipv6s.txt` and `valid_ipv6s_tcp.txt` for IPv6 addresses
   - `valid_nat_testing_*` files for NAT-capable servers
6. **Commit validation results** - Update repository with current server status

```rust
// Example validation structure
async fn validate_stun_server(addr: SocketAddr, protocol: Protocol) -> ValidationResult {
    match protocol {
        Protocol::Udp => test_udp_stun(addr).await,
        Protocol::Tcp => test_tcp_stun(addr).await,
    }
}
```

## Testing Patterns

- Test files follow `*.test.*` pattern
- Focus on network connectivity and protocol validation
- Automated testing runs frequently (~20-30 times per day)
- Tests cover multiple scenarios:
  - Protocol compatibility (UDP/TCP)
  - IP version support (IPv4/IPv6)
  - NAT traversal capabilities
  - Server availability and response times

## Commands

| Command | Purpose |
|---------|---------|
| `/run-stun-validation` | Execute comprehensive STUN server validation across all protocols and IP versions |
| `/test-udp-servers` | Validate only UDP-based STUN servers |
| `/test-tcp-servers` | Validate only TCP-based STUN servers |
| `/check-nat-capability` | Test NAT traversal functionality for validated servers |
| `/update-server-lists` | Refresh and commit all validation result files |