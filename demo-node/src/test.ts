import test from 'node:test'
import assert from 'assert'
import RustLib from '@rsbind/node_rustlib'
import DemoTestStruct from '@rsbind/node_rustlib/dist/DemoTestStruct';

test('test add', (t) => {
    const rustLib = new RustLib();
    const demo = rustLib.newDemo();
    assert.strictEqual(100, demo.testI8100(100, 101));
    assert.strictEqual(100, demo.testI8100(100.6, 101.6));
    assert.strictEqual(100, demo.testU8100(100, 101));
    assert.strictEqual(100, demo.testU8100(100.5, 101.5));
    assert.strictEqual(100, demo.testI16100(100, 101));
    assert.strictEqual(100, demo.testU16100(100, 101));
    assert.strictEqual(100, demo.testI32100(100, 101));
    assert.strictEqual(100, demo.testU32100(100, 101));
    assert.strictEqual(100, demo.testU64100(100, 101));
    assert.strictEqual(100, demo.testU64100(100, 101));
    assert.strictEqual(100.0, demo.testF32100(100.0, 101.0));
    assert.strictEqual(100.0, demo.testF64100(100.0, 101.0));
    assert.strictEqual(true, demo.testBooleanTrue(true, false));
    assert.strictEqual("Hello", demo.testStringHello("Hello"));
    assert.deepStrictEqual([2, 2], demo.testI8Array([2, 2]));
    assert.deepStrictEqual([2, 2], demo.testI8Array([2.1, 2.1]));
    assert.deepStrictEqual([2, 2], demo.testI16Array([2, 2]));
    assert.deepStrictEqual([2, 2], demo.testI32Array([2, 2]));
    assert.deepStrictEqual([2, 2], demo.testI64Array([2, 2]));
    assert.deepStrictEqual([2, 2], demo.testU8Array([2, 2]));
    assert.deepStrictEqual([2, 2], demo.testU8Array([2.1, 2.1]));
    assert.deepStrictEqual([2, 2], demo.testU16Array([2, 2]));
    assert.deepStrictEqual([2, 2], demo.testU32Array([2, 2]));
    assert.deepStrictEqual([2, 2], demo.testU64Array([2, 2]));
    assert.deepStrictEqual([true, true], demo.testBoolTrueArray([true, true]));
    assert.deepStrictEqual([false, false], demo.testBoolFalseArray([false, false]));
    assert.deepStrictEqual(createStruct(), demo.testStruct(createStruct()));
});

function createStruct(): DemoTestStruct {
    return {
        i8_1: 1,
        u8_2: 2,
        i16_3: 3,
        u16_4: 4,
        i32_5: 5,
        u32_6: 6,
        i64_7: 7,
        u64_8: 8,
        f32_9: 9,
        f64_10: 10,
        bool_true: true,
        str_hello: "Hello"
    }
}

