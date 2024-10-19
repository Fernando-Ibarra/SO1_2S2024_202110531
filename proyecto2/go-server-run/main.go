package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"math/rand"
	"net"

	pb "go-server-run/gen/proto"

	"github.com/confluentinc/confluent-kafka-go/kafka"
	"google.golang.org/grpc"
)

type ApiGrpcServer struct {
	pb.UnimplementedAthleteuideServer
}

type KafkaData struct {
	Student    string
	Age        int64
	Faculty    string
	Discipline int64
	Status     int64
}

func getKafkaBrokerURL() string {
	kafkaBrokerURL := "my-cluster-kafka-bootstrap.kafka:9092"
	return kafkaBrokerURL
}

func randomStatusPlayer() int64 {
	shoot := rand.Intn(2) // 0 representa "cara", 1 representa "cruz"
	if shoot == 0 {
		return 0
	}
	return 1
}

func sendWinner(myData KafkaData) {
	my_json, err := json.Marshal(myData)
	if err != nil {
		log.Fatalf("failed to marshal: %v", err)
	}

	topic := "winners"
	producer, err := kafka.NewProducer(&kafka.ConfigMap{"bootstrap.servers": getKafkaBrokerURL()})
	if err != nil {
		log.Fatalf("failed to create producer: %v", err)
	}

	defer producer.Close()

	deliveryChan := make(chan kafka.Event)

	producer.Produce(&kafka.Message{
		TopicPartition: kafka.TopicPartition{Topic: &topic, Partition: kafka.PartitionAny},
		Value:          my_json,
	}, deliveryChan)

	e := <-deliveryChan
	m := e.(*kafka.Message)

	if m.TopicPartition.Error != nil {
		log.Fatalln("Error al enviar mensaje: ", m.TopicPartition.Error)
	} else {
		fmt.Println("Mensaje enviado a la partición: ", m.TopicPartition)
	}
}

func sendLosser(myData KafkaData) {
	my_json, err := json.Marshal(myData)
	if err != nil {
		log.Fatalf("failed to marshal: %v", err)
	}

	topic := "losers"
	producer, err := kafka.NewProducer(&kafka.ConfigMap{"bootstrap.servers": getKafkaBrokerURL()})
	if err != nil {
		log.Fatalf("failed to create producer: %v", err)
	}

	defer producer.Close()

	deliveryChan := make(chan kafka.Event)

	producer.Produce(&kafka.Message{
		TopicPartition: kafka.TopicPartition{Topic: &topic, Partition: kafka.PartitionAny},
		Value:          my_json,
	}, deliveryChan)

	e := <-deliveryChan
	m := e.(*kafka.Message)

	if m.TopicPartition.Error != nil {
		log.Fatalln("Error al enviar mensaje: ", m.TopicPartition.Error)
	} else {
		fmt.Println("Mensaje enviado a la partición: ", m.TopicPartition)
	}
}

func (s *ApiGrpcServer) CreateAthlete(ctx context.Context, req *pb.AthleteRequest) (*pb.AthleteResponse, error) {
	msg := pb.AthleteResponse{Student: req.Student}
	log.Printf("Received")
	playerStatus := randomStatusPlayer()
	dataPlayer := KafkaData{
		Student:    req.Student,
		Age:        req.Age,
		Faculty:    req.Faculty,
		Discipline: req.Discipline,
		Status:     playerStatus,
	}
	if playerStatus == 0 {
		log.Printf("Ganador: %v", req.Student)
		go sendWinner(dataPlayer)
		return &msg, nil
	}

	log.Printf("Perdedor: %v", req.Student)
	go sendLosser(dataPlayer)
	return &msg, nil
}

func main() {
	listen, err := net.Listen("tcp", "0.0.0.0:3002")
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}
	grpcServer := grpc.NewServer()
	pb.RegisterAthleteuideServer(grpcServer, &ApiGrpcServer{})
	log.Println("gRPC server is running on port 3002")
	err = grpcServer.Serve(listen)
	if err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
