#include <dataStructures/List.h>

#include <gmock/gmock.h>
#include <gtest/gtest.h>

#include <experimental/optional>

using ds::List;

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
    ASSERT_TRUE(l.head());
    ASSERT_THAT(l.head().value(), Eq(5));
}

}  // namespace testing
