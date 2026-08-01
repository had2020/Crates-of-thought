#include <string.h>
char *strrev(char *string) {
  size_t len = strlen(string);
  size_t from_back = len - 1;
  size_t half_len = len / 2;

  for (int i = 0; i < half_len; i++) {
    char copy = string[from_back];
    string[from_back] = string[i];
    string[i] = copy;
    from_back--;
  }
  return string;
}
