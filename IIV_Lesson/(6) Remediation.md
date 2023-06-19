---
title: (6) Remediation
updated: 2023-06-19 01:19:45Z
created: 2023-06-18 17:21:52Z
latitude: 35.77958970
longitude: -78.63817870
altitude: 0.0000
---

As part of the overall validation strategy your validator is in place and now you can refactor this specific code to take advantage of the trait on `PasteId`

To remediate the code we will simply replace the vulnerable retrieve function

```rust
#[get("/<id>")]
async fn retrieve(id: PasteId<'_>;) -> Option<File> {
    File::open(id.file_path()).await.ok()
}
```

From defense section where we implemented the validator we can now remove the the two middle lines from the original code and instead of a string literal we use the trait defined for `PasteId`.  

Now if you attempt to access the file with the same query string you will be returned a 404 error.