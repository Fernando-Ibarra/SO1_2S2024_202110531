package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/confluentinc/confluent-kafka-go/kafka"
	"github.com/redis/go-redis/v9"
)

var ctx = context.Background()

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

	redis_db := redis.NewClient(&redis.Options{
		Addr:     "redis-service:6379",
		Password: "M1R3D1SP4SSW0RD", // no password set
		DB:       0,                 // use default DB
	})

	_, err = redis_db.Ping(ctx).Result()
	if err != nil {
		log.Fatalf("Error al conectar a redis: %s\n", err)
	}

	fmt.Println("Conectado a la redis")

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

			vote := KafkaData{}
			err = json.Unmarshal(msg.Value, &vote)

			if redis_db != nil {

				err = redis_db.HIncrBy(ctx, "votos_alumnos", vote.Faculty, 1).Err()
				if err != nil {
					log.Printf("Error al incrementar votos por album: %s\n", err)
				}

				if vote.Discipline == 1 {
					err = redis_db.HIncrBy(ctx, "votos_disciplinas", "swim", 1).Err()
					if err != nil {
						log.Printf("Error al incrementar votos por album: %s\n", err)
					}
				} else if vote.Discipline == 2 {
					err = redis_db.HIncrBy(ctx, "votos_disciplinas", "run", 1).Err()
					if err != nil {
						log.Printf("Error al incrementar votos por album: %s\n", err)
					}
				} else if vote.Discipline == 3 {
					err = redis_db.HIncrBy(ctx, "votos_disciplinas", "box", 1).Err()
					if err != nil {
						log.Printf("Error al incrementar votos por album: %s\n", err)
					}
				}
			} else {
				log.Printf("Error al incrementar votos: %s\n", err)
			}

			log.Printf("Mensaje recibido: %s\n", string(msg.Value))
		}
	}

	c.Close()

}
