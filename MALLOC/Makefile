CC = gcc
CFLAGS = -Wall -Wextra -g

all: libmalloc.a

libmalloc.a: malloc.o
	ar rcs $@ $^

malloc.o: malloc.c
	$(CC) $(CFLAGS) -c malloc.c

clean:
	rm -f *.o libmalloc.a
