# URL Linker

Simple URL Shortening & Aliasing

Dynamically link URLs with custom names for shortening or improved readability.

## Example

When authenticated, the following request:

`POST https://g3t.ca/api/v1/urls`
```json
{
  "key": "rick",
  "url": "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
}
```

Will create the following link:

[g3t.ca/rick](https://g3t.ca/rick)
