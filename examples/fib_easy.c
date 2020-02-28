int fib(int n);
int main() {
    return fib(1);
}

int fib(int n) {
    if (n == 0 || n == 1)
        return n;
    else
        return (fib(n-1) + fib(n-2));
}
