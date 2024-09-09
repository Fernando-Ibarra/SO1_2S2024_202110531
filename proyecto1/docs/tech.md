# Proyecto 1


### Bash Scripting
```bash
#!/bin/bash
# Este es el shebang que indica que el script debe ser ejecutado usando el intérprete de Bash.

dockerImages=('DockerImage1' 'DockerImage2' 'DockerImage3' 'DockerImage4')
# Se define un array llamado dockerImages que contiene los nombres de las imágenes Docker disponibles.

while true; do
    # Inicia un bucle infinito.
    for i in {1..10}; do
        # Inicia un bucle que se ejecuta 10 veces.
        
        # Selecciona una imagen aleatoria del array dockerImages.
        image=${dockerImages[$RANDOM % ${#dockerImages[@]}]}
        echo "The image selected is: $image"
        
        # Obtiene el directorio actual y lo concatena con el nombre de la imagen seleccionada.
        path=$(pwd)/$image
        
        # Genera un nombre aleatorio para el contenedor usando /dev/urandom.
        name=$(cat /dev/urandom | tr -dc 'a-z0-9' | fold -w 10 | head -n 1)
        echo "The name of the container is: $name"
        
        # Construye la imagen Docker con el nombre generado.
        sudo docker build -t $name $path
        
        # Verifica si la imagen fue creada exitosamente.
        if [ $? -ne 0 ]; then
            echo "Error creating the image"
            exit 1
            # Si hubo un error al crear la imagen, imprime un mensaje de error y sale del script.
        fi
        
        # Ejecuta el contenedor en segundo plano con el nombre generado.
        sudo docker run -d --name $name $name
    done
    sleep 30
    # Espera 30 segundos antes de iniciar el siguiente ciclo del bucle infinito.
done
```


### Modulo del Kernel
```c
/**
 * systeminfo_show - Muestra información del sistema y procesos Docker.
 * @m: Puntero a la estructura seq_file para la salida.
 * @v: Puntero a datos privados (no utilizado).
 *
 * Esta función recopila y muestra información del sistema, incluyendo la memoria total,
 * memoria libre, memoria en uso y una lista de procesos Docker en ejecución. Para cada
 * proceso Docker, se muestra el PID, nombre, uso de CPU, línea de comandos, ID del contenedor,
 * RSS, VSZ y uso de memoria en porcentaje.
 *
 * Retorna: 0 en caso de éxito.
 */
static int systeminfo_show(struct seq_file *m, void *v) {
    struct sysinfo i;
    struct task_struct *task;
    char *string1 = "containerd-shim";
    int first_process = 1;
    int first_docker = 1;

    si_meminfo(&i);
    seq_printf(m, "{\n");
    seq_printf(m, "\t\"total_ram\": %lu,\n", i.totalram * 4);
    seq_printf(m, "\t\"free_ram\": %lu,\n", i.freeram * 4);
    seq_printf(m, "\t\"ram_in_use\": %lu,\n", (i.totalram - i.freeram) * 4);
    seq_printf(m, "\t\"processes\": [\n");

    for_each_process(task) {
        char *string2 = task->comm;
        if (strstr(string1, string2) != NULL) {
            if (first_docker == 1) {
                first_docker = 0;
                continue;
            }

            if (!first_process) {
                seq_printf(m, ",\n");
            }
            first_process = 0;

            seq_printf(m, "\t\t{\n");
            seq_printf(m, "\t\t\t\"pid\": %d,\n", task->pid);
            seq_printf(m, "\t\t\t\"name\": \"%s\",\n", task->comm);
            unsigned long total_jiffies = jiffies;
            unsigned long total_time = task->utime + task->stime;
            unsigned long cpu_usage = ((total_time * 10000) / total_jiffies);
            seq_printf(m, "\t\t\t\"cpu_usage\": %lu,\n", cpu_usage);

            struct mm_struct *mm = task->mm;
            if (mm) {
                unsigned long arg_start = mm->arg_start;
                unsigned long arg_end = mm->arg_end;
                unsigned long len = arg_end - arg_start;
                char *command_line = kmalloc(len + 1, GFP_KERNEL);
                if (command_line) {
                    memset(command_line, 0, len + 1);
                    if (access_process_vm(task, arg_start, command_line, len, 0) > 0) {
                        for (unsigned long i = 0; i < len; i++) {
                            if (command_line[i] == '\0') {
                                command_line[i] = ' ';
                            }
                        }
                        seq_printf(m, "\t\t\t\"command_line\": \"%s\",\n", command_line);
                        char *id_ptr = strstr(command_line, "-id ");
                        if (id_ptr) {
                            id_ptr += 4;
                            char *id_end = strpbrk(id_ptr, " \0");
                            if (id_end) {
                                *id_end = '\0';
                            }
                            seq_printf(m, "\t\t\t\"id_container\": \"%s\",\n", id_ptr);
                        }
                    }
                    kfree(command_line);
                } else {
                    seq_printf(m, "\"command_line\": \"<memory allocation failed>\",\n");
                }

                unsigned long rss = get_mm_rss(mm) * PAGE_SIZE;
                seq_printf(m, "\t\t\t\"rss\": %lu,\n", rss / 1024);
                seq_printf(m, "\t\t\t\"vsz\": %lu,\n", mm->total_vm * PAGE_SIZE / 1024);
                unsigned long total_memory = i.totalram * i.mem_unit;
                unsigned long mem_usage = ((100000 * rss) / total_memory) / 100;
                char mem_usage_str[10];
                sprintf(mem_usage_str, "%lu.%lu", mem_usage / 100, mem_usage % 100);
                seq_printf(m, "\t\t\t\"mem_usage\": %s\n", mem_usage_str);
            }
            seq_printf(m, "\t\t}");
        }
    }

    seq_printf(m, "\n\t]\n");
    seq_printf(m, "}\n");

    return 0;
}
```


### Rust 
Se datalla la descripcion de cada una de las funciones en el archivo rust:
1. **main**:
- Función principal del programa.
- Construye y levanta el contenedor de logs.
- Controla el bucle principal que analiza y elimina contenedores Docker periódicamente.
- Maneja la señal Ctrl+C para salir del bucle.
- Elimina el trabajo cron y genera gráficos al finalizar.

2. **build_container**:
- Construye y levanta los contenedores Docker usando docker-compose.
- Retorna el ID del contenedor de logs.

3. **read_containers**:
- Analiza el estado de los contenedores Docker y elimina los que no son necesarios.

4. **delete_cron_job**:
- Elimina el trabajo cron job del archivo bash.

5. **make_graphs_process**:
- Genera gráficos basados en los datos recopilados.


### Python Server
1. **read_root**:
- Endpoint raíz (GET /).
- Devuelve un mensaje de bienvenida.

2. **read_module**:
- Endpoint para leer datos del módulo CPU (POST /read-module).
- Recibe datos del CPU en el cuerpo de la solicitud y los guarda en un archivo JSON.
- Retorna una respuesta JSON con un mensaje de éxito.

3. **make_graphs**:
- Endpoint para generar gráficos de uso de RAM (POST /make_graphs).
- Lee los datos del archivo JSON y genera gráficos de uso de RAM.
- Guarda los gráficos en un archivo PNG y retorna una respuesta JSON con 
un mensaje de éxito.

4. **make_process_graphs**:
- Endpoint para generar gráficos de procesos usando matplotlib (POST /make_process_graphs).