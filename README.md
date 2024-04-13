# BambangShop Publisher App ⋆𐙚₊˚⊹♡
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [ ✔ ] Commit: `Create Subscriber model struct.`
    -   [ ✔ ] Commit: `Create Notification model struct.`
    -   [ ✔ ] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [ ✔ ] Commit: `Implement add function in Subscriber repository.`
    -   [ ✔ ] Commit: `Implement list_all function in Subscriber repository.`
    -   [ ✔ ] Commit: `Implement delete function in Subscriber repository.`
    -   [ ✔ ] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [ ✔ ] Commit: `Create Notification service struct skeleton.`
    -   [ ✔ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ✔ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ✔ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ✔ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ✔ ] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ✔ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ✔ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ✔ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ✔ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ✔ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

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
1. In the Model-View Controller (MVC) compound pattern, there is no “Service” and “Repository”. Model in MVC covers both data storage and business logic. Explain based on your understanding of design principles, why we need to separate “Service” and “Repository” from a Model?
Ans:

    Separating *Service* and *Repository* from a Model in the MVC pattern improves code organization, maintainability, and testability. It follows the Single Responsibility Principle, allowing each component to focus on a specific concern: the Model represents data and behavior, the Repository handles data storage, and the Service encapsulates business logic. This separation enhances modularity, flexibility, and scalability while reducing complexity and dependencies within the system.

2. What happens if we only use the Model? Explain your imagination on how the interactions between each model (Program, Subscriber, Notification) affect the code complexity for each model?
Ans:

    If we only use the Model without separating concerns into Service and Repository layers, the interactions between each model (Program, Subscriber, Notification) would likely lead to increased code complexity within each model.

    - Program Model:

    In a scenario where the Program model handles both business logic and data storage, it would need to manage interactions with both Subscriber and Notification models directly.
    This could lead to bloated code within the Program model, as it would need to handle not only its own logic but also the logic related to managing subscribers and sending notifications.
    The Program model might become tightly coupled with the Subscriber and Notification models, making it harder to maintain and modify independently.

    - Subscriber Model:

    Similarly, if the Subscriber model handles interactions with the Program and Notification models directly, it could become overwhelmed with responsibilities.
    The Subscriber model might need to handle logic related to subscribing and unsubscribing from programs, as well as receiving and processing notifications.
    This could result in increased code complexity and decreased modularity within the Subscriber model.

    - Notification Model:

    Without a separate Service layer, the Notification model would need to handle both business logic related to sending notifications and data storage concerns.
    Managing interactions with the Program and Subscriber models directly could lead to complex logic within the Notification model, as it tries to coordinate communication between different parts of the system.
    This could make the Notification model difficult to maintain and extend, as changes in other models could have ripple effects on its functionality.

3. Have you explored more about Postman? Tell us how this tool helps you to test your current work. You might want to also list which features in Postman you are interested in or feel like it is helpful to help your Group Project or any of your future software engineering projects.
Ans:

    Yes. Postman allows us to easily send requests to API endpoints and inspect the responses, making it convenient to test different API functionalities, including GET, POST, PUT, DELETE, etc. I also used Postman to create design API for the group project. I feel like the collaboration features will be so helpful for the group project, in which we can share collections, collaboratee on tests, and track changes made by team members. 

#### Reflection Publisher-3
1. Observer Pattern has two variations: Push model (publisher pushes data to subscribers) and Pull model (subscribers pull data from publisher). In this tutorial case, which variation of Observer Pattern that we use?
Ans:

    In this tutorial, we use push model variation of the Observer Pattern which could be shown by this part of code.

    ```
    pub struct NotificationService;

    impl NotificationService {
        pub fn notify(&self, product_type: &str, status: &str, product: Product) {
            let mut payload: Notification = Notification {
                product_title: product.clone().title,
                product_type: String::from(product_type),
                product_url: product.clone().get_url(),
                subscriber_name: String::from(""),
                status: String::from(status)
            };

            let subscribers: Vec<Subscriber> = SubscriberRepository::list_all(product_type);
            for subscriber in subscribers {
                payload.subscriber_name = subscriber.clone().name;
                let subscriber_clone = subscriber.clone();
                let payload_clone = payload.clone();
                thread::spawn(move || subscriber_clone.update(payload_clone));
            }
        }
    }

    ```

    It proactively notifies subscribers by pushing data (notifications) to them without them explicitly requesting it. The notify method iterates over the list of subscribers and pushes the notification payload to each subscriber by calling the update method on each subscriber instance.

2. What are the advantages and disadvantages of using the other variation of Observer Pattern for this tutorial case? (example: if you answer Q1 with Push, then imagine if we used Pull)
Ans:

    Advantages of Pull Model Observer Pattern:
    - Reduced Overhead: With the pull model, subscribers only request data when they need it. This can lead to reduced overhead, especially if there are many subscribers but not all of them need every update.
    - Better Resource Management: Subscribers can control when they request updates, allowing them to optimize resource usage based on their needs. This can be beneficial in scenarios where resources are limited or expensive.
    - Improved Scalability: Since subscribers pull data when needed, the system may be more scalable as it can handle a larger number of subscribers without significant performance degradation.

    Disadvantages of Pull Model Observer Pattern:
    - Latency: In the pull model, subscribers need to periodically poll the publisher for updates. This can introduce latency, especially if updates are infrequent or if subscribers need to poll frequently to ensure they don't miss updates.
    - Complexity: Implementing the pull model can be more complex than the push model, especially when dealing with scenarios like efficient polling intervals, handling missed updates, and managing multiple subscribers efficiently.
    - Increased Network Traffic: Polling for updates can result in increased network traffic, especially if subscribers poll frequently or if updates are small but frequent. This can impact both the publisher's and subscribers' network usage.
    - Potential for Missed Updates: If subscribers don't poll frequently enough or if updates are missed due to network issues or other factors, subscribers may miss important updates, leading to inconsistent data or behavior.

3. Explain what will happen to the program if we decide to not use multi-threading in the notification process.
Ans:

    Without multi-threading, the notification process would execute sequentially in the main thread. This could lead to blocking behavior, increased latency, poor responsiveness, and inefficient resource utilization. Subscribers would have to wait for their turn to receive notifications, potentially causing delays and impacting the overall performance of the application.