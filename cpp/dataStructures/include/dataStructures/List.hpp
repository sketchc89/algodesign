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

}  // namespace ds
