### Description
This application allows to impose CPU load on a computer. For example,
```bash
stress --cpu 4 --timeout 30
```

### Building docker image of your application

First, build your image, e.g.: `docker build -t stress .`.
If your cloud uses a different CPU architecture than your development
machine (e.g., you are on a Mac M1 and your cloud provider is amd64),
you'll want to build the image for that platform, e.g.:
`docker build --platform=linux/amd64 -t stress .`.

Then, push it to your registry, e.g. `docker push myregistry.com/stress`.