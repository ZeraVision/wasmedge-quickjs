// Copyright Joyent and Node contributors. All rights reserved. MIT license.

'use strict';
import common from '../common';
if (!common.hasCrypto)
  common.skip('missing crypto');

import assert from 'assert';
import crypto from 'crypto';

// 'should consider equal strings to be equal'
assert.strictEqual(
  crypto.timingSafeEqual(Buffer.from('foo'), Buffer.from('foo')),
  true
);

// 'should consider unequal strings to be unequal'
assert.strictEqual(
  crypto.timingSafeEqual(Buffer.from('foo'), Buffer.from('bar')),
  false
);

{
  // Test TypedArrays with different lengths but equal byteLengths.
  const buf = crypto.randomBytes(16).buffer;
  const a1 = new Uint8Array(buf);
  const a2 = new Uint16Array(buf);
  const a3 = new Uint32Array(buf);

  for (const left of [a1, a2, a3]) {
    for (const right of [a1, a2, a3]) {
      assert.strictEqual(crypto.timingSafeEqual(left, right), true);
    }
  }
}

{
  // When the inputs are floating-point numbers, timingSafeEqual neither has
  // equality nor SameValue semantics. It just compares the underlying bytes,
  // ignoring the TypedArray type completely.

  const cmp = (fn) => (a, b) => a.every((x, i) => fn(x, b[i]));
  const eq = cmp((a, b) => a === b);
  const is = cmp(Object.is);

  function test(a, b, { equal, sameValue, timingSafeEqual }) {
    assert.strictEqual(eq(a, b), equal);
    assert.strictEqual(is(a, b), sameValue);
    assert.strictEqual(crypto.timingSafeEqual(a, b), timingSafeEqual);
  }

  test(new Float32Array([NaN]), new Float32Array([NaN]), {
    equal: false,
    sameValue: true,
    timingSafeEqual: true
  });

  test(new Float64Array([0]), new Float64Array([-0]), {
    equal: true,
    sameValue: false,
    timingSafeEqual: false
  });

  const x = new BigInt64Array([0x7ff0000000000001n, 0xfff0000000000001n]);
  test(new Float64Array(x.buffer), new Float64Array([NaN, NaN]), {
    equal: false,
    sameValue: true,
    timingSafeEqual: false
  });
}

assert.throws(
  () => crypto.timingSafeEqual(Buffer.from([1, 2, 3]), Buffer.from([1, 2])),
  {
    code: 'ERR_CRYPTO_TIMING_SAFE_EQUAL_LENGTH',
    name: 'RangeError',
    message: 'Input buffers must have the same byte length'
  }
);

assert.throws(
  () => crypto.timingSafeEqual('not a buffer', Buffer.from([1, 2])),
  {
    code: 'ERR_INVALID_ARG_TYPE',
    name: 'TypeError',
  }
);

assert.throws(
  () => crypto.timingSafeEqual(Buffer.from([1, 2]), 'not a buffer'),
  {
    code: 'ERR_INVALID_ARG_TYPE',
    name: 'TypeError',
  }
);
