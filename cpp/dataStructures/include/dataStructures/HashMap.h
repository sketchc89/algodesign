#pragma once

#include "List.h"

#include <experimental/optional>
#include <functional>
#include <memory>

namespace ds {

/*! @brief Key and Value pair */
template <typename K, typename V>
struct KeyValuePair {
    KeyValuePair();
    KeyValuePair(K k, V v);
    K key;
    V value;
};  // struct KeyValuePair


/*! @brief Unordered HashMap */
template <typename K, typename V>
class HashMap {
public:
    HashMap();
    virtual ~HashMap();
    void insert(const K& key, const V& value);
    void insertReplace(const K& key, const V& value);
    std::experimental::optional<V> at(const K& key);

private:
    static constexpr size_t DEFAULT_ARRAY_SIZE = 16;
    size_t _arraySize;
    List<KeyValuePair<K, V>>* _array;
    std::hash<K> _hash;

};  // class HashMap

template <typename K, typename V>
bool operator==(const ds::KeyValuePair<K, V>& lhs, const ds::KeyValuePair<K, V>& rhs);

}  // namespace ds


#include "HashMap.hpp"
