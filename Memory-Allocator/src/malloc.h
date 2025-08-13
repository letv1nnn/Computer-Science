#ifndef MALLOC_H
#define MALLOC_H

#include <assert.h>
#include <string.h>
#include <sys/types.h>
#include <unistd.h>

#define DEBUG_MODE 1

void *malloc(size_t size);
void free(void *ptr);
void *calloc(size_t nmemb, size_t size);
void *realloc(void *ptr, size_t size);


#endif
