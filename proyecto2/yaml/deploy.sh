#!/bin/bash

echo "Aplicando los archivos YAML de Kubernetes..."

echo "Aplicando Ingress..."
kubectl apply -f ingress.yml
kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/main/deploy/static/provider/cloud/deploy.yaml

echo "Aplicando los servicios..."
kubectl apply -f go-deployment.yml
kubectl apply -f rust-deployment.yml

echo "Aplicando los servicios..."
kubectl apply -f go-swim-deployment.yml
kubectl apply -f go-run-deployment.yml
kubectl apply -f go-box-deployment.yml

echo "Aplicando kafka..."
kubectl create namespace kafka
kubectl create -f 'https://strimzi.io/install/latest?namespace=kafka' -n kafka
kubectl apply -f https://strimzi.io/examples/latest/kafka/kafka-persistent-single.yaml -n kafka
kubectl apply -f kafka.yml

echo "Aplicando los servicios..."
kubectl apply -f go-winner-deployment.yml
kubectl apply -f go-loser-deployment.yml

echo "Aplicando Horizontal Pod Autoscalers (HPA)..."
kubectl apply -f go-hpa.yml
kubectl apply -f rust-hpa.yml