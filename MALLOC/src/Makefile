CC = gcc
CFLAGS = -g -Wall -Wextra -Werror -fsanitize=address -std=gnu11

TARGET = final
TEST_PROGRAM = test_malloc

all: $(TARGET)

$(TARGET): libmalloc.a $(TEST_PROGRAM)
	echo "Buil complete - use ./$(TEST_PROGRAM) to test"

libmalloc.a: malloc.o
	ar rcs $@ $^

$(TEST_PROGRAM): test_malloc.o libmalloc.a
	$(CC) $(FLAGS) -o $@ $^

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f *.o *.a $(TEST_PROGRAM)

