#pragma once

namespace ds {

/*! @brief Resizable array */
template <typename T>
class Vector {
public:
    /*! @brief Default constructor */
    Vector();
    /*! @brief Construct resizable array of given size zero-initialized */
    Vector(size_t size);
    /*! @brief Construct resizable array of given size initialized to given value
     */
    Vector(size_t size, const T& val);

    /*! @brief access operator */
    const T& operator[](size_t idx) const;

private:
    T _array[0];
    size_t _used;
    size_t _reserved;
};

// impl
#include "Vector.hpp"

}  // namespace ds
