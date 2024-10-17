#!/bin/bash

echo "Aplicando los archivos YAML de Kubernetes..."

echo "Aplicando Ingress..."
kubectl create ns nginx-ingress
kubectl apply -f ingress.yml
kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/main/deploy/static/provider/cloud/deploy.yaml

echo "Aplicando kafka..."
kubectl create namespace kafka
kubectl create -f 'https://strimzi.io/install/latest?namespace=kafka' -n kafka
kubectl apply -f strimzi.yml -n kafka
kubectl apply -f kafka.yml

echo "Aplicando redis..."
kubectl apply -f redis.yml
kubectl create secret generic redis-secret --from-literal=url=redis-service:6379
kubectl create secret generic redis-password-secret --from-literal=password=M1R3D1SP4SSW0RD

echo "Aplicando los servicios..."
kubectl apply -f go-deployment.yml
kubectl apply -f rust-deployment.yml

echo "Aplicando Horizontal Pod Autoscalers (HPA)..."
kubectl apply -f go-hpa.yml
kubectl apply -f rust-hpa.yml

echo "Aplicando los servicios..."
kubectl apply -f go-swim-deployment.yml
kubectl apply -f go-run-deployment.yml
kubectl apply -f go-box-deployment.yml

echo "Aplicando los servicios..."
kubectl apply -f go-winner-deployment.yml
kubectl apply -f go-loser-deployment.yml

echo "Aplicando grafana..."
kubectl apply -f grafana.yml

echo "Aplicando prometheus..."
helm repo add prometheus-community https://prometheus-community.github.io/helm-charts
helm repo update
helm install prometheus prometheus-community/prometheus