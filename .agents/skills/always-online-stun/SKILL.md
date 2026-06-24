```markdown
# always-online-stun Development Patterns

> Auto-generated skill from repository analysis

## Overview
This skill introduces the development patterns and workflows used in the `always-online-stun` Rust codebase. You'll learn about the project's coding conventions, file organization, and how to execute and automate key repository workflows—especially the process for updating host and IP lists used for network and NAT testing.

## Coding Conventions

### File Naming
- Use **snake_case** for all file names.
  - Example: `valid_hosts.txt`, `valid_ipv4s_tcp.txt`, `src/network_utils.rs`

### Import Style
- Use **relative imports** within the project.
  - Example:
    ```rust
    mod utils;
    use crate::network::stun_client;
    ```

### Export Style
- Use **named exports** for modules and functions.
  - Example:
    ```rust
    pub fn perform_stun_check() { ... }
    pub mod nat_testing;
    ```

### Commit Patterns
- Commit messages are **freeform** (no strict prefix), averaging 34 characters.
  - Example:
    ```
    update valid_ipv4s.txt and valid_hosts.txt
    ```

## Workflows

### update-valid-hosts-and-ip-lists
**Trigger:** When you need to refresh or regenerate the lists of valid hosts and IPs used for connectivity or NAT testing (typically ~30 times/month).  
**Command:** `/refresh-valid-hosts`

1. Regenerate or update the following files:
    - `valid_hosts.txt`
    - `valid_hosts_tcp.txt`
    - `valid_ipv4s.txt`
    - `valid_ipv4s_tcp.txt`
    - `valid_ipv6s.txt`
    - `valid_ipv6s_tcp.txt`
    - `valid_nat_testing_hosts.txt`
    - `valid_nat_testing_hosts_tcp.txt`
    - `valid_nat_testing_ipv4s.txt`
    - `valid_nat_testing_ipv4s_tcp.txt`
    - `valid_nat_testing_ipv6s.txt`
    - `valid_nat_testing_ipv6s_tcp.txt`
2. Ensure both standard and NAT-testing variants are updated for IPv4 and IPv6, and for both UDP and TCP.
3. Commit all updated files together with a timestamped message.
    - Example commit message:
      ```
      refresh: update valid hosts and IP lists (2024-06-10)
      ```
4. Push the changes to the repository.

#### Example: Updating a List File in Rust
```rust
use std::fs::write;

fn update_valid_hosts(new_hosts: &str) -> std::io::Result<()> {
    write("valid_hosts.txt", new_hosts)
}
```

## Testing Patterns

- **Framework:** Unknown (not explicitly detected).
- **File Pattern:** Test files follow the `*.test.*` pattern.
  - Example: `network_utils.test.rs`
- **Best Practice:** Place tests in files named with `.test.` for clarity and consistency.

#### Example: Test File Structure
```rust
// network_utils.test.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stun_response_parsing() {
        // test logic here
    }
}
```

## Commands

| Command                | Purpose                                                      |
|------------------------|--------------------------------------------------------------|
| /refresh-valid-hosts   | Regenerate and update all valid hosts and IP lists           |

```