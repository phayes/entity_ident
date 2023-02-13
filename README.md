# EntityIdent
Efficient entity identifiers for public APIs


Example:

 - `user_C3M2XCLwa3LjkkH4V15muQS4lzI`
 - `post_dDsDU9KAREyrb4i64V15mXCLwa3`

Breakdown:

```
user                _             C3M2XCLwa3LjkkH4V15muQS4lzI
Entity Identifier   Seperator     Random Bytes
```

1. Entity Identifier: limited to 8 characters in the range `[a-bA-B0-9]`
2. `_` seperator
3. 128 bits of random bytes. Nominally base62 encoded, but is not encoded or decoded during normal operation. 

Features:

1. URL Safe. Filename Safe.
2. Easily identifies what type of entity is being referenced.
3. Leaks no information about the entity other than it's type.
4. Stateless generation. 128 bits of cryptographically-secure random data ensures effectively no probabability of collision.
5. Very efficient. 
   - Limited to 31 bytes, allowing for a 32 byte in-memory representation with a 1-byte niche for recording length.
   - Stack allocated
