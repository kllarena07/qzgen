# qzgen
An easy way to generate QR codes using the web browser.

## Running project locally
1. Clone the repo
```
git clone https://github.com/kllarena07/qzgen.git
```
2. Run Dockerfile
```
cd qzgen
docker run -d -p 8080:8080 qzgen
```
3. Create a QR Code by visiting `127.0.0.1:8080`. See instructions below on how to generate a QR code

## How to use
GET `/gen` and then use the query parameter `url` to input a URL.

Example:
```
/gen?url=https://www.google.com
```
This will generate a QR code for `https://www.google.com`.

The QR code will be returned as an image to the web browser where it can be downloaded for distribution.

## Note and Disclaimer
Sharing the link will not mean that you are sharing the same QR code. Generated QR codes are not stored, so a new QR code will be generated for each hit to the API endpoint.

In other words, there can exist different QR codes for `https://www.google.com` at the same time.
