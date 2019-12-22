/*! @file Vector.hpp
 *  @brief Implements templated Vector data structure */

#include <algorithm>  // std::copy, max, swap

template <typename T>
Vector<T>::Vector() : Vector<T>(0, T{}) {}

template <typename T>
Vector<T>::Vector(size_t size) : Vector<T>(size, T{}) {}

template <typename T>
Vector<T>::Vector(size_t size, const T& val)  //
        : _used{size},
          _reserved{std::max(MIN_RESERVED, _used)},
          _data{new T[_reserved]} {
    for (size_t i = 0; i < _used; ++i) {
        _data[i] = val;
    }
}

template <typename T>
Vector<T>::~Vector() {
    delete[] _data;
}

template <typename T>
Vector<T>::Vector(const Vector& other)  //
        : _used{other._used},
          _reserved{other._reserved},
          _data{new T[_reserved]} {
    std::copy(other._data, other._data + other._reserved, _data);
}

template <typename T>
Vector<T>::Vector(Vector&& other) noexcept  //
        : _used{},
          _reserved{},
          _data{nullptr} {
    other.swap(*this);
}

template <typename T>
Vector<T>& Vector<T>::operator=(const Vector& other) {
    Vector<T> temp(other);
    temp.swap(*this);
    return *this;
}

template <typename T>
Vector<T>& Vector<T>::operator=(Vector&& other) noexcept {
    other.swap(*this);
    return *this;
}

template <typename T>
void Vector<T>::swap(Vector<T>& other) noexcept {
    std::swap(_used, other._used);
    std::swap(_reserved, other._reserved);
    std::swap(_data, other._data);
}

template <typename T>
void Vector<T>::pushBack(const T& val) {
    if (_used >= _reserved) {
        growArray();
    }
    _data[_used] = val;
    _used++;
}

template <typename T>
void Vector<T>::growArray() {
    size_t oldSize = _reserved;
    _reserved *= GROW_FACTOR;
    T* newData = new T[_reserved];
    for (size_t i = 0; i < oldSize; ++i) {
        newData[i] = _data[i];
    }
    delete[] _data;
    _data = newData;
}

template <typename T>
const T& Vector<T>::operator[](size_t idx) const {
    assert(idx < _used);
    return _data[idx];
}

template <typename T>
size_t Vector<T>::capacity() const noexcept {
    return _reserved;
}

template <typename T>
size_t Vector<T>::size() const noexcept {
    return _used;
}

template <typename T>
void Vector<T>::reserve(size_t reserve) {
    // prevent overflow
    reserve = std::min(reserve, std::numeric_limits<size_t>::max() / GROW_FACTOR);

    while (_reserved < reserve) {
        _reserved *= GROW_FACTOR;
    }
}

template <typename T>
void swap(Vector<T>& lhs, Vector<T>& rhs) {
    lhs.swap(rhs);
}
