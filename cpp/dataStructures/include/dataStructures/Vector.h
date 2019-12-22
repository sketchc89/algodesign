#pragma once

namespace ds {

/*! @brief Resizable array */
template <typename T> class Vector<T> {
public:
  /*! @brief Default constructor */
  Vector();
  /*! @brief Construct resizable array of given size zero-initialized */
  Vector(size_t size);
  /*! @brief Construct resizable array of given size initialized to given value
   */
  Vector(size_t size, T val);

private:
};

} // namespace ds
