/*! @file Vector.hpp
 *  @brief Implements templated Vector data structure */

template <typename T>
Vector<T>::Vector() {}

template <typename T>
Vector<T>::Vector(size_t size) {}

template <typename T>
Vector<T>::Vector(size_t size, const T& val) {}

template <typename T>
const T& Vector<T>::operator[](size_t idx) const {
    assert(idx < _used);
    return _array[idx];
}
