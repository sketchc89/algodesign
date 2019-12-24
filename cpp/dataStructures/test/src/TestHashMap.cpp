#include <gmock/gmock.h>
#include <gtest/gtest.h>

#include <dataStructures/HashMap.h>

using ds::HashMap;
using namespace ::testing;

namespace testing {

TEST(HashMap, Initializes) {
    HashMap<int32_t, std::string> map;
}

class EmptyHashMapFixture : public Test {
protected:
    HashMap<int32_t, std::string> map;
};  // class EmptyHashMapFixture

TEST_F(EmptyHashMapFixture, InsertKey) {
    map.insert(100, std::string("one hundred"));
    ASSERT_THAT(map.at(100).value(), Eq("one hundred"));
}

TEST_F(EmptyHashMapFixture, InsertKeyTwice) {
    map.insert(100, std::string("one hundred"));
    map.insert(100, std::string("two hundred"));
    ASSERT_THAT(map.at(100).value(), Eq("one hundred"));
}

TEST_F(EmptyHashMapFixture, InsertReplaceKey) {
    map.insert(100, std::string("one hundred"));
    map.insert(100, std::string("two hundred"));
    ASSERT_THAT(map.at(100).value(), Eq("one hundred"));
}

}  // namespace testing
