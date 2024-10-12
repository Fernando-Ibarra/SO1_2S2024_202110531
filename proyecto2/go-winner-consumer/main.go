package main

import (
	"log"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/confluentinc/confluent-kafka-go/kafka"
)

type KafkaData struct {
	Student    string `json:"student"`
	Age        int64  `json:"age"`
	Faculty    string `json:"faculty"`
	Discipline int64  `json:"discipline"`
	Status     int64  `json:"status"`
}

func getKafkaBrokerURL() string {
	kafkaBrokerURL := "my-cluster-kafka-bootstrap.kafka:9092"
	return kafkaBrokerURL
}

func main() {
	c, err := kafka.NewConsumer(&kafka.ConfigMap{
		"bootstrap.servers": getKafkaBrokerURL(),
		"group.id":          "consumer-default-group",
		"auto.offset.reset": "earliest",
	})

	if err != nil {
		log.Fatalf("Error al crear el consumidor: %s\n", err)
	}

	topic := "winners"
	err = c.Subscribe(topic, nil)
	if err != nil {
		log.Fatalf("Error al suscribirse al tema: %s\n", err)
	}

	sigchan := make(chan os.Signal, 1)
	signal.Notify(sigchan, syscall.SIGINT, syscall.SIGTERM)

	run := true
	for run {

		select {
		case sig := <-sigchan:
			log.Printf("Terminando por señal: %v\n", sig)
			run = false
		default:
			msg, err := c.ReadMessage(100 * time.Millisecond)
			if err != nil {
				continue
			}

			log.Printf("Mensaje recibido: %s\n", string(msg.Value))
		}
	}

	c.Close()

}
