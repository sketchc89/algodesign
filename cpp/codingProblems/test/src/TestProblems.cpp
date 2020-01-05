#include "Problems.h"

#include <gmock/gmock.h>
#include <gtest/gtest.h>

using namespace std;

namespace testing {

TEST(SingleElementInASortedArray, Examples1) {
    vector<int> vi{1, 1, 2, 3, 3, 4, 4, 8, 8};

    ASSERT_THAT(uniqueBinarySearch(vi), Eq(2));
}
TEST(SingleElementInASortedArray, Examples2) {
    vector<int> vi{3, 3, 7, 7, 10, 11, 11};

    ASSERT_THAT(uniqueBinarySearch(vi), Eq(10));
}

}  // namespace testing
