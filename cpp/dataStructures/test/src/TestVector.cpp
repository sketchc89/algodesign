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
    // void SetUp() override {}
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

class FilledVectorFixture : public EmptyVectorFixture {
    void SetUp() override {
        for (size_t i = 0; i < 32; ++i) {
            v.pushBack(i);
        }
        std::cerr << v[0];
    }
};  // class FilledVectorFixture


TEST_F(FilledVectorFixture, CopyConstructible) {
    Vector other(v);
    ASSERT_THAT(other[0], Eq(0));
    ASSERT_THAT(other[16], Eq(16));
}


TEST_F(FilledVectorFixture, MoveConstructible) {
    Vector other(std::move(v));
    ASSERT_THAT(other[31], Eq(31));
}

TEST_F(FilledVectorFixture, MoveAssignable) {
    Vector<int32_t> other;
    other = std::move(v);
    ASSERT_THAT(other[31], Eq(31));
}

TEST_F(FilledVectorFixture, CopyAssignable) {
    Vector<int32_t> other;
    other = v;
    ASSERT_THAT(other[31], Eq(31));
}

}  // namespace testing
