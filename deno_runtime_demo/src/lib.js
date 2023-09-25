import { Buffer } from 'node:buffer';

export default {
    add: function (a, b) {
        return a + b;
    },
    base64: function (str) {
        let s = Buffer.from(str, "utf8").toString("base64");
        console.log(`base64: ${s}`);
        return s;
    }
}
