# Tripod ID
Distributable user-friendly id.

## Examples

- `123` : shortest version
- `456-789` : default size, still user freindly and sufficient randomness (for personal data)
- `abc-def-ghj` : long version. alphabets except i, l and o are also valid　
## Specs
### Characters



## Perpose
When I considering implementing IDs for users(not for internal system) to specify items, such as GitHub commit hashes or issue numbers, the following issues arose.

- Sequential numbers like Git issues are difficult to implement in distributes systems because collitions are unavoidable.
- Random number like UUID is too long for users
- Short random number like 7-digit commit hash seems good but is is not standardized specification.

So I decided to make my own ID specifications.

