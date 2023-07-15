# TCYSM-bot
## Note
This is a bit messy and build solvely for invite->role mappings. These are no longer used and thus neither will the bot; this repository will be archived in the future in favour of a new and improved bot.
## Invite mapping structure
**Note: currently, this is a JSON file. This will be updated to a proper DB in the future.**  
The persistant invite mappings will have the following structure:
```json
[
{
    "code": "<invite-code>",
    "roles": [
        "role1",
        "role2",
        "..."
    ]
},
{
    "code": "<invite-code>",
    "roles": [
        "role1",
        "role2",
        "..."
    ]
}
    "..."
]
