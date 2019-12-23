#pragma once

#include <experimental/optional>
#include <memory>

namespace ds {

template <typename T>
class Node;

template <typename T>
using Link = std::unique_ptr<Node<T>>;

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
    virtual ~List() = default;
    void swap(List& other) noexcept;
    std::experimental::optional<T> head() const noexcept;
    void pushBack(const T& elem);

private:
    Link<T> _head;
};  // class List

template <typename T>
class Node {
public:
    Node(T elem);
    Node(const Node& other);
    Node(Node&& other);
    Node& operator=(const Node& other) = default;
    Node& operator=(Node&& other) = default;
    virtual ~Node();
    void swap(Node& other);

private:
    T _elem;
    Link<T> _next;
};

}  // namespace ds

// template <typename T>
// void std::swap(ds::List<T>& lhs, ds::List<T>& rhs) noexcept;
//
// template <typename T>
// void std::swap(ds::Node<T>& lhs, ds::Node<T>& rhs) noexcept;

#include "List.hpp"
