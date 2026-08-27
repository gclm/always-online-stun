---
name: update-valid-hosts-and-ip-lists
description: Workflow command scaffold for update-valid-hosts-and-ip-lists in always-online-stun.
allowed_tools: ["Bash", "Read", "Write", "Grep", "Glob"]
---

# /update-valid-hosts-and-ip-lists

Use this workflow when working on **update-valid-hosts-and-ip-lists** in `always-online-stun`.

## Goal

Updates all valid hosts and IP address list files, including both standard and NAT-testing variants, for both IPv4 and IPv6, and for both UDP and TCP. Likely a periodic refresh of network test data.

## Common Files

- `valid_hosts.txt`
- `valid_hosts_tcp.txt`
- `valid_ipv4s.txt`
- `valid_ipv4s_tcp.txt`
- `valid_ipv6s.txt`
- `valid_ipv6s_tcp.txt`

## Suggested Sequence

1. Understand the current state and failure mode before editing.
2. Make the smallest coherent change that satisfies the workflow goal.
3. Run the most relevant verification for touched files.
4. Summarize what changed and what still needs review.

## Typical Commit Signals

- Regenerate or update valid_hosts.txt and valid_hosts_tcp.txt
- Regenerate or update valid_ipv4s.txt and valid_ipv4s_tcp.txt
- Regenerate or update valid_ipv6s.txt and valid_ipv6s_tcp.txt
- Regenerate or update valid_nat_testing_hosts.txt and valid_nat_testing_hosts_tcp.txt
- Regenerate or update valid_nat_testing_ipv4s.txt and valid_nat_testing_ipv4s_tcp.txt

## Notes

- Treat this as a scaffold, not a hard-coded script.
- Update the command if the workflow evolves materially.