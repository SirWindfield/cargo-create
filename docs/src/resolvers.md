# Variable Resolver

Resolvers are used to find the value of a given variable during runtime. `jen` does implement a set of default resolvers that populate templates containing variables listed at [here][variables].

## Author Resolver

The author resolver works by checking different environment variables to get the value for the user's name and E-Mail address. The following variables are resolved in the order they are listed inside the table:

| author_email        | author_name        |
| ------------------- | ------------------ |
| CARGO_EMAIL         | CARGO_NAME         |
| GIT_AUTHOR_EMAIL    | GIT_AUTHOR_NAME    |
| GIT_COMMITTER_EMAIL | GIT_COMMITTER_NAME |
| EMAIL               | USER               |

[variables]: ./variables.md
