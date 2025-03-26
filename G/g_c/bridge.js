// 从编译生成的cjs文件中导入Wasm Module
import Module from './module.cjs'

// 考虑到真实网页场景，通过网络加载wasm可能很慢，所以Module是一个异步函数，
// 而我们是本地的环境，这里直接await就好
const wasm = await Module();
// 使用 cwrap 函数方便的包装 C 中的函数
// 使用方法：cwrap(函数名, 返回值类型, 参数列表)
const c_func = wasm.cwrap('func', 'number', ['number', 'array', 'number']);

// 测试时真正调用的方法
export const bocchiShutUp = (flag, seq, size) => {
  // 由于 seq 这样的 js数组 没有对应的C语言类型，
  // 而C语言的数组入参均表现为指针，所以需要包装一下
  let array = new Uint8Array((new Int32Array(seq)).buffer);
  return c_func(flag, array, size);
};
