#include <algorithm>
#include <array>

extern "C" int32_t func(int32_t flag, int32_t *seq, int32_t size) {
  std::array<int, 7> cnt = {};
  std::for_each_n(seq, size, [flag10 = flag * 10, &cnt](int x) {
    int index = x - flag10;
    if (1 <= index && index <= 6)
      cnt[index]++;
  });
  auto idx1 = std::max_element(cnt.begin(), cnt.end()) - cnt.begin();
  auto idx2 = cnt.rend() - std::max_element(cnt.rbegin(), cnt.rend()) - 1;
  return idx1 == idx2 ? flag * 10 + idx1 : 10;
}
