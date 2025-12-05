# C/Makefile

CC      = cc
CFLAGS  = -Wall -Wextra -Wpedantic -std=c99 -O2 -Iinclude/
LDFLAGS =

SRCDIR  = src
OBJDIR  = obj
BINDIR  = bin

SOURCES = $(SRCDIR)/lz77.c $(SRCDIR)/main.c
OBJECTS = $(OBJDIR)/lz77.o $(OBJDIR)/main.o
TARGET  = $(BINDIR)/lz77

.PHONY: all clean test

all: $(TARGET)

$(TARGET): $(OBJECTS) | $(BINDIR)
	$(CC) $(LDFLAGS) -o $@ $(OBJECTS)

$(OBJDIR)/%.o: $(SRCDIR)/%.c | $(OBJDIR)
	$(CC) $(CFLAGS) -c -o $@ $<

$(OBJDIR):
	mkdir -p $(OBJDIR)

$(BINDIR):
	mkdir -p $(BINDIR)

clean:
	rm -rf $(OBJDIR) $(BINDIR)

# Basic round-trip test
test: $(TARGET)
	@echo "Testing compression round-trip..."
	@echo "The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog." > /tmp/lz77_test_input.txt
	@$(TARGET) -i /tmp/lz77_test_input.txt -o /tmp/lz77_test_compressed.lz77
	@$(TARGET) -d -i /tmp/lz77_test_compressed.lz77 -o /tmp/lz77_test_output.txt
	@diff -q /tmp/lz77_test_input.txt /tmp/lz77_test_output.txt && echo "PASS: Round-trip successful"
	@rm -f /tmp/lz77_test_input.txt /tmp/lz77_test_compressed.lz77 /tmp/lz77_test_output.txt
	@echo "Testing string input..."
	@$(TARGET) -s "abracadabra" -o /tmp/lz77_str.lz77
	@$(TARGET) -d -i /tmp/lz77_str.lz77 > /tmp/lz77_str_out.txt
	@echo -n "abracadabra" | diff - /tmp/lz77_str_out.txt && echo "PASS: String round-trip successful"
	@rm -f /tmp/lz77_str.lz77 /tmp/lz77_str_out.txt
