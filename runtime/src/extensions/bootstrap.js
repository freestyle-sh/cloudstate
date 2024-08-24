import * as url from "ext:deno_url/00_url.js";
import "ext:deno_url/01_urlpattern.js";

import "ext:deno_web/00_infra.js";
import "ext:deno_web/01_dom_exception.js";
import "ext:deno_web/01_mimesniff.js";
import "ext:deno_web/02_event.js";
import "ext:deno_web/02_structured_clone.js";
import "ext:deno_web/02_timers.js";
import "ext:deno_web/03_abort_signal.js";
import "ext:deno_web/04_global_interfaces.js";
import "ext:deno_web/05_base64.js";
import "ext:deno_web/06_streams.js";
import "ext:deno_web/08_text_encoding.js";
import "ext:deno_web/09_file.js";
import "ext:deno_web/10_filereader.js";
import "ext:deno_web/12_location.js";
import "ext:deno_web/13_message_port.js";
import "ext:deno_web/14_compression.js";
import "ext:deno_web/15_performance.js";
import "ext:deno_web/16_image_data.js";

import * as crypto from "ext:deno_crypto/00_crypto.js";

Object.defineProperty(globalThis, "URL", {
  value: url.URL,
  enumerable: false,
  configurable: true,
  writable: true,
});

Object.defineProperty(globalThis, "URLPattern", {
  value: url.URLPattern,
  enumerable: false,
  configurable: true,
  writable: true,
});

Object.defineProperty(globalThis, "URLSearchParams", {
  value: url.URLSearchParams,
  enumerable: false,
  configurable: true,
  writable: true,
});

Object.defineProperty(globalThis, "CryptoKey", {
  value: crypto.CryptoKey,
  enumerable: false,
  configurable: true,
  writable: true,
});

Object.defineProperty(globalThis, "crypto", {
  value: crypto.crypto,
  enumerable: false,
  configurable: true,
  writable: false,
});

Object.defineProperty(globalThis, "Crypto", {
  value: crypto.Crypto,
  enumerable: false,
  configurable: true,
  writable: true,
});

Object.defineProperty(globalThis, "SubtleCrypto", {
  value: crypto.SubtleCrypto,
  enumerable: false,
  configurable: true,
  writable: true,
});
