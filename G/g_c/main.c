#include <stdint.h>

#define MAX(x, y) ((x) > (y) ? (x) : (y))

int32_t func(int32_t flag, int32_t *seq, int32_t size) {
  int cnt[7] = {};
  int flag10 = flag * 10;
  for (int i = 0; i < size; ++i) {
    int index = seq[i] - flag10;
    if (1 <= index && index <= 6)
      cnt[index]++;
  }
  int max_cnt = 0;
  for (int i = 1; i <= 6; ++i)
    max_cnt = MAX(max_cnt, cnt[i]);
  int ans = 0;
  for (int i = 1; i <= 6; ++i)
    if (cnt[i] == max_cnt) {
      if (ans == 0) {
        ans = flag10 + i;
      } else {
        return 10;
      }
    }
  return ans;
}
