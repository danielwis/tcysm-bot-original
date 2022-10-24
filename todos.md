1. Find out how to generate invites as well as tracking who joins with which invite.
2. Verify users through KTH e-mail addresses?
3. Figure out how to utilise a db together with the bot. What data needs to be stored persistently?
    1. Verifications? DiscordUser -> email
    2. Messages to keep track of?
    3. Invite links / how many uses per link to know which role to assign them to
        - Example below, store as JSON temporarily then migrate when it scales up
4. Add reaction roles or other things with `message.reaction_users`.

```json
invites = {
    invite-id: {
        roles: {
           role1,
           role2,
        },
        uses: n
    },
}
```
