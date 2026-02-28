# 测试不同异步运行执行器执行执行1ms定时任务的抖动

## 执行器

1. async_executor
2. futures
3. tokio
4. global_async_executor （底层使用`async_executor` block_on 方法）

## 测试流程

1. `loop_until_end_count` 函数每隔1ms将计数器自增1,计数器使用原子变量封装
2. `monitor` 函数每隔1s检查计数器是否自增1000次
3. 记录超过1000次的最大次数,少于1000次最少次数，以及总共不等于1000次的次数

## 结果

执行3次，每次持续1分钟左右

- async_executor
  
```shell
[async_executor] max upper jitter: 0 ticks (≈0μs), max lower jitter: 0 ticks (≈0μs), offset count: 0 in 60000 ticks

[async_executor] max upper jitter: 0 ticks (≈0μs), max lower jitter: 0 ticks (≈0μs), offset count: 0 in 60000 ticks

[async_executor] max upper jitter: 0 ticks (≈0μs), max lower jitter: 0 ticks (≈0μs), offset count: 0 in 60000 ticks
```

- tokio
  
```shell
[tokio excutor] max upper jitter: 1 ticks (≈1000μs), max lower jitter: 1 ticks (≈1000μs), offset count: 20 in 60000 ticks

[tokio excutor] max upper jitter: 1 ticks (≈1000μs), max lower jitter: 3 ticks (≈3000μs), offset count: 19 in 60997 ticks

[tokio excutor] max upper jitter: 1 ticks (≈1000μs), max lower jitter: 1 ticks (≈1000μs), offset count: 18 in 60000 ticks
```

- futures executor

```shell
[futures] max upper jitter: 1 ticks (≈1000μs), max lower jitter: 4 ticks (≈4000μs), offset count: 21 in 60996 ticks

[futures] max upper jitter: 1 ticks (≈1000μs), max lower jitter: 1 ticks (≈1000μs), offset count: 22 in 60000 ticks

[futures] max upper jitter: 1 ticks (≈1000μs), max lower jitter: 1 ticks (≈1000μs), offset count: 16 in 60000 ticks
```

- global async executor

```shell
[global_async_executor] max upper jitter: 0 ticks (≈0μs), max lower jitter: 0 ticks (≈0μs), offset count: 0 in 60000 ticks

[global_async_executor] max upper jitter: 0 ticks (≈0μs), max lower jitter: 0 ticks (≈0μs), offset count: 0 in 60000 ticks

[global_async_executor] max upper jitter: 0 ticks (≈0μs), max lower jitter: 0 ticks (≈0μs), offset count: 0 in 60000 ticks
```
