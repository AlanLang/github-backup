# github-backup
github 仓库备份工具

## 用法

```bash
docker pull alanlang/github-backup:latest
docker run -itd --name github-backup -v <local path>:/appuser/backup --env GITHUB_TOKEN=<Your token here> --restart=always github-backup
```