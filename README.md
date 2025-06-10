# SVG to PNG Cloudflare Worker

![SVG to PNG Converter](https://user-images.githubusercontent.com/33700526/207815865-9b471652-5723-4d35-8847-dce0fb9701eb.png)

SVG to PNG converter in Cloudflare Workers with support for both URL-based and direct SVG content conversion.

## Installation

### Windows Specific
- Install [Strawberry Perl](https://strawberryperl.com/)

### All OS
- Install [Rust](https://www.rust-lang.org/tools/install)
- Install [Cloudflare Wrangler](https://developers.cloudflare.com/workers/cli-wrangler/install-update)
- `wrangler login`
- Create a Cloudflare worker with name: `svg-to-png`
- `wrangler dev` to local test
- `wrangler publish` to publish to Cloudflare

## Usage

### GET Request (URL-based)
Convert SVG from a URL by appending it to the worker endpoint:

```
https://svg-to-png.mrproper.dev/{SVG_URL}
```

**Demo**: https://svg-to-png.mrproper.dev/https://docs.tandoor.dev/logo_color.svg

### POST Request

You can make a POST request with either an SVG URL or direct SVG content in the request body.

#### Option 1: Convert from URL

Send a JSON object containing the SVG URL:

```bash
curl -X POST \
  -H "Content-Type: application/json" \
  -d '{"url": "https://docs.tandoor.dev/logo_color.svg"}' \
  https://svg-to-png.mrproper.dev
```

#### Option 2: Convert from SVG Content

Send a JSON object containing the SVG content directly:

```bash
curl -X POST \
  -H "Content-Type: application/json" \
  -d '{"svg": "<svg width=\"100\" height=\"100\"><circle cx=\"50\" cy=\"50\" r=\"40\" fill=\"red\"/></svg>"}' \
  https://svg-to-png.mrproper.dev
```

### Request Body Schema

The POST request body should be a JSON object with one of the following fields:

```json
{
  "url": "https://example.com/image.svg"  // SVG URL to fetch and convert
}
```

**OR**

```json
{
  "svg": "<svg>...</svg>"  // Direct SVG content to convert
}
```

### Response

All successful requests return a PNG image with the following headers:
- `Content-Type: image/png`
- `Cache-Control: public, max-age=3600`

### Error Handling

The worker returns appropriate HTTP status codes and error messages:
- `400 Bad Request`: When neither `url` nor `svg` is provided in POST requests
- `405 Method Not Allowed`: For unsupported HTTP methods
- `500 Internal Server Error`: For processing errors (invalid SVG, network issues, etc.)

## Features

- ✅ Convert SVG from URL (GET and POST)
- ✅ Convert SVG from direct content (POST)
- ✅ Automatic caching headers
- ✅ Comprehensive error handling
- ✅ Support for various SVG formats
- ✅ Fast processing with Rust and WASM
