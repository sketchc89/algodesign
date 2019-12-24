#include <iostream>

namespace ds {

template <typename T>
List<T>::List() : _head{nullptr} {}

template <typename T>
List<T>::~List() {
    Node<T>* nextNode = _head;
    Node<T>* currentNode = nullptr;
    while (nextNode != nullptr) {
        currentNode = nextNode;
        nextNode = nextNode->next;
        delete currentNode;
    }
}

template <typename T>
std::experimental::optional<T> List<T>::head() const noexcept {
    std::experimental::optional<T> res;
    if (_head != nullptr) {
        res = _head->value;
    }
    return res;
}

template <typename T>
void List<T>::pushBack(const T& value) {
    auto node = new Node<T>;
    node->value = value;
    node->next = _head;
    _head = node;
}

template <typename T>
void List<T>::popBack() {
    if (head()) {
        auto oldHead = _head;
        _head = _head->next;
        delete oldHead;
    }
}

template <typename T>
void List<T>::swap(List& other) noexcept {
    Node<T>* temp = _head;
    _head = other._head;
    other._head = temp;
}

template <typename T>
std::experimental::optional<T> List<T>::find(const T& val) const noexcept {
    std::experimental::optional<T> res;
    Node<T> const* currentNode = _head;
    while (currentNode != nullptr) {
        auto& v = currentNode->value;
        if (v == val) {
            res = v;
            return res;
        }
        currentNode = currentNode->next;
    }

    return res;
}

template <typename T>
bool List<T>::remove(const T& val) {
    Node<T>* currentNode = _head;
    Node<T>* prevNode = nullptr;

    while (currentNode != nullptr) {
        if (currentNode->value == val) {
            if (prevNode == nullptr) {
                // indicates value found at first position and head needs to be updated
                _head = nullptr;
            } else {
                // otherwise link previous node to node after node to be removed
                prevNode->next = currentNode->next;
            }
            delete currentNode;
            return true;
        }
        prevNode = currentNode;
        currentNode = currentNode->next;
    }
    return false;
}

}  // namespace ds

namespace std {
template <typename T>
void swap(ds::List<T>& lhs, ds::List<T>& rhs) noexcept {
    lhs.swap(rhs);
}

}  // namespace std
