
## Existing Libraries

https://github.com/nsmryan/ccsds_primary_header exists but has some problems as well as some good parts

Features
- enforcement of the spec
- organizes the data in memory the same way as the protocol specifies
- Uses Enums to provide more end-user-friendly names for different flags and data types values


Anti-features
- likely doesnt support streaming data (as it is a network protocol)
- requires that unsafe code be used to write a packet out to binary
- requires use of names like fields for accessing individual fields
- why are the types limited to arrays of a specific length
- 


## Parsing
### Nom
Nom was initially used as a parser, mostly because it was one of the first ones I could find, but was discontinued because it only seemed to really be able to parse one-way (from bytes to a rust struct). not the other way around

### Serde
Serde seemed attractive, but also seemed like it was for defining more generic, less rigid dataformats that could hold anything (like json) rather than data with a specific bit pattern for specific fields.

### Manually
Ultimately the method that KubOS chose (manual bitshifting) was used because it was inherently limited to parsing data into the specific structs defined by the library (PrimaryHeader) and not any arbitrary struct like `serde` would. This implementation was also already part of base KubOS and just needed to be extracted out and separated from the secondary header stuff