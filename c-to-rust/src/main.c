#include <stdint.h>
#include <stdio.h>

extern int32_t double_input(int32_t input);
extern char *hello_world();

int main() {
    int input = 4;
    int output = double_input(input);
    printf("%d * 2 = %d\n", input, output);
    char *hello = hello_world();
    printf("hello=\"%s\" (should be \"Hello world!\")\n", hello);
    return 0;
}
