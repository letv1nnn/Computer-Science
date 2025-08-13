#include "malloc.h"
#include <stdio.h>
#include <assert.h>


void test_malloc() {
    printf("== Malloc Test ==\n");
    int *arr = (int *)malloc(10 *sizeof(int));
    assert(arr != NULL);
    printf("Allocated 10 ints at %p\n", (void *)arr);

    for (int i = 0; i < 10; i++)
        arr[i] = i * 10;
    for (int i = 0; i < 10; i++)
        assert(arr[i] == i * 10);

    free(arr);
    printf("Freed memory successfully\n");
}

void test_calloc() {
    printf("== Calloc Test ==\n");
    int *arr = (int *)calloc(20, sizeof(int));
    assert(arr != NULL);

    for (int i = 0; i < 20; i++)
        assert(arr[i] == 0);

    printf("Calloc initialized 20 ints to zero at %p\n", (void*)arr);

    free(arr);
    printf("Freed memory successfully\n");
}

void test_realloc() {
    printf("== Realloc Test ==\n");
    int *arr = (int *)malloc(5 * sizeof(int));
    assert(arr != NULL);

    for (int i = 0; i < 5; i++)
        arr[i] = i + 1;

    int *new_arr = (int *)realloc(arr, 10 * sizeof(int));
    assert(new_arr != NULL);
    for (int i = 0; i < 5; i++) {
        assert(new_arr[i] == i + 1 && "Data not copied during realloc");
    }

    for (int i = 5; i < 10; i++) {
        new_arr[i] = i + 1;
    }
    free(new_arr);
    printf("Freed memory successfully\n\n");
}

int main() {
    printf("===== Memory allocator tests =====\n\n");

    test_malloc();
    test_calloc();
    test_realloc();

    printf("===== All tests passed =====\n");
    return 0;
}

