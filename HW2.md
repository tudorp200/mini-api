[Homework 2] Create an application (backend, frontend) that uses at least 3 Web services. One of the services will be the RESTful Web service, created in Homework 1. The information will be presented in a graphical Web interface. For development, it is mandatory to use at least one framework.
Additional Information:
the application should have client-side and server-side, which means that a web interface is required;
the communication logic with the three Web services will be implemented in the backend component of the application;
you need to handle the errors and exception cases that may appear in your application while integrating with the Web services;
General observations:
In the evaluation process, the complexity of the used APIs (the remaining two services) will be taken into consideration;
The used APIs should require minimal authentication using an API key;
Any form of secret (e.g. email password, secret key) should not be hardcoded in code. Try to at least store it in a configuration file;
The homework can be implemented in any programming language


Additional Information:

You MUST use at least one framework for this homework, both for frontend and backend.
Your backend should communicate with the frontend using either JSON or XML; backend and frontend are two decoupled components; is it not allowed to respond with HTML and CSS from your backend; 
Your backend should perform requests to the three web services, according to the business logic you design, in order to be able to provide various functionalities to the client;
Make sure your backend is able to handle errors or exceptions that may be thrown by the Web services;
Make sure your backend responds with the corresponding status codes for each situation it reaches in the processing of a request ( ex. 200 - Ok, 404 - Not Found, 500 - Internal Server Error etc.)
