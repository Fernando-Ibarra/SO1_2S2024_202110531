package main

import (
	"fmt"
	"net/http"

	"github.com/gin-gonic/gin"
)

type athlete struct {
	Student    string `json:"student"`
	Age        int64  `json:"age"`
	Faculty    string `json:"faculty"`
	Discipline int64  `json:"discipline"`
}

func postAthlete(c *gin.Context) {
	var newAthlete athlete
	if err := c.BindJSON(&newAthlete); err != nil {
		return
	}
	c.IndentedJSON(http.StatusCreated, newAthlete)

	fmt.Println("Student: ", newAthlete.Student)
	fmt.Println("Age: ", newAthlete.Age)
	fmt.Println("Faculty: ", newAthlete.Faculty)
	fmt.Println("Discipline: ", newAthlete.Discipline)
}

func main() {
	router := gin.Default()
	router.POST("/", postAthlete)
	router.Run("0.0.0.0:8080")
}
