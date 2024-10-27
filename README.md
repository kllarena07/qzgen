# qzgen
An easy way to generate QR codes using the web browser.

## How to use
GET `/gen` and then use the query parameter `url` to input a URL.

Example:
```
/gen?url=https://www.google.com
```
This will generate a QR code for `https://www.google.com`.

The QR code will be returned as an image to the web browser where it can be downloaded for distribution.

Note: sharing the link will not mean that you are sharing the same QR code. Generated QR codes are not stored, so a new QR code will be generated for each hit to the API endpoint.

In other words, there can exist different QR codes for `https://www.google.com` at the same time.
