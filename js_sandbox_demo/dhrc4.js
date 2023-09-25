

TextEncoder = function TextEncoder() { };

TextEncoder.prototype.encode = function (str) {
    var Len = str.length, resPos = -1;
    // The Uint8Array's length must be at least 3x the length of the string because an invalid UTF-16
    //  takes up the equivelent space of 3 UTF-8 characters to encode it properly. However, Array's
    // have an auto expanding length and 1.5x should be just the right balance for most uses.
    var resArr = typeof Uint8Array === 'undefined' ? new Array(Len * 1.5) : new Uint8Array(Len * 3);
    for (var point = 0, nextcode = 0, i = 0; i !== Len;) {
        point = str.charCodeAt(i), i += 1;
        if (point >= 0xD800 && point <= 0xDBFF) {
            if (i === Len) {
                resArr[resPos += 1] = 0xef/*0b11101111*/; resArr[resPos += 1] = 0xbf/*0b10111111*/;
                resArr[resPos += 1] = 0xbd/*0b10111101*/; break;
            }
            // https://mathiasbynens.be/notes/javascript-encoding#surrogate-formulae
            nextcode = str.charCodeAt(i);
            if (nextcode >= 0xDC00 && nextcode <= 0xDFFF) {
                point = (point - 0xD800) * 0x400 + nextcode - 0xDC00 + 0x10000;
                i += 1;
                if (point > 0xffff) {
                    resArr[resPos += 1] = (0x1e/*0b11110*/ << 3) | (point >>> 18);
                    resArr[resPos += 1] = (0x2/*0b10*/ << 6) | ((point >>> 12) & 0x3f/*0b00111111*/);
                    resArr[resPos += 1] = (0x2/*0b10*/ << 6) | ((point >>> 6) & 0x3f/*0b00111111*/);
                    resArr[resPos += 1] = (0x2/*0b10*/ << 6) | (point & 0x3f/*0b00111111*/);
                    continue;
                }
            } else {
                resArr[resPos += 1] = 0xef/*0b11101111*/; resArr[resPos += 1] = 0xbf/*0b10111111*/;
                resArr[resPos += 1] = 0xbd/*0b10111101*/; continue;
            }
        }
        if (point <= 0x007f) {
            resArr[resPos += 1] = (0x0/*0b0*/ << 7) | point;
        } else if (point <= 0x07ff) {
            resArr[resPos += 1] = (0x6/*0b110*/ << 5) | (point >>> 6);
            resArr[resPos += 1] = (0x2/*0b10*/ << 6) | (point & 0x3f/*0b00111111*/);
        } else {
            resArr[resPos += 1] = (0xe/*0b1110*/ << 4) | (point >>> 12);
            resArr[resPos += 1] = (0x2/*0b10*/ << 6) | ((point >>> 6) & 0x3f/*0b00111111*/);
            resArr[resPos += 1] = (0x2/*0b10*/ << 6) | (point & 0x3f/*0b00111111*/);
        }
    }
    if (typeof Uint8Array !== 'undefined') return resArr.subarray(0, resPos + 1);
    // else // IE 6-9
    resArr.length = resPos + 1; // trim off extra weight
    return resArr;
};


const Q = 37;
const A = 35;

function calc(g, n, p) {
    return g ^ n % p;
}

// function gen_keys() {
//     let pri = "";
//     let pub = "";
//     for (let i = 0; i < 36; ++i) {
//         const priSub = Math.floor(Math.random() * Q);
//         const pubSub = calc(A, priSub, Q);
//         pri += String.fromCharCode(priSub);
//         pub += String.fromCharCode(pubSub);
//     }
//     pri = new Buffer(pri, "utf8").toString("base64");
//     pub = new Buffer(pub, "utf8").toString("base64");
//     return {
//         public_key: pub,
//         private_key: pri
//     };
// }

function gen_keys() {

    let pri = "";
    let pub = "";

    for (let i = 0; i < 36; ++i) {

        const priSub = Math.floor(Math.random() * Q);
        const pubSub = calc(A, priSub, Q);

        pri += String.fromCharCode(priSub);
        pub += String.fromCharCode(pubSub);

    }

    // 编码为 base64
    const priBytes = new TextEncoder().encode(pri);
    const pubBytes = new TextEncoder().encode(pub);

    const priKey = btoa(priBytes);
    const pubKey = btoa(pubBytes);

    return {
        public_key: pubKey,
        private_key: priKey
    }

}

