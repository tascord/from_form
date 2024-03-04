#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use from_form::*;

    #[derive(Debug)]
    struct Email(String);
    impl ToString for Email {
        fn to_string(&self) -> String {
            self.0.clone()
        }
    }
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

    #[derive(Debug)]
    struct Username(String);
    impl ToString for Username {
        fn to_string(&self) -> String {
            self.0.clone()
        }
    }
    impl TryFrom<String> for Username {
        type Error = String;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            if value.len() >= 3 {
                Ok(Username(value))
            } else {
                Err("Invalid username".to_string())
            }
        }
    }

    #[derive(Debug)]
    struct Password(String);
    impl ToString for Password {
        fn to_string(&self) -> String {
            self.0.clone()
        }
    }
    impl TryFrom<String> for Password {
        type Error = String;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            if value.len() >= 8 {
                Ok(Password(value))
            } else {
                Err("Invalid password".to_string())
            }
        }
    }

    #[allow(dead_code)]
    #[derive(FromForm, Debug)]
    struct Signup {
        email: Email,
        #[rename("handle")]
        username: Username,
        password: Password,
    }

    #[test]
    fn it_works() {
        let mut form_data = HashMap::<String,String>::new();
        form_data.insert("email".to_string(), "imflo.pink@gmail.com".to_string());
        form_data.insert("handle".to_string(), "imflo".to_string());
        form_data.insert("password".to_string(), "password".to_string());

        let signup = Signup::try_from(form_data);
        assert!(signup.is_ok());

        println!("{:?}", signup.unwrap());
    }
}
