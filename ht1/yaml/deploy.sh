#!/bin/bash

echo "Aplicando los archivos YAML de Kubernetes..."

echp "Aplicando Ingress..."
kubectl apply -f ingress.yml
kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/main/deploy/static/provider/cloud/deploy.yaml

echo "Aplicando Server"
kubectl apply -f go-deployment.yml
kubectl apply -f go-service.yml