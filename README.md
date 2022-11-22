# github-backup
github 仓库备份工具

## 用法

```bash
docker pull alanlang/github-backup:latest
docker run -itd --name github-backup -v <local path>:/appuser/backup --env GITHUB_TOKEN=<Your token here> --restart=always github-backup
```

## GITHUB_TOKEN 获取
Settings -> Developer settings -> Personal access tokens -> Generate new token

![20221122181312@2x.png](https://vip2.loli.io/2022/11/22/AhtiyE94rlcSwa6.png)