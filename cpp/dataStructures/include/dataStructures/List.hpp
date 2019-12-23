namespace ds {

template <typename T>
List<T>::List() : _head{nullptr} {}

template <typename T>
Node<T>::Node(T elem) : _elem{elem}, _next{nullptr} {}

template <typename T>
std::experimental::optional<T> List<T>::head() const noexcept {
    std::experimental::optional<T> result;
    if (_head != nullptr) {
        result = _head->value();
    }
    return result;
}

template <typename T>
void List<T>::pushBack(const T& elem) {
    _head = std::make_unique<Node<T>>(elem);
}

template <typename T>
T Node<T>::value() const noexcept {
    return _elem;
}

}  // namespace ds
