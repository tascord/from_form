#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
#[cfg(test)]
mod tests {
    use std::{collections::HashMap, default};
    use from_form::*;
    struct Email(String);
    #[automatically_derived]
    impl ::core::fmt::Debug for Email {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Email", &&self.0)
        }
    }
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
    struct Username(String);
    #[automatically_derived]
    impl ::core::fmt::Debug for Username {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Username", &&self.0)
        }
    }
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
    struct Password(String);
    #[automatically_derived]
    impl ::core::fmt::Debug for Password {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Password", &&self.0)
        }
    }
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
    struct Signup {
        email: Email,
        #[rename("handle")]
        username: Username,
        password: Password,
        secret: String,
    }
    impl TryFrom<std::collections::HashMap<String, String>> for Signup {
        type Error = String;
        fn try_from(
            form_data: std::collections::HashMap<String, String>,
        ) -> Result<Self, Self::Error> {
            Ok(Signup {
                email: <Email>::try_from(
                        form_data
                            .get("email")
                            .ok_or_else(|| {
                                let res = ::alloc::fmt::format(
                                    format_args!("{0} not found", "email"),
                                );
                                res
                            })?
                            .to_string(),
                    )
                    .map_err(|e| {
                        let res = ::alloc::fmt::format(format_args!("{0:?}", e));
                        res
                    })?,
                username: <Username>::try_from(
                        form_data
                            .get("handle")
                            .ok_or_else(|| {
                                let res = ::alloc::fmt::format(
                                    format_args!("{0} not found", "handle"),
                                );
                                res
                            })?
                            .to_string(),
                    )
                    .map_err(|e| {
                        let res = ::alloc::fmt::format(format_args!("{0:?}", e));
                        res
                    })?,
                password: <Password>::try_from(
                        form_data
                            .get("password")
                            .ok_or_else(|| {
                                let res = ::alloc::fmt::format(
                                    format_args!("{0} not found", "password"),
                                );
                                res
                            })?
                            .to_string(),
                    )
                    .map_err(|e| {
                        let res = ::alloc::fmt::format(format_args!("{0:?}", e));
                        res
                    })?,
                secret: <String>::try_from(
                        form_data
                            .get("secret")
                            .ok_or_else(|| {
                                let res = ::alloc::fmt::format(
                                    format_args!("{0} not found", "secret"),
                                );
                                res
                            })?
                            .to_string(),
                    )
                    .map_err(|e| {
                        let res = ::alloc::fmt::format(format_args!("{0:?}", e));
                        res
                    })?,
            })
        }
    }
    #[automatically_derived]
    #[allow(dead_code)]
    impl ::core::fmt::Debug for Signup {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Signup",
                "email",
                &self.email,
                "username",
                &self.username,
                "password",
                &self.password,
                "secret",
                &&self.secret,
            )
        }
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::it_works"]
    pub const it_works: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::it_works"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/index.rs",
            start_line: 72usize,
            start_col: 8usize,
            end_line: 72usize,
            end_col: 16usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(it_works()),
        ),
    };
    fn it_works() {
        let mut form_data = HashMap::<String, String>::new();
        form_data.insert("email".to_string(), "imflo.pink@gmail.com".to_string());
        form_data.insert("handle".to_string(), "imflo".to_string());
        form_data.insert("password".to_string(), "password".to_string());
        form_data.insert("secret".to_string(), "secret".to_string());
        let signup = Signup::try_from(form_data);
        if !signup.is_ok() {
            ::core::panicking::panic("assertion failed: signup.is_ok()")
        }
        {
            ::std::io::_print(format_args!("{0:?}\n", signup.unwrap()));
        };
    }
    #[allow(dead_code)]
    struct ComplexStruct {
        field_1: Option<String>,
        field_2: Option<String>,
    }
    impl TryFrom<std::collections::HashMap<String, String>> for ComplexStruct {
        type Error = String;
        fn try_from(
            form_data: std::collections::HashMap<String, String>,
        ) -> Result<Self, Self::Error> {
            Ok(ComplexStruct {
                field_1: <Option<
                    String,
                >>::try_from(
                        form_data
                            .get("field_1")
                            .ok_or_else(|| {
                                let res = ::alloc::fmt::format(
                                    format_args!("{0} not found", "field_1"),
                                );
                                res
                            })?
                            .to_string(),
                    )
                    .map_err(|e| {
                        let res = ::alloc::fmt::format(format_args!("{0:?}", e));
                        res
                    })?,
                field_2: <Option<
                    String,
                >>::try_from(
                        form_data
                            .get("field_2")
                            .ok_or_else(|| {
                                let res = ::alloc::fmt::format(
                                    format_args!("{0} not found", "field_2"),
                                );
                                res
                            })?
                            .to_string(),
                    )
                    .map_err(|e| {
                        let res = ::alloc::fmt::format(format_args!("{0:?}", e));
                        res
                    })?,
            })
        }
    }
    #[automatically_derived]
    #[allow(dead_code)]
    impl ::core::fmt::Debug for ComplexStruct {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "ComplexStruct",
                "field_1",
                &self.field_1,
                "field_2",
                &&self.field_2,
            )
        }
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::complex"]
    pub const complex: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::complex"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/index.rs",
            start_line: 93usize,
            start_col: 8usize,
            end_line: 93usize,
            end_col: 15usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(complex()),
        ),
    };
    fn complex() {
        let mut form_data = HashMap::<String, String>::new();
        form_data.insert("field_1".to_string(), "abc".to_string());
        form_data.insert("field_2".to_string(), "abc".to_string());
        let signup = Signup::try_from(form_data);
        if !signup.is_ok() {
            ::core::panicking::panic("assertion failed: signup.is_ok()")
        }
        {
            ::std::io::_print(format_args!("{0:?}\n", signup.unwrap()));
        };
    }
}
#[rustc_main]
#[coverage(off)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&complex, &it_works])
}
