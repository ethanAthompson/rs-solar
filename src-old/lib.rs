#![allow(warnings)]

pub mod earth;
pub mod mars;
pub mod shared;

/// The standard formatting for earth dates
pub static DATEFORMAT: &str = "%m/%d/%Y %r %Z";

/*
    What is the purpose of rust_solar?
    ```markdown

        > The purpose of this library is to prove my research,
            about the solar system.
    ```

    How do you use rust_solar?
    ```markdown

        > You use rust solar to synchronize earth and space datetimes
        * You can use rust_solar in leptos, yew, anywhere that works.
    ```

    Features
    ```markdown
        * Datetime Conversions
            -[ ] Mars (Inner Planet)
            -[ ] Saturn (Outer Planet)
            -[ ] Titan (Moon)
            -[ ] 2 Vesta (Asteroid)
            -[ ] Haileys (Comet)
            -[x] Earth (Default)

        * Calendar Support
            -[x] Gregorian
            -[x] ISO
            -[x] Indian
            -[x] Chinese

        * Iterators
            -[x] Tz_Variants : All earth timezones
            -[ ] Ctz_Variants : All space locations
    ```
*/
