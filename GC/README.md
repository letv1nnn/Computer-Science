# Garbage Collector

A lightweight mark-and-sweep garbage collector implemented in C as part of the [Malloc-and-GC](https://github.com/letv1nnn/Malloc-and-GC/tree/main/MALLOC) repository. Designed for educational use, it's a minimal yet extendable reference for memory management.

## Features
- Manual `malloc`-style allocation via `gc_malloc`
- Automated tracing of live pointers from root set
- Classic mark–sweep cleanup to reclaim unused memory
- Designed for easy integration and experimentation
