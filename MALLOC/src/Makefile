CC = gcc
CFLAGS = -g -Wall -Wextra -Werror -std=gnu11

TARGET = test_malloc

all: $(TARGET)

$(TARGET): test_malloc.o malloc.o
	$(CC) $(CFLAGS) -o $@ $^

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f *.o $(TARGET)

