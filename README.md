# Aplicación de Chat

Este repositorio contiene una aplicación de chat siguiendo una arquitectura cliente-servidor. 

Está compuesta por:

- **server**: Implementación del servidor de chat.
- **client**: Aplicación cliente que se conecta al servidor.
- **protocol**: Definición de mensajes y formatos usados en la comunicación.
- **docs**: Documentación adicional y especificaciones (se va a cambiar por el reporte).
- **.github**: Configuraciones de CI

## Descripción

La aplicación permite a múltiples usuarios intercambiar mensajes en tiempo real usando sockets TPC/IP.
El servidor se ejecuta localmente o en un entorno de producción y expone un punto de conexión al cliente.

**URL de conexión desplegado**  

host: chat.puapc.net

puerto: 1234

## Requisitos

- Rust 1.93.0 o superior
- Cargo
- Conexión de red para comunicación cliente-servidor
- Docker

## Instalación

1. Clonar el repositorio:
   ```bash
   git clone https://github.com/ivannpy/myp_proyecto_1
   cd myp_proyecto_1
   ```
2. Compilar los binarios:
   ```bash
   cargo build --release
   ```

## Uso

### Levantar el servidor

1. Ejecuta el binario:
   ```bash
   cd server
   cargo run --release
   ```
   El servidor escuchará por defecto en el puerto `1234`.

### Ejecución del cliente

1. Ejecuta el binario especificando la dirección del servidor:
   ```bash
   cd client
   cargo run --release
   ```
2. Ingresa el host, el puerto,tu nombre de usuario y comienza a enviar mensajes.

