## Invite mapping structure
### Currently, a JSON file. Will be updated to a proper DB in the future.
The persistant invite mappings will have the following structure:
```json
[
    invite_code1: [
        role1,
        role2,
        ...
    ],
    invite_code2: [
        role1
    ],
    ...
]
```

On startup, we want to do the following:
- Read this file into a vector of `InviteRoles`
- Get the currently active invites from the Discord API
- For each `InviteRoles` struct we have locally
    - Check if the invite code is active
    - If it is:
        - Update the cached map to contain these roles
    - If it isn't:
        - Remove this entry from the vector.
- Serialise the new vector and write it back to file
