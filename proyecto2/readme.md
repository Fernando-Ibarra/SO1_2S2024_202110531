# Proyecto 2

## Ingress

#### Instalación de Ingress

```bash
kubectl create ns ingress-nginx
kubectl apply -f ingress.yml
kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/main/deploy/static/provider/cloud/deploy.yaml
```

#### Configuración de Ingress

Se configura para que el ingress redireccione a los servicios de Rust y Go, según la ruta que se le indique. En este caso, los paths son `/agronomia` y `/ingenieria` respectivamente.

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: app-ingress
  annotations:
    nginx.ingress.kubernetes.io/rewrite-target: /
spec:
  ingressClassName: nginx 
  rules:
  - http:
      paths:
      - path: /agronomia
        pathType: Prefix
        backend:
          service:
            name: rust-service
            port:
              number: 80
      - path: /ingenieria
        pathType: Prefix
        backend:
          service:
            name: go-service
            port:
              number: 80

```

## Rust - Client

### Servicio de Rust

Este archivo contiene la implementación principal del servidor Rust utilizando Rocket y gRPC para manejar solicitudes de atletas.

#### Dependencias

El archivo utiliza las siguientes dependencias:

- `rocket`: Para manejar las solicitudes HTTP.
- `serde` y `serde_json`: Para la serialización y deserialización de datos JSON.
- `tonic`: Para la comunicación gRPC.
- `tokio`: Para manejar tareas asíncronas.

#### Estructuras de Datos

- `HttpAthleteRequest`

Esta estructura representa la solicitud HTTP que se recibe para crear un atleta.

```rust
#[derive(Deserialize, Serialize)]
struct HttpAthleteRequest {
    student: String,
    age: i64,
    faculty: String,
    discipline: i64,
}
```

#### Módulos

Este módulo incluye las definiciones generadas por tonic a partir de los archivos proto.

```rust
pub mod main {
    tonic::include_proto!("main");
}
```

#### Rutas

Esta ruta maneja las solicitudes POST para crear un atleta. Dependiendo de la disciplina del atleta, se envía una solicitud gRPC al servidor correspondiente.

```rust
#[post("/", format = "json", data = "<athlete>")]
async fn create_athlete(athlete: Json<HttpAthleteRequest>) -> content::RawHtml<String> {
    let athlete_data = athlete.into_inner();
    let grpc_response = match athlete_data.discipline {
        1 => task::spawn(async move { grpc_swim_server(athlete_data).await }).await.unwrap(),
        2 => task::spawn(async move { grpc_run_server(athlete_data).await }).await.unwrap(),
        3 => task::spawn(async move { grpc_box_server(athlete_data).await }).await.unwrap(),
        _ => Err("Invalid discipline".into()),
    };

    match grpc_response {
        Ok(response) => content::RawHtml(format!("gRPC response: {}", response)),
        Err(e) => content::RawHtml(format!("Error: {}", e)),
    }
}
```

#### gRPC

Esta función envía una solicitud gRPC al servidor de natación.

```rust
async fn grpc_swim_server(athlete: HttpAthleteRequest) -> Result<String, String> {
    let mut client = match AthleteuideClient::connect("http://go-service-swim:3001").await {
        Ok(client) => client,
        Err(e) => return Err(format!("Failed to connect to gRPC server Swim: {}", e)),
    };

    let request = tonic::Request::new(AthleteRequest {
        student: athlete.student,
        age: athlete.age,
        faculty: athlete.faculty,
        discipline: athlete.discipline,
    });

    match client.create_athlete(request).await {
        Ok(response) => Ok(response.into_inner().student),
        Err(e) => Err(format!("Error sending request to gRPC server 1: {}", e)),
    }
}
```

#### Server

La función rocket monta la ruta create_athlete en el servidor Rocket.

```rust
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![create_athlete])
}
```

#### Protobuf

El archivo build.rs se encarga de compilar los archivos proto necesarios para gRPC.

```bash
fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::compile_protos("proto/athlete.proto")?;
    Ok(())
}
```

### K8s - Rust

```yml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: rust-server
spec:
  replicas: 1
  selector:
    matchLabels:
      app: rust-server
  template:
    metadata:
      labels:
        app: rust-server
    spec:
      containers:
      - name: rust-container
        image: feribarra27/rust-server:latest
        imagePullPolicy: Always
        ports:
        - containerPort: 8080
        resources:
          requests:
            cpu: "50m"
            memory: "16Mi"
          limits:
            cpu: "100m"
            memory: "32Mi"
