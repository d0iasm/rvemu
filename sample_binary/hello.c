#include <unistd.h>

int main()
{
  write(1, "Hello, RISCV\n", 13);
  return 0;
}
