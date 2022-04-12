Hi bashbunni viewers. Sorry for the shitty project :) Wrote it in a week. You guys don't have credentials to use the api, but swagger docs are here: https://g3t.ca/api/v1/swagger

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
