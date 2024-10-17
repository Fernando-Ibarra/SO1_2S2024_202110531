package main

import (
	"context"
	"log"
	"net/http"

	pb "go-server/gen/proto"

	"github.com/gin-gonic/gin"
	"google.golang.org/grpc"
)

type athlete struct {
	Student    string `json:"student"`
	Age        int64  `json:"age"`
	Faculty    string `json:"faculty"`
	Discipline int64  `json:"discipline"`
}

func grpcGoSwimming(newAthlete athlete) {
	conn, err := grpc.NewClient("go-service-swim:3001", grpc.WithInsecure())
	if err != nil {
		log.Fatalf("failed to connect: %v", err)
	}
	client := pb.NewAthleteuideClient(conn)

	resp, err := client.CreateAthlete(context.Background(), &pb.AthleteRequest{Student: newAthlete.Student, Age: newAthlete.Age, Faculty: newAthlete.Faculty, Discipline: newAthlete.Discipline})
	if err != nil {
		log.Fatalf("failed to call: %v", err)
	}
	log.Printf("Response: %s", resp.Student)
	defer conn.Close()
}

func grpcGoRunning(newAthlete athlete) {
	conn, err := grpc.NewClient("go-service-run:3002", grpc.WithInsecure())
	if err != nil {
		log.Fatalf("failed to connect: %v", err)
	}
	client := pb.NewAthleteuideClient(conn)

	resp, err := client.CreateAthlete(context.Background(), &pb.AthleteRequest{Student: newAthlete.Student, Age: newAthlete.Age, Faculty: newAthlete.Faculty, Discipline: newAthlete.Discipline})
	if err != nil {
		log.Fatalf("failed to call: %v", err)
	}
	log.Printf("Response: %s", resp.Student)
	defer conn.Close()
}

func grpcGoBoxing(newAthlete athlete) {
	conn, err := grpc.NewClient("go-service-box:3003", grpc.WithInsecure())
	if err != nil {
		log.Fatalf("failed to connect: %v", err)
	}
	client := pb.NewAthleteuideClient(conn)

	resp, err := client.CreateAthlete(context.Background(), &pb.AthleteRequest{Student: newAthlete.Student, Age: newAthlete.Age, Faculty: newAthlete.Faculty, Discipline: newAthlete.Discipline})
	if err != nil {
		log.Fatalf("failed to call: %v", err)
	}
	log.Printf("Response: %s", resp.Student)
	defer conn.Close()
}

func postAthlete(c *gin.Context) {
	var newAthlete athlete
	if err := c.BindJSON(&newAthlete); err != nil {
		return
	}
	c.IndentedJSON(http.StatusCreated, newAthlete)

	switch newAthlete.Discipline {
	case 1:
		go grpcGoSwimming(newAthlete)
	case 2:
		go grpcGoRunning(newAthlete)
	case 3:
		go grpcGoBoxing(newAthlete)
	}
}

func main() {
	router := gin.Default()
	router.POST("/", postAthlete)
	router.Run("0.0.0.0:8080")
}
