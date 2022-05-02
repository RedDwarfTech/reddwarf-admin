

切换到Alpine镜像后，Kubernetes启动Pod提示错误：

```
Back-off restarting failed container
```

同时提示退出码为139。139一般说明没有常驻进程号为1的进程，也就是说明主应用启动后立即就退出了。在Pod的yaml中添加让容器可以一直运行的指令配置：

```yaml
 command:
       - /bin/ash
       - '-ce'
       - tail -f /dev/null
```

这样容器就不会退出，可以登录进入容器查看进程运行情况。登录后，确实没有找到主进程。手工运行主进程：



```bash
/app # ./reddwarf-admin 
🔧 Configured for release.
   >> address: 0.0.0.0
   >> port: 11015
   >> workers: 12
   >> ident: Rocket
   >> keep-alive: 5s
   >> limits: bytes = 8KiB, data-form = 2MiB, file = 1MiB, form = 32KiB, json = 1MiB, msgpack = 1MiB, string = 8KiB
   >> tls: disabled
   >> temp dir: /tmp
   >> log level: normal
   >> cli colors: true
   >> shutdown: ctrlc = true, force = true, signals = [SIGTERM], grace = 2s, mercy = 3s
🛰  Routes:
   >> (health) GET /actuator/health
   >> (liveness) GET /actuator/liveness
   >> (add) POST /manage/app/v1/add
   >> (page) POST /manage/app/v1/page
   >> (edit) PUT /manage/app/v1/edit
   >> (detail) PUT /manage/app/v1/detail/<id>
   >> (trend_overview) GET /manage/home/v1/trend/overview
   >> (overview) GET /manage/home/v1/dashboard/overview
   >> (add) POST /manage/product/v1/add
   >> (page) POST /manage/product/v1/page
   >> (edit) PUT /manage/product/v1/edit
   >> (get) PUT /manage/product/v1/detail/<id>
   >> (add) POST /manage/app/tags/v1/add
   >> (list) GET /manage/sys/dict/v1/list
   >> (list) POST /manage/app/tags/v1/list
   >> (page) POST /manage/app/tags/v1/page
   >> (edit) PUT /manage/app/tags/v1/edit
   >> (page) POST /manage/app/user/v1/page
   >> (edit_pwd) POST /manage/app/user/v1/pwd/edit
   >> (detail) PUT /manage/app/tags/v1/detail/<id>
Segmentation fault (core dumped)
/app # 
```



主进程运行时提示段错误导致此问题。


















