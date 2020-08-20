# Variables

`jen` provides a set of default variables that can be used inside of template files. The following table lists all available variables:

| Name             | Description                                                                                                                                      |
| ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| **arch**         | The system's architecture. For a list of possible values see [here][arch].                                                                       |
| **author_email** | The user's E-Mail. The value depends on the type of project. See [here][author_resolvers] for a detailed description of the discovery process.   |
| **author_name**  | The user's name. The value depends on the type of project. See [here][author_resolvers] for a detailed description of the discovery process.     |
| **current_time** | The current date and time. The format is `2015-06-30 23:59:60.500 <TZ>` where `<TZ>` is the system's timezone.                                   |
| **os_family**    | The system's OS family. For a list of possible values see [here][os_family].                                                                     |
| **os**           | The system's OS. For a list of possible values see [here][os].                                                                                   |
| features.`name`  | A map that can be used to check if a given feature `name` has been enabled. Useful for conditional content that should be included inside a file |

[arch]: https://doc.rust-lang.org/stable/std/env/consts/constant.ARCH.html
[author_resolvers]: ./resolvers.md#author-resolver
[os_family]: https://doc.rust-lang.org/stable/std/env/consts/constant.FAMILY.html
[os]: https://doc.rust-lang.org/stable/std/env/consts/constant.OS.html
