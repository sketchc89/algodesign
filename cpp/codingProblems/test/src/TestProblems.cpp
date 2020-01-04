#include "Problems.h"

#include <gmock/gmock.h>
#include <gtest/gtest.h>

using namespace std;

namespace testing {

TEST(SingleElementInASortedArray, Examples) {
    vector<int> vi{1, 1, 2, 3, 3, 4, 4, 8, 8};

    ASSERT_THAT(uniqueBinarySearch(vi), Eq(2));
}

}  // namespace testing
