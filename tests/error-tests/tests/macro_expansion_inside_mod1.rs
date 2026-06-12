#[macro_export]
macro_rules! example_bad_syntax {
    () => {
        enum E {
//           ^ERR(>=1.71.0-beta) while parsing this enum
//           ^MSG(>=1.98.0-beta) See Primary: ↓:12
//      ^^^^HELP(>=1.71.0-beta,<1.98.0-beta) perhaps you meant to use `struct` here
//      ^^^^HELP(>=1.71.0-beta,<1.98.0-beta) /Accept Replacement:.*/
//      ^^^^MSG(>=1.71.0-beta,<1.98.0-beta) See Primary: ↓:12
            // This is somewhat of an odd example, since rustc gives two
            // syntax errors.
            Kind(x: u32)
//                ^ERR /expected one of .*, found `:`/
//                ^ERR(>=1.41.0-beta) /expected one of .* tokens/
//                ^ERR(>=1.18.0,<1.41.0-beta) /expected one of .* here/
//                ^MSG(>=1.20.0) See Also: macro-expansion-inside-1.rs:6
//                ^ERR(<1.34.0-beta) /expected one of .*, found `:`/
//                ^ERR(>=1.18.0,<1.34.0-beta) expected one of
//                ^MSG(>=1.20.0,<1.34.0-beta) See Also: macro-expansion-inside-1.rs:6
//                ^HELP(>=1.71.0-beta) enum variants can be
//                ^MSG(>=1.71.0-beta) See Also: ↑:4
//                ^HELP(>=1.98.0-beta) if you meant to write
//                ^HELP(>=1.98.0-beta) /Accept Replacement:.*/
//              ^HELP(>=1.98.0-beta) if you meant to create
//              ^HELP(>=1.98.0-beta) /Accept Replacement:.*/
//                     ^HELP(>=1.98.0-beta) if you meant to create
//                     ^HELP(>=1.98.0-beta) /Accept Replacement:.*/
        }
    }
}
