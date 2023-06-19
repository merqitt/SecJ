## Reconnaissance

There are different techniques that can be used for detecting this type of vulnerability.  Tools such as Web Application Vulnerability Scanners (i.e. AppScan, Nessus, etc.) or targeted manual code review.

In this example you will concentrate on manual review so that you are better able expand your mental model.  This security mindset will allow us to concentrate on specific parts of the code allowing for more effective and efficient software development.

You should particularly pay close attention to the route and handler functionality as they direct the flow of data and how it is manipulated.  When looking for instances of improper input validation it is beneficial to look through various different forms of user input, these can come from a myriad of sources such as-

**input fields, cookies, query strings, and headers**

As you look over the App code in the next module - what is the input that we should start with?

*Query strings*

Unfortunately even in "safe" languages like Rust this is not enough to prevent an exploit.