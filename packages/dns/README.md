## @lingze/dns

Use dns server to get record.A with domain.

```js
const {resolveAddr} = require("@lingze/dns");
const result = resolveAddr("www.google.com");

// result = [{record: 'xxx.xx.xx.xx'}]
```