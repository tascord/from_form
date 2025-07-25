#[cfg(test)]
mod tests {
    use std::{collections::HashMap, sync::Arc};

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
    #[ff(crate = "from_form")]
    struct Signup {
        email: Email,
        #[rename("handle")]
        username: Username,
        password: Password,
        secret: String,
    }

    #[test]
    fn it_works() {
        let mut form_data = HashMap::<String, String>::new();
        form_data.insert("email".to_string(), "imflo.pink@gmail.com".to_string());
        form_data.insert("handle".to_string(), "imflo".to_string());
        form_data.insert("password".to_string(), "password".to_string());
        form_data.insert("secret".to_string(), "secret".to_string());

        let signup = Signup::try_from(form_data);
        assert!(signup.is_ok());
        assert_eq!(Signup::COLUMNS.len(), 4);

        println!("{:?}", signup.unwrap());
    }

    #[test]
    fn arc_str() {
        let mut form_data: HashMap<Arc<str>, Arc<str>> = HashMap::<Arc<str>, Arc<str>>::new();
        form_data.insert("email".into(), "imflo.pink@gmail.com".into());
        form_data.insert("handle".into(), "imflo".into());
        form_data.insert("password".into(), "password".into());
        form_data.insert("secret".into(), "secret".into());

        let signup = Signup::try_from(form_data);
        assert!(signup.is_ok());
        assert_eq!(Signup::COLUMNS.len(), 4);

        println!("{:?}", signup.unwrap());
    }

    #[allow(dead_code)]
    #[derive(FromForm, Debug)]
    struct ComplexStruct {
        field_1: Option<String>,
        #[doc = "blah blah"]
        field_2: Option<String>,
    }

    #[test]
    fn complex() {
        let mut form_data = HashMap::<String, String>::new();
        form_data.insert("field_1".to_string(), "abc".to_string());
        form_data.insert("field_2".to_string(), "abc".to_string());

        let signup = ComplexStruct::try_from(form_data);
        assert!(signup.is_ok());

        println!("{:?}", signup.unwrap());
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    struct FStrType(i8);
    impl std::str::FromStr for FStrType {
        type Err = <i8 as std::str::FromStr>::Err;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            i8::from_str(s).map(FStrType)
        }
    }

    #[allow(dead_code)]
    #[derive(FromForm, Debug)]
    struct StrTest {
        #[from_str]
        field: FStrType,
    }
}
