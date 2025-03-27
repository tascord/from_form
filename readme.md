<div align="center">
    <h2>🗂️ FromForm 🦀</h2>
    <div>
    <a href="https://crates.io/crates/from-form"><img alt="Crates.io Version" src="https://img.shields.io/crates/v/from-form?style=for-the-badge"></a>
    <a href="https://docs.rs/from-form"><img alt="docs.rs" src="https://img.shields.io/docsrs/from-form?style=for-the-badge"></a>    
    </div>
</div>

### Go from Map<K, V> data to T
```rs
use from_form::*;

#[derive(FromForm, Debug)]
struct Signup {
    email: Email,
    #[rename("handle")]
    username: String,
    password: String,
    secret: String,
}

#[derive(Debug)]
struct Email(String);
impl ToString for Email {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

// NewType for custom parsing rules
impl TryFrom<String> for Email {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.contains('@') {
            Ok(Email(value))
        } else {
            Err("Invalid email".to_string())
        }
    }
}

let mut form_data = HashMap::<String, String>::new();
form_data.insert("email".to_string(), "dog@imflo.pet".to_string()); // <- Uses custom parsing
form_data.insert("handle".to_string(), "imflo".to_string()); // <- Alias'
form_data.insert("password".to_string(), "password".to_string());
form_data.insert("secret".to_string(), "secret".to_string());

let signup = Signup::try_from(form_data); // <- It works!

```

### Things of note
- If your type impliments `FromStr` but not `TryFrom<String>`, use the `#[from_str]` attr on the field.
- The columns of your struct are available through `T::COLUMNS`