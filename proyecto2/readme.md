## Start

```bash
./deploy.sh
```

## Pending Commands
```bash
kubectl apply -f https://github.com/kubernetes-sigs/metrics-server/releases/latest/download/components.yaml
kubectl get apiservices | grep metrics
kubectl get hpa
```


### To-do
```bash
redis
grafana
prometheus
```