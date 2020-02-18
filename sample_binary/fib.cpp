int main() {
  int n=10;
  if(n <= 1) {
    return n;
  }
  int fib = 1;
  int fibPrev = 1;
  for(int i = 2; i < n; i++) {
    int temp = fib;
    fib += fibPrev;
    fibPrev = temp;
  }
  return fib;
}
