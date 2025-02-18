#include <stdio.h>
#include <stdlib.h>

typedef void* hello_String;
extern size_t hello_String_copy(hello_String s, char* buffer, size_t buffer_size);
extern void hello_String_free(hello_String s);
extern size_t hello_String_length(hello_String s);
extern hello_String hello_String_new(const char* s, size_t size);

extern hello_String hello_greeting_for(hello_String s);
extern void hello_init(void);

char* string_to_c(hello_String s);

int main(void) {
    hello_init();
    // Make strings.
    hello_String name = hello_String_new("world", 0);
    if (!name) goto DONE;
    char* greeting = string_to_c(hello_greeting_for(name));
    if (!greeting) goto FREE_NAME;
    // Print result.
    printf("%s\n", greeting);
    // Clean up.
    free(greeting);
    FREE_NAME:
    hello_String_free(name);
    DONE:
    return 0;
}

char* string_to_c(hello_String s) {
    size_t size = hello_String_length(s) + 1;
    char* c = malloc(size);
    if (!c) goto DONE;
    hello_String_copy(s, c, size);
    hello_String_free(s);
    DONE:
    return c;
}
