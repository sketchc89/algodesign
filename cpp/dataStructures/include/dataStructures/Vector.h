#pragma once

#include <cstring>

namespace ds {

/*! @brief Resizable array */
template <typename T>
class Vector {
public:
    using ConstIterator = T* const;
    using Iterator = T*;
    /*! @brief Default constructor */
    Vector();
    /*! @brief Construct resizable array of given size zero-initialized
     *  @param size Number of indices to fill with zeros */
    Vector(size_t size);
    /*! @brief Construct resizable array of given size initialized to given value
     *  @param size Number of indices to fill with val
     *  @param val Value to fill Vector with */
    Vector(size_t size, const T& val);
    /*! @brief Copy constructor */
    Vector(const Vector& other);
    /*! @brief Move constructor */
    Vector(Vector&& other) noexcept;
    /*! @brief Copy assignment */
    Vector<T>& operator=(const Vector& other);
    /*! @brief Move assignment */
    Vector<T>& operator=(Vector&& other) noexcept;
    /*! @brief Destructor */
    virtual ~Vector();

    /*! @brief access operator */
    const T& operator[](size_t idx) const;

    /*! @brief append val to the end of Vector
     *  @param val Value to place at end of array */
    void pushBack(const T& val);

    /*! @brief remove last element from end of Vector
     *  @details If vector shrinks by SHRINK_FACTOR then reduce capacity */
    void popBack();

    void swap(Vector& other) noexcept;

    /*! @brief Number of spots reserved in memory for data */
    size_t capacity() const noexcept;
    /*! @brief Number of spots filled with data */
    size_t size() const noexcept;

    /*! @brief Increases capacity of Vector to at least the given value
     *  @details Will at most reserve size_t max / GROW_FACTOR to prevent overflow
     *  @nWill not change capacity if reserve is less than current capacity
     *  @param reserve Vector will reserve more than this many spots
     *  (i.e. will usually reserve some factor of 2 above) */
    void reserve(size_t reserve);

    /*! @brief Shrink or grow Vector to new size
     *  @details If size is smaller than current size then values are truncated
     *  @nIf size is larger than current size then values are filled with default initialized values
     *  @param size New size of Vector */
    void resize(size_t size);

    /*! @brief Does the Vector contain any data. True if empty, false otherwise */
    bool isEmpty() const noexcept;

    Iterator begin() noexcept;
    ConstIterator cBegin() const noexcept;
    Iterator end() noexcept;
    ConstIterator cEnd() const noexcept;

private:
    /* Member functions */
    /*! @brief Increase size of array and copy old data into new larger array */
    void growArray();

    /* Member variables */
    /*! When Vector tries to use more spots than are reserved,
     * Vector will grow by a factor of GROW_FACTOR during append operation */
    static constexpr size_t GROW_FACTOR = 2;
    /*! When Vector is using SHRINK_FACTOR times fewer spots than reserved,
     * Vector will shrink by a factor of GROW_FACTOR during delete operation, */
    static constexpr size_t SHRINK_FACTOR = 4;
    /*! Vector has a minimum number of spots reserved */
    static constexpr size_t MIN_RESERVED = 8;
    /*! Counter of how many Ts are in Vector */
    size_t _used;
    /*! Counter of how many Ts are reserved for Vector in memory */
    size_t _reserved;
    /*! Dynamic array for holding contents of Vector */
    T* _data;
};  // class Vector

}  // namespace ds

namespace std {
/*! @brief swap data from Vector on lhs to rhs and vice-versa
 *  @param lhs Vector
 *  @param rhs Another vector */
template <typename T>
void swap(ds::Vector<T>& lhs, ds::Vector<T>& rhs);
}  // namespace std


// impl
#include "Vector.hpp"