---
apiVersion: v1
kind: Service
metadata:
  name: rust-service
spec:
  selector:
    app: rust-server
  ports:
    - protocol: TCP
      port: 80
      targetPort: 8080
  type: LoadBalancer

```

## Go - Client

### Dependencias (Go Server)

El archivo utiliza las siguientes dependencias:

- `github.com/gin-gonic/gin`: Para manejar las solicitudes HTTP.
- `github.com/Shopify/sarama`: Para interactuar con Kafka.
- `github.com/joho/godotenv`: Para cargar variables de entorno desde un archivo `.env`.

### Estructuras de Datos (Go Server)

#### `Athlete`

Esta estructura representa un atleta con sus atributos.

```go
type Athlete struct {
    Student   string `json:"student"`
    Age       int    `json:"age"`
    Faculty   string `json:"faculty"`
    Discipline int   `json:"discipline"`
}
```

### Funciones (Go Server)

#### Servidor

La función principal que inicializa el servidor y configura las rutas.

```go
func main() {
    err := godotenv.Load()
    if err != nil {
        log.Fatal("Error loading .env file")
    }

    router := gin.Default()
    router.POST("/athlete", createAthlete)
    router.Run(":8080")
}
```

#### createAthlete

Esta función maneja las solicitudes POST para crear un atleta y envía los datos a Kafka.

```go
func createAthlete(c *gin.Context) {
    var athlete Athlete
    if err := c.ShouldBindJSON(&athlete); err != nil {
        c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
        return
    }

    producer, err := sarama.NewSyncProducer([]string{os.Getenv("KAFKA_BROKER")}, nil)
    if err != nil {
        log.Fatal("Error creating Kafka producer:", err)
    }
    defer producer.Close()

    message, err := json.Marshal(athlete)
    if err != nil {
        log.Fatal("Error marshalling athlete data:", err)
    }

    msg := &sarama.ProducerMessage{
        Topic: os.Getenv("KAFKA_TOPIC"),
        Value: sarama.StringEncoder(message),
    }

    _, _, err = producer.SendMessage(msg)
    if err != nil {
        log.Fatal("Error sending message to Kafka:", err)
    }

    c.JSON(http.StatusOK, gin.H{"status": "athlete created"})
}
```

##### Variables de Entorno

El archivo utiliza las siguientes variables de entorno, que deben estar definidas en un archivo .env:

- KAFKA_BROKER: La dirección del broker de Kafka.
- KAFKA_TOPIC: El tema de Kafka al que se enviarán los mensajes.

#### POST

Esta ruta recibe una solicitud POST con los datos de un atleta en formato JSON y los envía a Kafka.

- Solicitud

```json
{
    "student": "John Doe",
    "age": 21,
    "faculty": "Engineering",
    "discipline": 1
}
```

- Respuesta

```json
{
    "status": "athlete created"
}
```

### K8s - Go

```yml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: go-server
spec:
  replicas: 1
  selector:
    matchLabels:
      app: go-server
  template:
    metadata:
      labels:
        app: go-server
    spec:
      containers:
      - name: go-container
        image: feribarra27/go-server:latest
        imagePullPolicy: Always
        ports:
        - containerPort: 8080
        resources:
          requests:
            cpu: "100m"
            memory: "128Mi"
          limits:
            cpu: "250m"
            memory: "256Mi"
---
apiVersion: v1
kind: Service
metadata:
  name: go-service
spec:
  selector:
    app: go-server
  ports:
    - protocol: TCP
      port: 80
      targetPort: 8080
  type: LoadBalancer
