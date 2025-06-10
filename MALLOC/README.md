# Custom Memory Allocator

### Implementation of `malloc`, `calloc`, `realloc`, and `free` in C

This project demonstrates a basic memory allocator written in C, mimicking the functionality of the standard library's dynamic memory management functions. It uses a linked list to manage memory blocks, allowing for dynamic allocation and deallocation at runtime.

---

## Features

- Custom implementation of:
  - `malloc(size_t size)`
  - `calloc(size_t num, size_t size)`
  - `realloc(void *ptr, size_t size)`
  - `free(void *ptr)`
- Memory blocks managed with a simple **singly linked list**
- Basic **free list reuse** to avoid unnecessary memory requests
- Clean separation of metadata and user data
- Optional debug checks (enabled via `DEBUG_MODE`)

---

## Building the Project

### Compile
```bash
make clean
make CFLAGS="-g -Wall -Wextra -Werror -std=gnu11"
