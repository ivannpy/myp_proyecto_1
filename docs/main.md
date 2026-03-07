# Modelado de la aplicación chat

Se va a implementar una aplicación de chat siguiendo una arquitectura Cliente-Servidor, que se comunican vía enchufes y que implementan el protocolo especificado.

Se identifican 3 entidades principales:

1. Cliente
2. Servidor
3. Protocolo

## Servidor

__Atributos__

- __Escucha__: un __escucha de conexiones TCP__ para escuchar nuevos intentos de conexiones.
- __Estado__: Una estructura de datos que almacene el estado actual del servidor, por ejemplo, qué usuarios están en línea y qué salas han sido creadas.

__Comportamiento__

- El servidor debe saber __crearse__ en un puerto dado.
- El servidor debe saber __ejecutarse__ o iniciarse, lo que significa que debe empezar a escuchar nuevas conexiones entrantes.
- El servidor debe saber __manejar conexiones__ entrantes en nuevos hilos de ejecución.
- El servidor debe saber __recibir mensajes__ desde el cliente.
- El servidor debe saber __escribir mensajes__ hacia el cliente.


## Protocolo

El protocolo no tiene comportamiento, pero es compartido entre el cliente y el servidor, por lo que debe ser una entidad aparte.

El protocolo modela los __mensajes__ que se envían cliente y servidor.

Identificamos dos tipos de mensajes: los que recibe el cliente y los que recibe el servidor.

__Mensajes que recibe el servidor__

- Identify: parametrizado por el nombre del usuario que se está identificando


__Mensajes que recibe el cliente__

- Response: indica la operación realizada, el resultado de realizarla e información extra. Hay varios tipos de resultados a las "responses", por ejemplo, Success y UserAlreadyExists.


























