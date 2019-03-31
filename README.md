# Lubeck

Fast, Memory safe, Alternative Userspace Scheduler and Container Runtime execution pipeline in Rust. Lubeck is meant to provide an integration path to OCI. Lubeck supports guaranteed reservations, jails and zones with namespaces (via an interlocking directorate); container lifecycle operation observability; rootless containers, supervisors, lenient cgroup namespaces.

Lubeck is very much a logical extension to the existing unix process model with self deterrmined threads. These threads have a signal mask, assigned a CPU affinity and cgroup resource metric. Lubeck can make informed scheduling decisions based on memory consistency.

There is only one sleeping OS thread at Lubeck which distributes delays to unwanted or troublesome threads. In addition, with vEnvi integration, Lubeck is fit for thread level, signal driven: Causal Profiling. 


