## Defense

To prevent these type of attacks all user input should be thoroughly scrutinized.  If the input does not meet the input validation requirements it should be rejected, sanitized, or filtered before being used anywhere else in the application.

One of the most common ways to implement input validation is with the use of Data type validators that are available in most modern web application frameworks, which are written for a variety of languages.

Each framework  differs slightly in how they implement these guards/validators, such as the use of the `Validators` module when using the `actix-web` framework.  In this specific case, the app is using `Rocket` as the Rust based web application framework.

The main learning goal:  **Knowing what input you are trying to validate and what specific guard will be appropriate.**

The function in the previous code is making a GET request.  Using this information you can navigate to the documentation of the framework and search for what will work best. Knowing that you are making a request, simply type "request" into the API documentation search bar and view the result.  As you look at the `rocket::request` module you see a trait, which takes a dynamic path string and converts it into a concrete value - `FromParam`.

This will defend you against the exploit, by validating the id before it is used.

First import the necessary module functionality into the application.

```rust
use rocket::request::FromParam;
```

Using the documentation make note of the required methods and associated types.

```rust
impl<'a> FromParam<'a> for PasteId<'a>; {
    type Error = &'a str;
    
    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        param.chars().all(|c| c.is_ascii_alphanumeric())
        	.then(|| PasteId(param.into()))
        	.ok_or(param)
    }
}
```

The rest of code simply uses the appropriate closures to check that the values are ASCII alphanumeric characters for `PasteId`.

*Further Learning:*

The use of `FromParam` can also be used to add a Whitelisting and/or blacklisting feature to the application which provides even more protection.