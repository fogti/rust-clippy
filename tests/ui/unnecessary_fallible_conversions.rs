#![warn(clippy::unnecessary_fallible_conversions)]

use core::str::FromStr;

fn main() {
    // --- TryFromMethod `T::try_from(u)` ---

    let _: i64 = 0i32.try_into().unwrap();
    //~^ unnecessary_fallible_conversions

    let _: i64 = 0i32.try_into().expect("can't happen");
    //~^ unnecessary_fallible_conversions

    // --- TryFromFunction `T::try_from(U)` ---

    let _ = i64::try_from(0i32).unwrap();
    //~^ unnecessary_fallible_conversions

    let _ = i64::try_from(0i32).expect("can't happen");
    //~^ unnecessary_fallible_conversions

    // --- TryIntoFunction `U::try_into(t)` ---

    let _: i64 = i32::try_into(0).unwrap();
    //~^ unnecessary_fallible_conversions

    let _: i64 = i32::try_into(0i32).expect("can't happen");
    //~^ unnecessary_fallible_conversions

    // --- TryFromFunction `<T as TryFrom<U>>::try_from(U)` ---

    let _ = <i64 as TryFrom<i32>>::try_from(0).unwrap();
    //~^ unnecessary_fallible_conversions

    let _ = <i64 as TryFrom<i32>>::try_from(0).expect("can't happen");
    //~^ unnecessary_fallible_conversions

    // --- TryIntoFunction `<U as TryInto<_>>::try_into(U)` ---

    let _: i64 = <i32 as TryInto<_>>::try_into(0).unwrap();
    //~^ unnecessary_fallible_conversions

    let _: i64 = <i32 as TryInto<_>>::try_into(0).expect("can't happen");
    //~^ unnecessary_fallible_conversions

    // --- FromStrFunction `<T as FromStr>::from_str(U)` ---`

    let _ = String::from_str("").unwrap();
    //~^ unnecessary_fallible_conversions

    let _ = String::from_str("").expect("can't happen");
    //~^ unnecessary_fallible_conversions

    // --- ParseMethod `U.parse::<T>()` ---

    let _: String = "".parse().unwrap();
    //~^ unnecessary_fallible_conversions

    let _: String = "".parse().expect("can't happen");
    //~^ unnecessary_fallible_conversions
}
