#include <gmock/gmock.h>
#include <gtest/gtest.h>

#include <dataStructures/Vector.h>

using ds::Vector;

namespace testing {

TEST(Vector, Initializes) {
    Vector<int32_t> v(3);
    EXPECT_THAT(v[0], Eq(0));
    EXPECT_THAT(v[1], Eq(0));
    EXPECT_THAT(v[2], Eq(0));
}

TEST(Vector, ValueInitializes) {
    Vector<int32_t> v(3, 5);
    EXPECT_THAT(v[0], Eq(5));
    EXPECT_THAT(v[1], Eq(5));
    EXPECT_THAT(v[2], Eq(5));
}

}  // namespace testing
