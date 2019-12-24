#pragma once

#include <experimental/optional>
#include <memory>

namespace ds {

template <typename T>
struct Node {
    T value;
    Node<T>* next;
};

/*! @brief Linked List */
template <typename T>
class List {
public:
    /*! @brief empty new list */
    List();
    List(const List& other) = default;
    List(List&& other) = default;
    List& operator=(const List& other) = default;
    List& operator=(List&& other) = default;
    virtual ~List();
    std::experimental::optional<T> head() const noexcept;
    void swap(List& other) noexcept;
    void pushBack(const T& value);
    void popBack();

private:
    Node<T>* _head;
};  // class List


}  // namespace ds

// template <typename T>
// void std::swap(ds::List<T>& lhs, ds::List<T>& rhs) noexcept;
//
// template <typename T>
// void std::swap(ds::Node<T>& lhs, ds::Node<T>& rhs) noexcept;

#include "List.hpp"