```

## Go - Server - Publisher

### Dependencias (Go Server Publisher)

El archivo utiliza las siguientes dependencias:

- `google.golang.org/grpc`: Para manejar las solicitudes gRPC.
- `github.com/joho/godotenv`: Para cargar variables de entorno desde un archivo `.env`.
- `github.com/sirupsen/logrus`: Para el registro de logs.

### Estructuras de Datos (Go Server Publisher)

#### `Athlete`

Esta estructura implementa el servidor gRPC para manejar las solicitudes de creación de atletas.

```go
type AthleteServer struct {
    main.UnimplementedAthleteuideServer
}
```

### Funciones (Go Server Publisher)

#### Servidor (Go Server Publisher)

La función principal que inicializa el servidor gRPC y carga las variables de entorno.

```go
func main() {
    err := godotenv.Load()
    if err != nil {
        log.Fatal("Error loading .env file")
    }

    lis, err := net.Listen("tcp", fmt.Sprintf(":%s", os.Getenv("PORT")))
    if err != nil {
        log.Fatalf("failed to listen: %v", err)
    }

    s := grpc.NewServer()
    main.RegisterAthleteuideServer(s, &AthleteServer{})

    log.Printf("server listening at %v", lis.Addr())
    if err := s.Serve(lis); err != nil {
        log.Fatalf("failed to serve: %v", err)
    }
}
```

#### CreateAthlete

Esta función maneja las solicitudes gRPC para crear un atleta y registra la información recibida.

```go
func (s *AthleteServer) CreateAthlete(ctx context.Context, in *main.AthleteRequest) (*main.AthleteReply, error) {
    log.Printf("Received: %v", in.GetStudent())
    return &main.AthleteReply{Student: "Athlete created: " + in.GetStudent()}, nil
}
```

##### Variables de Entorno 
El archivo utiliza las siguientes variables de entorno, que deben estar definidas en un archivo .env:

- PORT: El puerto en el que el servidor gRPC escuchará las solicitudes.

#### POST (Go Server Publisher)

Este método maneja las solicitudes gRPC para crear un atleta.

- Solicitud

```json
{
    "student": "John Doe",
    "age": 21,
    "faculty": "Engineering",
    "discipline": 1
}
```

- Respuesta

```json
{
    "student": "Athlete created: John Doe"
}
```

### K8s - Go - Publisher

```yml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: go-server-swim
spec:
  replicas: 1
  selector:
    matchLabels:
      app: go-server-swim
  template:
    metadata:
      labels:
        app: go-server-swim
    spec:
      containers:
      - name: go-container-swim
        image: feribarra27/go-server-swim:latest
        imagePullPolicy: Always
        ports:
        - containerPort: 3001
        resources:
          requests:
            cpu: "100m"
            memory: "128Mi"
          limits:
            cpu: "250m"
            memory: "256Mi"
---
apiVersion: v1
kind: Service
metadata:
  name: go-service-swim
spec:
  selector:
    app: go-server-swim
  ports:
    - protocol: TCP
      port: 3001
      targetPort: 3001
  type: ClusterIP
```

## Kafka

Se utiliza Kafka para enviar mensajes entre los servicios de Go y Rust. Se crean dos tópicos, uno para cada disciplina.

```bash
kubectl create namespace kafka
kubectl create -f 'https://strimzi.io/install/latest?namespace=kafka' -n kafka
kubectl apply -f strimzi.yml -n kafka
kubectl apply -f kafka.yml
```

### Strimzi

```yml
apiVersion: kafka.strimzi.io/v1beta2
kind: Kafka
metadata:
  name: my-cluster
spec:
  kafka:
    version: 3.8.0
    replicas: 1
    listeners:
      - name: plain
        port: 9092
        type: internal
        tls: false
      - name: tls
        port: 9093
        type: internal
        tls: true
    config:
      offsets.topic.replication.factor: 1
      transaction.state.log.replication.factor: 1
      transaction.state.log.min.isr: 1
      default.replication.factor: 1
      min.insync.replicas: 1
      inter.broker.protocol.version: "3.8"
    storage:
      type: jbod
      volumes:
      - id: 0
        type: persistent-claim
        size: 3Gi
        deleteClaim: false
  zookeeper:
    replicas: 1
    storage:
      type: persistent-claim
      size: 3Gi
      deleteClaim: false
  entityOperator:
    topicOperator: {}
    userOperator: {}
```

### Kafka - Tópicos

```yml
apiVersion: kafka.strimzi.io/v1beta2
kind: Kafka
metadata:
  name: my-cluster
spec:
  kafka:
    version: 3.7.0
    replicas: 1
    listeners:
      - name: plain
        port: 9092
        type: internal
        tls: false
      - name: tls
        port: 9093
        type: internal
        tls: true
    config:
      offsets.topic.replication.factor: 1
      transaction.state.log.replication.factor: 1
      transaction.state.log.min.isr: 1
      default.replication.factor: 1
      min.insync.replicas: 1
      inter.broker.protocol.version: "3.7"
    resources:
      requests:
        memory: "256Mi"
        cpu: "150m"
      limits:
        memory: "1Gi"
        cpu: "200m"
    storage:
      type: jbod
      volumes:
        - id: 0
          type: persistent-claim
          size: 3Gi
          deleteClaim: false
  zookeeper:
    replicas: 1
    resources:
      requests:
        memory: "128Mi"    # Reduce los recursos solicitados para Zookeeper
        cpu: "100m"
      limits:
        memory: "256Mi"    # Limita los recursos máximos para Zookeeper
        cpu: "200m"
    storage:
      type: persistent-claim
      size: 3Gi
      deleteClaim: false
  entityOperator:
    topicOperator:
      resources:
        requests:
          memory: "96Mi"
          cpu: "100m"
        limits:
          memory: "128Mi"
          cpu: "200m"
    userOperator:
      resources:
        requests:
          memory: "96Mi"
          cpu: "100m"
        limits:
          memory: "128Mi"
          cpu: "200m"
