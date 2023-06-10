// At the moment, the `string_data_item` is being parsed and converted from MUTF-8 bytes to a `String
// This is being done as I don't expect errors with MUTF-8 to String conversion to be common
// if this turns out to be false, I'll add an `enum StringDataItem` that will just store `bytes` on
// an error.
