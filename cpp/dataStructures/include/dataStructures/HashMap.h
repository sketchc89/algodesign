#pragma once

#include "List.h"

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
    void insert(K key, const V& value);
    std::experimental::optional<V> at(K key);

private:
    static constexpr size_t DEFAULT_ARRAY_SIZE = 16;
    size_t _arraySize;
    List<KeyValuePair<K, V>>* _array;
    std::hash<K> _hash;

};  // class HashMap

}  // namespace ds


#include "HashMap.hpp"
