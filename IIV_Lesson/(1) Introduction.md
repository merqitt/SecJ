## Insufficient Input Validation

Input validation is a crucial part of the Software Development Life Cycle (SDLC) and is essential when it comes to securing your web application.

Before moving into the hands-on portion of the lesson we will begin with some background information on input validation strategies and how to apply those rules in a Rust context.

Insufficient Input validation occurs when the application receives and processes data, but \*\*does not\*\* validate or incorrectly validates the path, format, and data requirements that the program expects. When this occurs  - a malicious actor has a large attack surface to enumerate and carry out full path disclosure attacks.  This can lead to unintended consequences and information disclosure

You will be using a simple App that retrieves files - for this module.  These GET request will gives us the ability to inspect and measure the effectiveness of our code and validation rules.

If you are new to Rust - [link](https://www.securityjourney.com/appsec-training-library)
