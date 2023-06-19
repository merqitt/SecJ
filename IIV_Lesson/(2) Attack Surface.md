## Attack Surface

This vulnerability can occur across a wide array of implementations and applications.  It can manifests itself in different ways depending on code complexity, network structure, and conformance to the specified business logic.

While the severity of this issue varies broadly from case to case, the most prevalent danger comes from the ability to use this vulnerability to cause greater damage across the organization, by linking it to other attacks such as directory traversal or even remote code execution.

A great example of how prolific the vulnerability is comes from the exploit of Apple's macOS and iOS operating systems in 2021, which did not properly validate input and allowed for a maliciously crafted PDF file (input) to be accepted and lead to arbitrary code execution.

To be able to properly validate our data we need to create properties that guard our application and in which to validate against your application.  Depending on the environment and its complexity these properties can be different.  For the coding exercise your concerns will be directly related to validating specific types.  The properties to keep in mind throughout the training and in our later code example center around the various forms of user input.