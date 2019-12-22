/*! @file Vector.hpp
 *  @brief Implements templated Vector data structure */

// #include <iostream>
template <typename T>
Vector<T>::Vector() : _used{0}, _reserved{0}, _data{std::make_unique<T[]>()} {}

template <typename T>
Vector<T>::Vector(size_t size) : _used{size} {
    _reserved = _used / 2 * 4;
    _data = std::make_unique<T[]>(_reserved);
}

template <typename T>
Vector<T>::Vector(size_t size, const T& val) : _used{size} {
    _reserved = _used / 2 * 4;
    _data = std::make_unique<T[]>(_reserved);
    for (size_t i = 0; i < _used; ++i) {
        _data[i] = val;
    }
}


template <typename T>
const T& Vector<T>::operator[](size_t idx) const {
    assert(idx < _used);
    _data[idx];
}
