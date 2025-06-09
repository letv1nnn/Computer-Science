CC = gcc
CFLAGS = -Wall -Wextra -g

SRC = src/malloc.c
OBJ = malloc.o
LIB = libmalloc.a

all: $(LIB)

$(LIB): $(OBJ)
	ar rcs $@ $^

$(OBJ): $(SRC)
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f *.o *.a
