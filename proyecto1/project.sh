#!/bin/bash

dockerImages=('DockerImage1' 'DockerImage2' 'DockerImage3' 'DockerImage4')

while true; do
    for i in {1..10}; do
      # Select a random image
      image=${dockerImages[$RANDOM % ${#dockerImages[@]}]}
      echo "The image selected is: $image"
      path=$(pwd)/$image
      # Generate a random name
      name=$(cat /dev/urandom | tr -dc 'a-z0-9' | fold -w 10 | head -n 1)
      echo "The name of the container is: $name"
      # Build the image
      sudo docker build -t $name $path
      # Check if the image was created
      if [ $? -ne 0 ]; then
        echo "Error creating the image"
        exit 1
      fi
      # Run the container
      sudo docker run -d --name $name $name
    done
    sleep 30
done