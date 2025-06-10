#ifndef MALLOC_H
#define MALLOC_H

#include <assert.h>
#include <string.h>
#include <sys/types.h>
#include <unistd.h>

#define DEBUG_MODE 0

void *malloc(size_t size);
void *calloc(size_t q_elements, size_t element_size);
void *realloc(void *ptr, size_t newSize);
void free(void *ptr);

#endif
