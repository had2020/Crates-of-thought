#include <limits.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>

void printBinary(int num) {
  for (int i = sizeof(int) * 8 - 1; i >= 0; i--) {
    printf("%d", (num >> i) & 1);
  }
  printf("\n");
}

int reverseBits(int n) {
  int r = 0;
  for (int i = 0; i < 32; i++) {
    r = (r << 1) | (n & 1);
    n >>= 1;
  }
  return r;
}

int main() {
  printBinary(reverseBits(0b01011));
  return 0;
}
