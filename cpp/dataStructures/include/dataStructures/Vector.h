#pragma once

#include <memory>

namespace ds {

/*! @brief Resizable array */
template <typename T>
class Vector {
public:
    /*! When Vector tries to use more spots than are reserved,
     * Vector will grow by a factor of GROW_FACTOR during append operation */
    static constexpr size_t GROW_FACTOR = 2;
    /*! When Vector is using SHRINK_FACTOR times fewer spots than reserved,
     * Vector will shrink by a factor of GROW_FACTOR during delete operation, */
    static constexpr size_t SHRINK_FACTOR = 4;
    /*! Vector has a minimum number of spots reserved */
    static constexpr size_t MIN_RESERVED = 8;

    /*! @brief Default constructor */
    Vector();
    /*! @brief Construct resizable array of given size zero-initialized
     *  @param size Number of indices to fill with zeros */
    Vector(size_t size);
    /*! @brief Construct resizable array of given size initialized to given value
     *  @param size Number of indices to fill with val
     *  @param val Value to fill Vector with */
    Vector(size_t size, const T& val);

    /*! @brief access operator */
    const T& operator[](size_t idx) const;

    /*! @brief append val to the end of Vector
     *  @param val Value to place at end of array */
    void pushBack(const T& val);

private:
    /*! Dynamic array for holding contents of Vector */
    T* _data;
    /*! Counter of how many Ts are in Vector */
    size_t _used;
    /*! Counter of how many Ts are reserved for Vector in memory */
    size_t _reserved;
};

// impl
#include "Vector.hpp"

}  // namespace ds
