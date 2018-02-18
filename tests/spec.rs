/*
Copyright 2016 Mozilla
Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.
*/

#![cfg_attr(feature = "cargo-clippy", allow(blacklisted_name))]
#![feature(proc_macro)]

extern crate enum_str_derive;

use enum_str_derive::{EnumStrCamelCase, EnumStrKebabCase, EnumStrMixedCase};

#[derive(EnumStrCamelCase)]
pub enum Unit {
    Foo,
    BarBaz
}

#[derive(EnumStrKebabCase)]
pub enum Tuple {
    Foo(String),
    BarBaz(Vec<String>)
}

#[derive(EnumStrMixedCase)]
pub enum Struct {
    Foo { value: String },
    BarBaz { value: Vec<String> }
}

#[test]
fn test_1() {
    let foo = Unit::Foo;
    assert_eq!(foo.as_ref(), "Foo");

    let bar = Unit::BarBaz;
    assert_eq!(bar.as_ref(), "BarBaz");
}

#[test]
fn test_2() {
    let foo = Tuple::Foo("heck".to_string());
    assert_eq!(foo.as_ref(), "foo");

    let bar = Tuple::BarBaz(vec!["heck".to_string()]);
    assert_eq!(bar.as_ref(), "bar-baz");
}

#[test]
fn test_3() {
    let foo = Struct::Foo {
        value: "heck".to_string()
    };
    assert_eq!(foo.as_ref(), "foo");

    let bar = Struct::BarBaz {
        value: vec!["heck".to_string()]
    };
    assert_eq!(bar.as_ref(), "barBaz");
}
