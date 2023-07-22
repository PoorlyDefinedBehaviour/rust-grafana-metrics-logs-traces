### Running Grafana agent

[Docs](https://grafana.com/docs/agent/latest/static/set-up/install/install-agent-docker/)

- `-v $PWD/grafana-agent/agent:/etc/agent` maps the folder where the `agent.yml` file is in the host to to the folder where the agent expects it to be.
- `-v $PWD/log:/var/log/app` maps the folder where the log files are in the host to the folder where the agent expects it to be.

```
docker run \
  -v $PWD/grafana-agent/data:/etc/agent/data \
  -v $PWD/grafana-agent/agent.yaml:/etc/agent/agent.yaml \
  -v $PWD/log:/var/log/app \
  -p 4317:4317 \
  grafana/agent:v0.35.0
```

