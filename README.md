# reddwarf-admin



## 为什么用rust

reddwarf-admin是用rust实现的后台接口。后台接口最初的实现版本是用Java，但是Java实现的后台接口存在如下问题：

* 资源占用高。这是最核心的痛点，就目前的情况，无法投入更多的硬件资源。一个Java进程需要500MB左右的内存和1Core CPU，意味着2GB内存的主机启动2个Java进程资源就已经捉襟见肘。而且经常会存在CPU资源不足导致Pod无法启动。虽然替换成[OpenJ9](https://www.eclipse.org/openj9/)节省了20%-30%内存，还是未从根本上解决问题。

* 启动慢。这也是[Quarkus](https://quarkus.io/)要解决的问题，不过从老项目迁移到Quarkus没有想象的平滑。

因为目前后端没有复杂的逻辑，接口数量不多，遂尝试用rust重写了接口。新的后端Pod资源配置如下：

```yaml
resources:
   limits:
      cpu: 50m
      memory: 60Mi
   requests:
      cpu: 20m
      memory: 6Mi
```

初始分配内存6MB，初始分配CPU 0.02核心。解决了最核心的痛点，但是rust目前还不具备强大的生态，比如和Apollo配置中心集成的client库，和注册中心交互的库等等。即使有对应的库，维护也不稳定，适合尝试不适合生产。



## 开始开发

```bash
git clone https://github.com/jiangxiaoqiang/reddwarf-admin.git
# 编译
cargo build
```
