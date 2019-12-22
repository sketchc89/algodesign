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

class EmptyVectorFixture : public Test {
protected:
    Vector<int32_t> v;
    void SetUp() override {
        Vector<int32_t> v;
    }
};  // class EmptyVectorFixture

TEST_F(EmptyVectorFixture, PushesBack) {
    v.pushBack(10);
    v.pushBack(11);
    v.pushBack(12);
    EXPECT_THAT(v[0], Eq(10));
    EXPECT_THAT(v[1], Eq(11));
    EXPECT_THAT(v[2], Eq(12));
}

TEST_F(EmptyVectorFixture, PushesBackALot) {
    for (size_t i = 0; i < 256; ++i) {
        v.pushBack(i);
    }
}
}  // namespace testing
