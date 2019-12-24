#include <dataStructures/List.h>

#include <gmock/gmock.h>
#include <gtest/gtest.h>

#include <experimental/optional>
#include <string>

using ds::List;
using namespace ::testing;

namespace testing {
TEST(List, Initializes) {
    List<int32_t> list;
}

class EmptyListFixture : public Test {
protected:
    List<int32_t> l;
};  // class EmptyListFixture

TEST_F(EmptyListFixture, EmptyList) {
    ASSERT_FALSE(l.head());
}

TEST_F(EmptyListFixture, PushesBack) {
    l.pushBack(5);
    auto res = l.head();
    ASSERT_TRUE(res);
    ASSERT_THAT(res.value(), Eq(5));
}

TEST_F(EmptyListFixture, PushesBackMultiple) {
    l.pushBack(5);
    l.pushBack(6);

    auto res = l.head();
    ASSERT_TRUE(res);
    ASSERT_THAT(res.value(), Eq(6));
}

TEST_F(EmptyListFixture, PopsBack) {
    l.pushBack(5);

    l.popBack();
    auto res = l.head();
    ASSERT_FALSE(res);
}

TEST_F(EmptyListFixture, PopsBackMultiple) {
    l.pushBack(5);
    l.pushBack(6);

    auto res = l.head();
    ASSERT_TRUE(res);
    ASSERT_THAT(res.value(), Eq(6));

    l.popBack();
    res = l.head();
    ASSERT_TRUE(res);
    ASSERT_THAT(res.value(), Eq(5));

    l.popBack();
    res = l.head();
    ASSERT_FALSE(res);
}

TEST_F(EmptyListFixture, Swaps) {
    List<int32_t> l2;
    l.swap(l2);
}

TEST_F(EmptyListFixture, StdSwaps) {
    List<int32_t> l2;
    std::swap(l, l2);
}

class FilledListFixture : public EmptyListFixture {
    void SetUp() override {
        for (size_t i = 0; i < 32; ++i) {
            l.pushBack(i);
        }
    }
};  // class FilledListFixture

TEST_F(FilledListFixture, Find) {
    for (size_t i = 0; i < 32; ++i) {
        ASSERT_TRUE(l.find(0));
    }
    ASSERT_FALSE(l.find(32));
}

TEST_F(FilledListFixture, Remove) {
    int32_t val = 16;
    ASSERT_TRUE(l.find(val));
    ASSERT_TRUE(l.remove(val));
    ASSERT_FALSE(l.find(val));
}

}  // namespace testing
