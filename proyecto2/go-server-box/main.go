package main

import (
	"context"
	"fmt"
	"log"
	"net"

	pb "go-server-box/gen/proto"

	"google.golang.org/grpc"
)

type ApiGrpcServer struct {
	pb.UnimplementedAthleteuideServer
}

func (s *ApiGrpcServer) CreateAthlete(ctx context.Context, req *pb.AthleteRequest) (*pb.AthleteResponse, error) {
	msg := pb.AthleteResponse{Student: "hola"}
	fmt.Println(req)
	return &msg, nil
}

func main() {
	listen, err := net.Listen("tcp", "localhost:3002")
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
