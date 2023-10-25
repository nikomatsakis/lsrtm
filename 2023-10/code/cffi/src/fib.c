#include <stdio.h>

// PART 1

int fibonacci(int n) {
    if (n == 0) {
        return 0;
    } else if (n == 1) {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

// PART 2

// The rust function
int factorial(int n);

void call_rust(int n) {
    int r = factorial(n);
    printf("n = %d\n", r);
}

// PART 3

typedef struct {
    int16_t a;
    int64_t b;
    int16_t c;
} TestStruct;

TestStruct test_struct(TestStruct* ts) {
    return *ts;
}