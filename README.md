# TCYSM-bot
## Invite mapping structure
**Note: currently, this is a JSON file. This will be updated to a proper DB in the future.**  
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
