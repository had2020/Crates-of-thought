#include <stddef.h>
#include <stdio.h>

void printBinary(int num) {
  for (int i = sizeof(int) * 8 - 1; i >= 0; i--) {
    printf("%d", (num >> i) & 1);
  }
  printf("\n");
}

int reverseBits(int n) {
  int cn = 0;
  const int nlen = (sizeof(n) * 8) - 1;
  for (int i = nlen; i > 0; i--) {
    cn |= (n >> i) << i;
    printBinary(cn);
  }
  return cn;
}

int main() {
  printBinary(reverseBits(0b01011));
  return 0;
}