```

## Go - Consumer - Winner

### Dependencias (Go Consumer Winner)

El archivo utiliza las siguientes dependencias:

- `github.com/Shopify/sarama`: Para interactuar con Kafka.
- `github.com/go-redis/redis/v8`: Para interactuar con Redis.
- `github.com/joho/godotenv`: Para cargar variables de entorno desde un archivo `.env`.
- `github.com/sirupsen/logrus`: Para el registro de logs.
- `context`: Para manejar el contexto de las operaciones.

### Funciones (Go Consumer Winner)

#### Servidor (Go Consumer Winner)

La función principal que inicializa el consumidor de Kafka y configura la conexión a Redis.

```go
func main() {
    err := godotenv.Load()
    if err != nil {
        log.Fatal("Error loading .env file")
    }

    redisClient := redis.NewClient(&redis.Options{
        Addr:     os.Getenv("REDIS_ADDR"),
        Password: os.Getenv("REDIS_PASSWORD"),
        DB:       0,
    })

    consumer, err := sarama.NewConsumer([]string{os.Getenv("KAFKA_BROKER")}, nil)
    if err != nil {
        log.Fatal("Error creating Kafka consumer:", err)
    }
    defer consumer.Close()

    partitionConsumer, err := consumer.ConsumePartition(os.Getenv("KAFKA_TOPIC"), 0, sarama.OffsetNewest)
    if err != nil {
        log.Fatal("Error creating Kafka partition consumer:", err)
    }
    defer partitionConsumer.Close()

    for message := range partitionConsumer.Messages() {
        log.Printf("Received message: %s", string(message.Value))
        processMessage(redisClient, message.Value)
    }
}
```

#### processMessage

Esta función procesa los mensajes recibidos de Kafka y los guarda en Redis.

```go
func processMessage(redisClient *redis.Client, message []byte) {
    var winner Athlete
    err := json.Unmarshal(message, &winner)
    if err != nil {
        log.Printf("Error unmarshalling message: %v", err)
        return
    }

    ctx := context.Background()
    err = redisClient.Set(ctx, winner.Student, message, 0).Err()
    if err != nil {
        log.Printf("Error saving message to Redis: %v", err)
    } else {
        log.Printf("Saved winner to Redis: %s", winner.Student)
    }
}
```

### Estructuras de Datos (Go Consumer Winner)

- `Athlete`

Esta estructura representa un atleta con sus atributos.

```go
type Athlete struct {
    Student   string `json:"student"`
    Age       int    `json:"age"`
    Faculty   string `json:"faculty"`
    Discipline int   `json:"discipline"`
}
```

#### Variables de Entorno 
El archivo utiliza las siguientes variables de entorno, que deben estar definidas en un archivo .env:

- **KAFKA_BROKER**: La dirección del broker de Kafka.
- **KAFKA_TOPIC**: El tema de Kafka del que se consumirán los mensajes.
- **REDIS_ADDR**: La dirección del servidor Redis.
- **REDIS_PASSWORD**: La contraseña para el servidor Redis.

### Kafka - Consumer

El consumidor escucha mensajes en el tema de Kafka especificado y procesa los datos de los ganadores.

Ejemplo de Mensaje

```json
{
    "student": "John Doe",
    "age": 21,
    "faculty": "Engineering",
    "discipline": 1
}
```

### Redis

##### Almacenamiento

Los datos de los ganadores se almacenan en Redis con la clave siendo el nombre del estudiante.

- Valor: `{"student":"John Doe","age":21,"faculty":"Engineering","discipline":1}`


## Redis

```bash
kubectl apply -f redis.yml
kubectl create secret generic redis-secret --from-literal=url=redis-service:6379
kubectl create secret generic redis-password-secret --from-literal=password=M1R3D1SP4SSW0RD
```