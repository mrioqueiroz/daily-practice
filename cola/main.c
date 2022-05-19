#include <assert.h>
#include <stddef.h>
#include <stdlib.h>

const char *
who_is_next (long long n, size_t length, const char *const names[length])
{
  long long sizes = 1;
  while (n > sizes * length) {
    n -= sizes * length;
    sizes *= 2;
  }
  assert(n <= sizes * length);
  return names[(n - 1) / sizes];
}

int main(void) {
  return 0;
}
