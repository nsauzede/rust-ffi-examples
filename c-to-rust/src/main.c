#include <stdio.h>

#include "my_library.h"

int main() {
    int input = 4;
    int output = double_input(input);
    printf("%d * 2 = %d\n", input, output);
    char *hello = hello_world();
    printf("hello=\"%s\" (should be \"Hello world!\")\n", hello);
    free_string(hello);
    return 0;
}
