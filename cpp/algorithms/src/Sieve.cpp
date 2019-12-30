#include <bitset>
constexpr static size_t SIEVE_LENGTH = 1'000'000;

std::bitset<SIEVE_LENGTH> createSieve() {
    std::bitset<SIEVE_LENGTH> bs;
    bs.set();
    bs.reset(0);
    bs.reset(1);
    for (auto i = 2; i <= sqrt(bs.size()); ++i) {
        if (bs[i]) {
            for (auto j = 2; i * j < bs.size(); ++j) {
                bs.reset(i * j);
            }
        }
    }
    return bs;
}
