# Filters

`jen` comes with a set of buildin filters alongside those that [Tera][Tera] [already provides][Tera-filters].

| Name       | Description                                              |
| ---------- | -------------------------------------------------------- |
| camel_case | Converts text to camel case: `some text` -> `SomeText`.  |
| kebab_case | Alias for [slugify][Tera-slugify].                       |
| lower_case | Alias for [lower][Tera-lower].                           |
| mixed_case | Converts text to mixed case: `Some Text` -> `someText`.  |
| snake_case | Converts text to snake case: `Some text` -> `some_text`. |
| title_case | Alias for [title][Tera-title].                           |
| upper_case | Alias for [upper][Tera-upper].                           |

[Tera]: https://github.com/Keats/tera
[Tera-filters]: https://tera.netlify.app/docs/#built-in-filters
[Tera-slugify]: https://tera.netlify.app/docs/#slugify
[Tera-lower]: https://tera.netlify.app/docs/#lower
[Tera-title]: https://tera.netlify.app/docs/#title
[Tera-upper]: https://tera.netlify.app/docs/#built-in-filters
