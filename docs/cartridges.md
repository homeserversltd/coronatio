# Loadable cartridges

A cartridge turns a web interface on your network into a Coronatio tab. It is useful for dashboards, media servers, download tools, routers, and any other service with a browser interface.

## Add or remove a cartridge

1. Enter admin mode.
2. Select the `+` button in the tab bar.
3. Enter a title and an absolute `http://` or `https://` URL.
4. Optionally mark the tab as admin-only.
5. Select **Add tab**.

The same dialog lists current cartridges and provides a **Remove** button for each one. Both operations require an attended admin session. Coronatio validates the request and proxies it to its privileged actuator, which owns the registry change; Coronatio never writes the registry file directly.

## Registry format

The installed registry is `/etc/appliance/cartridges.json`:

```json
{
  "schema": "appliance.cartridges.v1",
  "cartridges": [
    {
      "id": "jellyfin",
      "title": "jellyfin",
      "url": "https://media.example.net",
      "guest_class": "iframe",
      "admin_only": false
    }
  ]
}
```

The top-level array may also be named `rows`. Coronatio ignores the registry if its schema is different or its JSON is invalid. Within a valid registry, it ignores invalid rows and later rows with duplicate IDs.

For development, `CORONATIO_CARTRIDGE_REGISTRY` selects another registry file.

## Titles, IDs, and URLs

Before submitting the form, the browser computes an ID by lowercasing the title, changing each run of spaces or special characters to `-`, and trimming leading or trailing hyphens. For example, `Media Server!` becomes `media-server` in the submitted request.

The current server performs a stricter final check. It derives the stored ID from the submitted title, removes at most one leading `@`, and otherwise leaves the title unchanged. The result must be non-empty and contain only lowercase ASCII letters, digits, and hyphens. Uppercase letters, spaces, underscores, slashes, and other punctuation are rejected. Until the browser and server normalization rules are aligned, use a lowercase hyphenated title such as `media-server`.

The URL must parse as an absolute URL with an `http` or `https` scheme. Other schemes and malformed URLs are rejected. The cartridge class must be `iframe`.

## Embedding requirement

The target service must allow itself to be framed. If it sends either of these protections, the cartridge will usually render as an empty pane:

- `X-Frame-Options: SAMEORIGIN` or `X-Frame-Options: DENY`
- a Content-Security-Policy with a `frame-ancestors` rule that does not allow the Coronatio origin

The recommended setup is to place the service behind a TLS-terminating reverse proxy and set a Content-Security-Policy that names the Coronatio origin, for example `frame-ancestors 'self' https://coronatio.example.net`. Use this instead of `X-Frame-Options`, which cannot express a second allowed origin. Make the smallest exception that permits your Coronatio origin; do not remove unrelated browser protections.

As a fallback, a service's direct plain-HTTP port works only when Coronatio itself is served over plain HTTP.

### Mixed content and HSTS

When Coronatio is served over HTTPS, browsers refuse to load HTTP iframes inside it as mixed content. Cartridge URLs must then use HTTPS.

If a hostname has previously been visited over HTTPS with HSTS, the browser silently upgrades HTTP URLs for that hostname to HTTPS. If the target port does not support TLS, the pane appears empty and the browser's network log shows an SSL error. Use the HTTPS URL instead.

## How a cartridge is rendered

Selecting a cartridge loads `/admit/<id>`. Coronatio returns an iframe with:

```html
sandbox="allow-scripts allow-same-origin allow-forms"
```

The framed service keeps its own styling and behavior. It will not look like a native Coronatio tab, but it remains available alongside the rest of your appliance controls.