#include <gmock/gmock.h>
#include <gtest/gtest.h>

#include <dataStructures/HashMap.h>

using ds::HashMap;

namespace testing {

TEST(HashMap, Initializes) {
    HashMap<int32_t, std::string> map;
}

class EmptyHashMapFixture : public Test {
protected:
    HashMap<int32_t, std::string> map;
};  // class EmptyHashMapFixture

}  // namespace testing
