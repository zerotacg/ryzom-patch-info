# ryzom-patch-info

The Docker image uses `/app` as its working directory and runs as a non-root user (`appuser`). To process index files
from your host machine, mount your local directory to /app using the `--volume` flag. The filepath can be relative to
the mounted directory (e.g., `ryzom_01028.idx` or `./ryzom_01028.idx`).

```shell
docker run --rm --volume /path/to/local/dir:/app ghcr.io/zerotacg/ryzom-patch-info:latest --index-file ryzom_[version].idx
```