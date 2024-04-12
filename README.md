# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---
### Mandatory (Publisher) Reflections

#### Reflection Publisher-1
1. In the Observer pattern diagram explained by the Head First Design Pattern book, Subscriber is defined as an interface. Explain based on your understanding of Observer design patterns, do we still need an interface (or trait in Rust) in this BambangShop case, or a single Model struct is enough?
Ans:

In the Observer design pattern, the Subscriber represents the observers who are interested in being notified of changes in the state of the subject (or publisher). While it's common to define Subscriber as an interface in many implementations of the Observer pattern, but not strictly necessary. 

In BambangShop, I think single model struct for subscribers is sufficient. Here are the reasons why:

- SubscriberRepository manages subscribers for different product types, but there is *no indication that different types of subscribers require different behaviors.* All subscribers are handled uniformly within the repository without any need for polymorphism or dynamic behavior switching.
- SubscriberRepository *directly handles subscriber objects without any need for an interface or trait to define a common contract.* Subscribers are added, listed, and deleted using concrete methods specific to the repository, which suggests that a single model struct is enough to represent subscribers.

2. id in Program and url in Subscriber is intended to be unique. Explain based on your understanding, is using Vec (list) sufficient or using DashMap (map/dictionary) like we currently use is necessary for this case?
Ans:

I think it using DashMap like the current implementation is better than just using Vec. A DashMap provides a key-value mapping where each key is unique. This ensures that duplicate identifiers are not allowed. If we need to frequently check for the existence of an identifier or perform lookups by identifier, a DashMap would provide more efficient access. Retrieving an element from a DashMap has an average time complexity of O(1) compared to Vec which has a time complexity of O(n), where n is the number of elements in the list. Additionally, DashMap allows for easy deletion of elements by key, which might be useful if you need to remove subscribers or programs by their unique identifiers.

3. When programming using Rust, we are enforced by rigorous compiler constraints to make a thread-safe program. In the case of the List of Subscribers (SUBSCRIBERS) static variable, we used the DashMap external library for thread safe HashMap. Explain based on your understanding of design patterns, do we still need DashMap or we can implement Singleton pattern instead?
Ans:

The Singleton pattern can be used to ensure that only one instance of a particular object exists in memory, however it does not inherently provide thread safety. Plus in a multi-threaded environment, if multiple threads attempt to access or modify the Singleton instance concurrently without proper synchronization, it can lead to data races and undefined behavior. So, I think using DashMap or similar thread-safe data structures is still necessary to ensure proper synchronization and thread safety when dealing with shared state across multiple threads in Rust.

#### Reflection Publisher-2

#### Reflection Publisher-3
