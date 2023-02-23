# ky

ky 是使用redis传输协议, 使用多线程并发的内存数据库, 目前没有写入日志功能.

## 性能

ky在运行redis-benchmark
![ky_redis_benchmark](image/使用redis-benchmark测试ky.png)

ky在运行redis-benchmark下的cpu和内存消耗
![ky_cpu](image/ky的cpu和内存.png)

redis在运行redis-benchmark
![redis_banchmark](image/使用redis-benchmark测试redis-server.png)

redis在运行redis-benchmark下的cpu和内存消耗
![redis_cpu](image/redis-server的cpu和内存.png)
