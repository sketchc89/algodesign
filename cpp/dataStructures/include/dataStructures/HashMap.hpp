
namespace ds {

template <typename K, typename V>
KeyValuePair<K, V>::KeyValuePair() : key{K{}}, value{V{}} {}

template <typename K, typename V>
KeyValuePair<K, V>::KeyValuePair(K k, V v) : key{k}, value{v} {}

template <typename K, typename V>
HashMap<K, V>::HashMap()
        : _arraySize{DEFAULT_ARRAY_SIZE}, _array{new List<KeyValuePair<K, V>>[_arraySize]} {}

template <typename K, typename V>
HashMap<K, V>::~HashMap() {
    delete[] _array;
}

template <typename K, typename V>
void HashMap<K, V>::insert(__attribute__((unused)) K key, __attribute__((unused)) const V& value) {
    auto k = _hash(key);
    KeyValuePair<K, V> kv(key, value);
    _array[k % _arraySize].pushBack(kv);
}

template <typename K, typename V>
std::experimental::optional<V> HashMap<K, V>::at(K key) {
    std::experimental::optional<V> res;

    auto kv = _array[_hash(key) % _arraySize].head();
    if (kv) {
        res = kv.value().value;
    }
    return res;
}

}  // namespace ds
