namespace ds {

template <typename T>
List<T>::List() : _head{nullptr} {}

template <typename T>
Node<T>::Node(T elem) : _elem{elem}, _next{nullptr} {}

}  // namespace ds