// function secret(pri_key, other_pub) {
//     let secret = "";
//     const pri = new Buffer(pri_key, "base64").toString("utf8");
//     const pub = new Buffer(other_pub, "base64").toString("utf8");
//     for (let i = 0; i < 36; ++i) {
//         const r = pri.charCodeAt(i);
//         const b = pub.charCodeAt(i);
//         const s = calc(b, r, Q);
//         secret += String.fromCharCode(s);
//     }
//     secret = new Buffer(secret, "utf8").toString("base64");
//     return secret;
// }

function secret(pri_key, other_pub) {

    // base64解码为ArrayBuffer
    const priBytes = atob(pri_key);
    const priBuffer = new ArrayBuffer(priBytes.length);
    const priView = new Uint8Array(priBuffer);
    for (let i = 0; i < priBytes.length; i++) {
        priView[i] = priBytes.charCodeAt(i);
    }

    const pubBytes = atob(other_pub);
    const pubBuffer = new ArrayBuffer(pubBytes.length);
    const pubView = new Uint8Array(pubBuffer);
    for (let i = 0; i < pubBytes.length; i++) {
        pubView[i] = pubBytes.charCodeAt(i);
    }

    // 计算共享密钥
    let secret = '';
    for (let i = 0; i < 36; ++i) {
        const r = priView[i];
        const b = pubView[i];
        const s = calc(b, r, Q);
        secret += String.fromCharCode(s);
    }

    // 编码为base64
    const encoded = btoa(secret);

    return encoded;

}

function bin2hex(bin) {
    let i = 0, l = bin.length, chr, hex = "";
    for (i; i < l; ++i) {
        chr = bin.charCodeAt(i).toString(16);
        hex += chr.length < 2 ? "0" + chr : chr;
    }
    return hex;
}

function hex2bin(hex) {
    let bytes = [];
    for (let i = 0; i < hex.length - 1; i += 2) {
        bytes.push(parseInt(hex.substring(i, 2), 16));
    }
    return String.fromCharCode.apply(String, bytes);
}

function rc4(str, key) {
    let i;
    let s = [], j = 0, x, res = "";
    for (i = 0; i < 256; i++) {
        s[i] = i;
    }
    for (i = 0; i < 256; i++) {
        j = (j + s[i] + key.charCodeAt(i % key.length)) % 256;
        x = s[i];
        s[i] = s[j];
        s[j] = x;
    }
    i = 0;
    j = 0;
    for (let y = 0; y < str.length; y++) {
        i = (i + 1) % 256;
        j = (j + s[i]) % 256;
        x = s[i];
        s[i] = s[j];
        s[j] = x;
        res += String.fromCharCode(str.charCodeAt(y) ^ s[(s[i] + s[j]) % 256]);
    }
    return res;
}

// function hex2b64(data) {
//     return new Buffer(data, "hex").toString("base64");
// }

function hex2b64(data) {
    // 将 hex 字符串转换为 ArrayBuffer
    const buffer = new ArrayBuffer(data.length / 2);
    const view = new Uint8Array(buffer);
    for (let i = 0; i < data.length; i += 2) {
        view[i / 2] = parseInt(data.substring(i, i + 2), 16);
    }
    // 将 ArrayBuffer 编码为 base64
    const encoded = btoa(String.fromCharCode(...view));
    return encoded;
}

// function b642hex(data) {
//     return new Buffer(data, "base64").toString("hex");
// }

function b642hex(data) {

    // base64解码为ArrayBuffer
    const bytes = atob(data);
    const buffer = new ArrayBuffer(bytes.length);
    const view = new Uint8Array(buffer);
    for (let i = 0; i < bytes.length; i++) {
        view[i] = bytes.charCodeAt(i);
    }

    // 转换为hex
    let hex = '';
    for (let byte of view) {
        hex += byte.toString(16).padStart(2, '0');
    }

    return hex;

}

function rc4encryption(data, key) {
    return hex2b64(bin2hex(rc4(data, key)));
}

function rc4decryption(data, key) {
    return rc4(hex2bin(b642hex(data)), key);
}

// export default {
//     gen_key: gen_keys,
//     secret: secret,
//     encrypto: rc4encryption,
//     decrypto: rc4decryption
// };
