#include <stdio.h>

int get(int x) {
  return x * 8;
}

int main(int, char **) {
  int res = get(8);
  printf("%d", res);
  return 0;
}

