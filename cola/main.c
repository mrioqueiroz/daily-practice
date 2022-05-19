#include <stddef.h>
#include <stdlib.h>

const char *
who_is_next (long long n, size_t length, const char *const names[length])
{
  long long *sizes = malloc(length * sizeof(n));

  for (size_t i = 0; i < length; ++i) {
    sizes[i] = 1LL;
  }

  size_t head = 0;

  while (n > sizes[head]) {
    n -= sizes[head];
    sizes[head] *= 2;
    head = (head + 1) % length;
  }
  return names[head];
}

int main(void) {
  return 0;
}
